use crate::{config::Config, config_error::ConfigError, paths, recovery::RecoveryInfo};
use std::process::Command;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State};

pub struct RecoveryState(pub Mutex<Option<RecoveryInfo>>);

impl RecoveryState {
    pub fn new() -> Self {
        RecoveryState(Mutex::new(None))
    }
}

pub struct SafeMode(pub bool);

pub fn load_config(detected_disks: &[String]) -> (Option<Config>, Option<ConfigError>) {
    let config_path = paths::config_path();
    match crate::config::load_or_create(&config_path, detected_disks) {
        Ok(config) => (Some(config), None),
        Err(error) => {
            eprintln!("startup: {error}");
            (None, Some(error))
        }
    }
}

pub fn is_safe_mode(args: &[String]) -> bool {
    args.iter().any(|arg| arg == "--safe-mode")
}

#[tauri::command]
pub fn open_path(path: String) -> Result<(), String> {
    Command::new("xdg-open")
        .arg(&path)
        .spawn()
        .map_err(|e| format!("open: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn recovery_info(state: State<'_, RecoveryState>) -> Option<RecoveryInfo> {
    state.0.lock().ok()?.clone()
}

#[tauri::command]
pub fn recovery_action(app: AppHandle, action: String) -> Result<String, String> {
    match action.as_str() {
        "restore_backup" => {
            let cfg = crate::recovery::restore_backup().map_err(|e| e.to_string())?;
            let state = app.state::<crate::config_write::LastWrite>();
            if let Ok(mut guard) = state.0.lock() {
                *guard = crate::config::serialize(&cfg);
            }
            if let Some(state) = app.try_state::<crate::startup::RecoveryState>() {
                if let Ok(mut guard) = state.0.lock() {
                    *guard = None;
                }
            }
            if let Some(cfg_state) = app.try_state::<std::sync::Arc<std::sync::RwLock<Config>>>() {
                if let Ok(mut guard) = cfg_state.write() {
                    *guard = cfg.clone();
                }
                let reconcile_handle = app.clone();
                let _ = app.run_on_main_thread(move || {
                    crate::window::reconcile(&reconcile_handle, &cfg);
                });
            }
            Ok("restored".into())
        }
        "use_defaults" => {
            let detected = {
                let disks = sysinfo::Disks::new_with_refreshed_list();
                crate::disk::discover(&disks).into_iter().map(|d| d.id).collect::<Vec<_>>()
            };
            let cfg = crate::recovery::use_defaults(&detected);
            let state = app.state::<crate::config_write::LastWrite>();
            if let Ok(mut guard) = state.0.lock() {
                *guard = crate::config::serialize(&cfg);
            }
            if let Some(state) = app.try_state::<RecoveryState>() {
                if let Ok(mut guard) = state.0.lock() {
                    *guard = None;
                }
            }
            if let Some(cfg_state) = app.try_state::<std::sync::Arc<std::sync::RwLock<Config>>>() {
                if let Ok(mut guard) = cfg_state.write() {
                    *guard = cfg.clone();
                }
                let reconcile_handle = app.clone();
                let _ = app.run_on_main_thread(move || {
                    crate::window::reconcile(&reconcile_handle, &cfg);
                });
            }
            Ok("defaults".into())
        }
        "use_in_memory_defaults" => {
            let detected = {
                let disks = sysinfo::Disks::new_with_refreshed_list();
                crate::disk::discover(&disks).into_iter().map(|d| d.id).collect::<Vec<_>>()
            };
            let cfg = crate::recovery::use_in_memory_defaults(&detected);
            if let Some(state) = app.try_state::<RecoveryState>() {
                if let Ok(mut guard) = state.0.lock() {
                    *guard = None;
                }
            }
            if let Some(cfg_state) = app.try_state::<std::sync::Arc<std::sync::RwLock<Config>>>() {
                if let Ok(mut guard) = cfg_state.write() {
                    *guard = cfg.clone();
                }
            }
            Ok("defaults_in_memory".into())
        }
        "revalidate" => {
            let error = crate::recovery::revalidate().err().map(|e| e.to_string());
            let info = error.as_deref().map(|msg| crate::recovery::RecoveryInfo {
                message: msg.into(),
                config_path: paths::config_path().to_string_lossy().to_string(),
                backups_dir: paths::backups_dir().to_string_lossy().to_string(),
            });
            if let Some(state) = app.try_state::<RecoveryState>() {
                if let Ok(mut guard) = state.0.lock() {
                    *guard = info;
                }
            }
            Ok(match error {
                Some(_) => "invalid".into(),
                None => "valid".into(),
            })
        }
        _ => Err(format!("unknown recovery action: {action}")),
    }
}
