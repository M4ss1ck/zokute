use crate::{config, config::Profile, paths, window};
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Manager};

pub enum ExternalChange {
    Config(String, config::Config),
    Profile(String, Profile),
}

pub struct ExternalConfig(pub Mutex<Option<ExternalChange>>);

#[tauri::command]
pub fn accept_external_config(app: AppHandle) -> Result<(), String> {
    let candidate = app
        .try_state::<ExternalConfig>()
        .and_then(|m| m.0.lock().ok().and_then(|mut g| g.take()));
    let Some(change) = candidate else { return Ok(()) };
    match change {
        ExternalChange::Config(_, candidate_config) => {
            if let Some(state) = app.try_state::<Arc<RwLock<config::Config>>>() {
                let mut guard = state.write().map_err(|e| e.to_string())?;
                *guard = candidate_config.clone();
                drop(guard);
            }
            if let Some(profile_state) = app.try_state::<Arc<RwLock<Profile>>>() {
                let profile_path = paths::profile_path(&candidate_config.active_profile);
                if let Ok(profile_source) = std::fs::read_to_string(&profile_path) {
                    if let Ok(profile) = toml::from_str::<Profile>(&profile_source) {
                        if let Ok(mut guard) = profile_state.write() {
                            *guard = profile.clone();
                            drop(guard);
                            crate::audio::sync(&app, &profile);
                            let handle = app.clone();
                            let _ = app.run_on_main_thread(move || {
                                window::reconcile(&handle, &profile);
                            });
                        }
                    }
                }
            }
        }
        ExternalChange::Profile(_, candidate_profile) => {
            if let Some(state) = app.try_state::<Arc<RwLock<Profile>>>() {
                let mut guard = state.write().map_err(|e| e.to_string())?;
                *guard = candidate_profile.clone();
                drop(guard);
                crate::audio::sync(&app, &candidate_profile);
                let handle = app.clone();
                let _ = app.run_on_main_thread(move || {
                    window::reconcile(&handle, &candidate_profile);
                });
            }
        }
    }
    Ok(())
}

#[tauri::command]
pub fn dismiss_external_config(app: AppHandle) {
    if let Some(holder) = app.try_state::<ExternalConfig>() {
        if let Ok(mut guard) = holder.0.lock() {
            *guard = None;
        }
    }
}
