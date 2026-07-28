use crate::{config, config_write, paths, window};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    sync::mpsc::channel,
    sync::{Arc, RwLock},
    thread,
};
use tauri::{AppHandle, Manager};

pub fn start(app: AppHandle, config_state: Arc<RwLock<config::Config>>) {
    let path = paths::config_path();
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
            let is_ours = app
                .try_state::<config_write::LastWrite>()
                .and_then(|last| last.0.lock().ok().map(|guard| !config_write::should_reload(&guard, &contents)))
                .unwrap_or(false);
            if is_ours {
                continue;
            }
            if let Ok(next) = config::parse(&contents) {
                let mut guard = config_state.write().expect("config lock");
                *guard = next.clone();
                crate::audio::sync(&app, &next);
                let scheduler = app.clone();
                let reconcile_app = app.clone();
                let state = config_state.clone();
                let _ = scheduler.run_on_main_thread(move || {
                    if let Ok(config) = state.read() {
                        window::reconcile(&reconcile_app, &config);
                    }
                });
            }
        }
    });
}
