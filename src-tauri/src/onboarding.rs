use crate::{
    atomic_file, config, config::Config, config::Profile,
    config_validate, paths,
};
use serde::Serialize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use tauri::{AppHandle, Manager};

#[derive(Clone, Serialize)]
pub struct WorkArea {
    pub identity: String,
    pub width: i32,
    pub height: i32,
}

#[derive(serde::Deserialize)]
pub struct OnboardingChoice {
    pub theme: String,
    pub preset: String,
    pub autostart: bool,
}

pub struct NeedsOnboarding(pub Arc<AtomicBool>);

pub fn needs_onboarding() -> bool {
    !paths::config_path().exists()
}

pub fn detect_work_areas() -> Vec<WorkArea> {
    let areas = gtk_work_areas();
    if !areas.is_empty() {
        return areas;
    }
    vec![WorkArea { identity: "0".into(), width: 1920, height: 1080 }]
}

fn gtk_work_areas() -> Vec<WorkArea> {
    #[cfg(target_os = "linux")]
    {
        #[allow(unused_imports)]
        use gtk::prelude::*;
        let _ = gtk::init();
        let display = match gtk::gdk::Display::default() {
            Some(d) => d,
            None => return vec![],
        };
        let mut areas = Vec::new();
        for i in 0..display.n_monitors() {
            if let Some(monitor) = display.monitor(i) {
                let rect = monitor.geometry();
                areas.push(WorkArea {
                    identity: format!("monitor-{i}"),
                    width: rect.width(),
                    height: rect.height(),
                });
            }
        }
        return areas;
    }
    #[cfg(not(target_os = "linux"))]
    vec![]
}

#[tauri::command]
pub fn needs_onboarding_cmd() -> bool {
    needs_onboarding()
}

#[tauri::command]
pub fn onboarding_work_areas() -> Vec<WorkArea> {
    detect_work_areas()
}

#[tauri::command]
pub fn apply_onboarding(app: AppHandle, choice: OnboardingChoice) -> Result<(), String> {
    let area = detect_work_areas().first().cloned()
        .unwrap_or(WorkArea { identity: "0".into(), width: 1920, height: 1080 });
    let disks = sysinfo::Disks::new_with_refreshed_list();
    let detected_disks = crate::disk::discover(&disks)
        .into_iter().map(|disk| disk.id).collect::<Vec<_>>();
    let (config, profile) = crate::onboarding_presets::snapshot_for(
        &choice.preset, &choice.theme, &area, &detected_disks,
    );
    config_validate::check(&config).map_err(|e| e.to_string())?;
    config_validate::check_profile(&profile).map_err(|e| e.to_string())?;
    atomic_file::write(&paths::config_path(), &config::serialize(&config))
        .map_err(|e| format!("write config: {e}"))?;
    let profile_path = paths::profile_path(&config.active_profile);
    let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
    atomic_file::write(&profile_path, &config::serialize_profile(&profile))
        .map_err(|e| format!("write profile: {e}"))?;
    crate::autostart::set_autostart(app.clone(), choice.autostart)?;
    if let Some(state) = app.try_state::<Arc<RwLock<Config>>>() {
        if let Ok(mut guard) = state.write() { *guard = config; }
    }
    if let Some(state) = app.try_state::<Arc<RwLock<Profile>>>() {
        if let Ok(mut guard) = state.write() { *guard = profile.clone(); }
    }
    if let Some(state) = app.try_state::<NeedsOnboarding>() {
        state.0.store(false, Ordering::Relaxed);
    }
    let h = app.clone();
    let _ = app.run_on_main_thread(move || crate::window::reconcile(&h, &profile));
    // The launch that shows onboarding skips background work because no config
    // existed yet. Now one does, so start it rather than waiting for a restart.
    if let (Some(config_state), Some(profile_state)) = (
        app.try_state::<Arc<RwLock<Config>>>().map(|s| s.inner().clone()),
        app.try_state::<Arc<RwLock<Profile>>>().map(|s| s.inner().clone()),
    ) {
        crate::background::start(&app, config_state, profile_state);
    }
    Ok(())
}
