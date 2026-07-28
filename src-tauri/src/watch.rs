use crate::{config, config_write, edit_mode, paths, window};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};

pub struct ExternalConfig(pub Mutex<Option<(String, config::Config)>>);

pub fn start(app: AppHandle, config_state: Arc<RwLock<config::Config>>) {
    let path = paths::config_path();
    app.manage(ExternalConfig(Mutex::new(None)));
    let app_clone = app.clone();
    thread::spawn(move || {
        let parent = path.parent().expect("config parent").to_path_buf();
        let (tx, rx) = channel();
        let mut watcher = RecommendedWatcher::new(
            move |result| {
                let _ = tx.send(result);
            },
            notify::Config::default(),
        )
        .expect("config watcher");
        watcher
            .watch(&parent, RecursiveMode::NonRecursive)
            .expect("watch config");
        while let Ok(result) = rx.recv() {
            let Ok(event) = result else { continue };
            if !event.paths.iter().any(|candidate| candidate == &path) {
                continue;
            }
            let Ok(contents) = std::fs::read_to_string(&path) else { continue };
            let is_ours = app_clone
                .try_state::<config_write::LastWrite>()
                .and_then(|last| last.0.lock().ok().map(|guard| !config_write::should_reload(&guard, &contents)))
                .unwrap_or(false);
            if is_ours {
                continue;
            }
            if !edit_mode::is_active(&app_clone) {
                if let Ok(next) = config::parse(&contents) {
                    let mut guard = config_state.write().expect("config lock");
                    *guard = next.clone();
                    crate::audio::sync(&app_clone, &next);
                    let scheduler = app_clone.clone();
                    let reconcile_app = app_clone.clone();
                    let state = config_state.clone();
                    let _ = scheduler.run_on_main_thread(move || {
                        if let Ok(config) = state.read() {
                            window::reconcile(&reconcile_app, &config);
                        }
                    });
                }
                continue;
            }
            if let Ok(next) = config::parse(&contents) {
                if let Some(holder) = app_clone.try_state::<ExternalConfig>() {
                    if let Ok(mut guard) = holder.0.lock() {
                        *guard = Some((contents, next));
                    }
                }
                let _ = app_clone.emit("external-config-changed", true);
            }
        }
    });
}

#[tauri::command]
pub fn accept_external_config(app: AppHandle) -> Result<(), String> {
    let state = app.try_state::<Arc<RwLock<config::Config>>>();
    let Some(state) = state else { return Ok(()) };
    let candidate = app
        .try_state::<ExternalConfig>()
        .and_then(|m| m.0.lock().ok().and_then(|mut g| g.take()));
    let Some((_, candidate_config)) = candidate else { return Ok(()) };
    {
        let mut guard = state.write().map_err(|e| e.to_string())?;
        *guard = candidate_config.clone();
    }
    crate::audio::sync(&app, &candidate_config);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        window::reconcile(&handle, &candidate_config);
    });
    Ok(())
}

#[tauri::command]
pub fn dismiss_external_config(app: AppHandle) -> Result<(), String> {
    if let Some(holder) = app.try_state::<ExternalConfig>() {
        if let Ok(mut guard) = holder.0.lock() {
            *guard = None;
        }
    }
    Ok(())
}
