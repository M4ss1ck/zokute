use crate::{config, config::Profile, config_validate, paths, settings_session, window};
use serde::Serialize;
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Manager};

pub enum ExternalChange {
    Config,
    Profile,
}

pub struct ExternalConfig(pub Mutex<Option<ExternalChange>>);

#[derive(Serialize)]
pub struct AcceptedExternal {
    config: config::Config,
    profile: Profile,
}

#[tauri::command]
pub fn accept_external_config(app: AppHandle) -> Result<AcceptedExternal, String> {
    let holder = app.try_state::<ExternalConfig>().ok_or("no external state")?;
    let mut pending = holder.0.lock().map_err(|error| error.to_string())?;
    if pending.is_none() { return Err("no external change".into()); }
    let config_path = paths::config_path();
    for _ in 0..3 {
        let config_source = std::fs::read_to_string(&config_path).map_err(|error| error.to_string())?;
        let config = config::parse(&config_source).map_err(|error| error.to_string())?;
        config_validate::check(&config).map_err(|error| error.to_string())?;
        let profile_path = paths::profile_path(&config.active_profile);
        let profile_source = std::fs::read_to_string(&profile_path).map_err(|error| error.to_string())?;
        let profile = toml::from_str::<Profile>(&profile_source).map_err(|error| error.to_string())?;
        config_validate::check_profile(&profile).map_err(|error| error.to_string())?;
        let stable_config = std::fs::read_to_string(&config_path).map_err(|error| error.to_string())?;
        let stable_profile = std::fs::read_to_string(&profile_path).map_err(|error| error.to_string())?;
        if config_source != stable_config || profile_source != stable_profile { continue; }
        let config_state = app.try_state::<Arc<RwLock<config::Config>>>().ok_or("no config")?;
        let profile_state = app.try_state::<Arc<RwLock<Profile>>>().ok_or("no profile")?;
        let mut config_guard = config_state.write().map_err(|error| error.to_string())?;
        let mut profile_guard = profile_state.write().map_err(|error| error.to_string())?;
        *config_guard = config.clone();
        *profile_guard = profile.clone();
        drop(profile_guard);
        drop(config_guard);
        crate::audio::sync(&app, &profile);
        let handle = app.clone();
        let next_profile = profile.clone();
        let _ = app.run_on_main_thread(move || window::reconcile(&handle, &next_profile));
        settings_session::snapshot(&app);
        *pending = None;
        return Ok(AcceptedExternal { config, profile });
    }
    Err("settings changed while reloading".into())
}

#[tauri::command]
pub fn dismiss_external_config(app: AppHandle) {
    if let Some(holder) = app.try_state::<ExternalConfig>() {
        if let Ok(mut guard) = holder.0.lock() {
            *guard = None;
        }
    }
}
