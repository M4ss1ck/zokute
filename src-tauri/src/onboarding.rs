use crate::{
    atomic_file, config, config::Config, config::Profile,
    config_validate, paths,
    position_model::{Anchor, Position},
};
use serde::Serialize;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use tauri::{AppHandle, Manager};
use std::collections::BTreeMap;

const SECTION_WIDTH: u32 = 360;

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

pub fn snapshot_for(preset: &str, theme: &str) -> (Config, Profile) {
    let areas = detect_work_areas();
    let first = areas.first().cloned().unwrap_or(WorkArea { identity: "0".into(), width: 1920, height: 1080 });

    let config = Config {
        schema_version: 3,
        active_profile: "default".into(),
        opacity: 0.92,
        text_opacity: 1.0,
        text_color: "#292824".into(),
        graph_color: None,
        icon_color: None,
        show_background: true,
        theme: theme.into(),
        accent_color: None,
        density: "compact".into(),
        font_scale: 1.0,
        sans_font: None,
        mono_font: None,
        byte_format: "binary".into(),
        temperature_unit: "celsius".into(),
        locale: None,
        extra: BTreeMap::new(),
    };

    let profile = match preset {
        "minimal" => Profile {
            profile_schema_version: 1,
            sections: vec![
                mk_section("clock", &first, 0),
                mk_section("date", &first, 1),
            ],
            system_fields: vec![],
            show_cpu_cores: false,
            disks: vec![],
            collect_interval_ms: 1000,
            monitor_catalog: BTreeMap::new(),
            fullscreen: Default::default(), extra: BTreeMap::new(),
        },
        "system_monitor" => {
            let ids = ["clock", "date", "cpu", "memory", "disk", "network"];
            let sections: Vec<_> = ids.iter().enumerate().map(|(i, id)| {
                mk_section(id, &first, i as i32)
            }).collect();
            Profile {
                profile_schema_version: 1,
                sections,
                system_fields: crate::config::DEFAULT_SYSTEM_FIELDS.iter().map(|f| f.to_string()).collect(),
                show_cpu_cores: true,
                disks: vec![],
                collect_interval_ms: 1000,
                monitor_catalog: BTreeMap::new(),
                fullscreen: Default::default(), extra: BTreeMap::new(),
            }
        }
        _ => Profile {
            profile_schema_version: 1,
            sections: vec![],
            system_fields: vec![],
            show_cpu_cores: false,
            disks: vec![],
            collect_interval_ms: 1000,
            monitor_catalog: BTreeMap::new(),
            fullscreen: Default::default(), extra: BTreeMap::new(),
        },
    };

    (config, normalize_onboarding_profile(profile))
}

fn mk_section(id: &str, area: &WorkArea, index: i32) -> crate::config::SectionConfig {
    let y_offset = 24 + index * 120;
    crate::config::SectionConfig {
        id: id.to_string(),
        instance: id.to_string(),
        enabled: true,
        show_header: true,
        position: Some(Position::Anchored {
            monitor_identity: area.identity.clone(),
            anchor: Anchor::TopLeft,
            offset_x: 24,
            offset_y: y_offset,
        }),
        monitor: 0, x: 24, y: y_offset,
        width: SECTION_WIDTH, height: None, scale: 1.0,
        color_mode: None, color_a: None, color_b: None, gradient_direction: None,
        clock_font: None, clock_color: None, clock_seconds: false,
        clock_24h: false, clock_ampm: true, clock_pad: true,
        clock_layout: None, clock_align: None,
        date_weekday: true, date_format: None, date_color: None,
        interactive: false, timezone: None,
        plugin_id: None, plugin_interval: 30, plugin_config: None,
        children: vec![], panel_gap: 0, panel_padding: 0,
        accent_color: None, transparent_surface: None, opacity_override: None,
        border_visible: None, radius_override: None, padding_override: None,
        font_scale: None, chart_colors: None,
        viz_bar_count: None, viz_min_hz: None, viz_max_hz: None,
        viz_gain: None, viz_smoothing: None, viz_decay: None,
        viz_mirror: None, viz_gap: None, viz_rounded_caps: None, viz_fps: None,
        extra: BTreeMap::new(),
    }
}

fn normalize_onboarding_profile(mut profile: Profile) -> Profile {
    let mut labels: Vec<String> = Vec::new();
    for section in &mut profile.sections {
        let base = if section.instance.is_empty() { &section.id } else { &section.instance };
        let mut label = base.clone();
        let mut suffix = 2;
        while labels.contains(&label) { label = format!("{base}-{suffix}"); suffix += 1; }
        section.instance = label.clone();
        labels.push(label);
    }
    profile
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
    let (config, profile) = snapshot_for(&choice.preset, &choice.theme);
    config_validate::check(&config).map_err(|e| e.to_string())?;
    config_validate::check_profile(&profile).map_err(|e| e.to_string())?;
    atomic_file::write(&paths::config_path(), &config::serialize(&config))
        .map_err(|e| format!("write config: {e}"))?;
    let profile_path = paths::profile_path(&config.active_profile);
    let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
    atomic_file::write(&profile_path, &config::serialize_profile(&profile))
        .map_err(|e| format!("write profile: {e}"))?;
    if choice.autostart {
        #[allow(unused_imports)]
        use tauri_plugin_autostart::ManagerExt;
        app.autolaunch().enable().ok();
    }
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
    Ok(())
}
