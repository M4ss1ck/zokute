use crate::{config, config::Profile, paths, window};
use serde::Serialize;
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Manager};

pub enum ExternalChange {
    Config(String, config::Config),
    Profile(String, Profile),
}

pub struct ExternalConfig(pub Mutex<Option<ExternalChange>>);

#[derive(Serialize)]
pub struct AcceptedExternal {
    config: config::Config,
    profile: Profile,
}

#[tauri::command]
pub fn accept_external_config(app: AppHandle) -> Result<AcceptedExternal, String> {
    let candidate = app
        .try_state::<ExternalConfig>()
        .and_then(|m| m.0.lock().ok().and_then(|mut g| g.take()));
    if let Some(change) = candidate {
        match change {
            ExternalChange::Config(_, candidate_config) => {
                if let Some(state) = app.try_state::<Arc<RwLock<config::Config>>>() {
                    let mut guard = state.write().map_err(|e| e.to_string())?;
                    *guard = candidate_config.clone();
                    drop(guard);
                }
                if let Some(profile_state) = app.try_state::<Arc<RwLock<Profile>>>() {
                    let profile_path = paths::profile_path(&candidate_config.active_profile);
                    let profile_source = std::fs::read_to_string(&profile_path)
                        .map_err(|error| error.to_string())?;
                    let profile = toml::from_str::<Profile>(&profile_source)
                        .map_err(|error| error.to_string())?;
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
    }
    let config = app.try_state::<Arc<RwLock<config::Config>>>()
        .and_then(|state| state.read().ok().map(|guard| guard.clone()))
        .ok_or("no config")?;
    let profile = app.try_state::<Arc<RwLock<Profile>>>()
        .and_then(|state| state.read().ok().map(|guard| guard.clone()))
        .ok_or("no profile")?;
    Ok(AcceptedExternal { config, profile })
}

#[tauri::command]
pub fn dismiss_external_config(app: AppHandle) {
    if let Some(holder) = app.try_state::<ExternalConfig>() {
        if let Ok(mut guard) = holder.0.lock() {
            *guard = None;
        }
    }
}
