use crate::{
    atomic_file, config,
    config::Config,
    config_validate, paths, window,
};
use crate::config::Profile;
use crate::config_write::{LastWrite, sanitize};
use std::sync::{Arc, RwLock};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn draft_profile(app: AppHandle, next: Profile) {
    let config = app.try_state::<Arc<RwLock<Config>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone()));
    let Some(config) = config else { return };
    if let Err(error) = crate::config_write::apply_in_memory(&app, config, next) {
        eprintln!("draft_profile: {error}");
    }
}

#[tauri::command]
pub fn activate_profile(app: AppHandle, profile_name: String) {
    let profile_path = paths::profile_path(&profile_name);
    if !profile_path.exists() {
        eprintln!("activate_profile: profile '{}' not found", profile_name);
        return;
    }
    let source = match std::fs::read_to_string(&profile_path) {
        Ok(s) => s,
        Err(e) => { eprintln!("activate_profile read error: {e}"); return; }
    };
    let profile: Profile = match toml::from_str(&source) {
        Ok(p) => p,
        Err(e) => { eprintln!("activate_profile parse error: {e}"); return; }
    };
    if let Err(e) = config_validate::check_profile(&profile) {
        eprintln!("activate_profile validation error: {e}");
        return;
    }
    let mut next_config = match app.try_state::<Arc<RwLock<Config>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone())) {
        Some(c) => c,
        None => return,
    };
    next_config.active_profile = profile_name.clone();
    let next_config = sanitize(next_config);
    let config_contents = config::serialize(&next_config);
    if let Some(last) = app.try_state::<LastWrite>() {
        if let Ok(mut guard) = last.0.lock() { *guard = config_contents.clone(); }
    }
    if let Err(e) = atomic_file::write(&paths::config_path(), &config_contents) {
        eprintln!("activate_profile config write error: {e}");
        return;
    }
    if let Some(state) = app.try_state::<Arc<RwLock<Config>>>() {
        if let Ok(mut guard) = state.write() { *guard = next_config; }
    }
    if let Some(state) = app.try_state::<Arc<RwLock<Profile>>>() {
        if let Ok(mut guard) = state.write() { *guard = profile.clone(); }
    }
    crate::audio::sync(&app, &profile);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || window::reconcile(&handle, &profile));
}
