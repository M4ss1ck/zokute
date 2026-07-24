use crate::{config, window};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    sync::mpsc::channel,
    sync::{Arc, RwLock},
    thread,
};
use tauri::{AppHandle, WebviewWindow};

pub fn start(app: AppHandle, window: WebviewWindow, config_state: Arc<RwLock<config::Config>>) {
    let path = config::path();
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
            if let Ok(next) = config::load(&path) {
                let mut guard = config_state.write().expect("config lock");
                *guard = next.clone();
                let app = app.clone();
                let window = window.clone();
                let _ = app.run_on_main_thread(move || {
                    window::apply_config(&window, &next);
                });
            }
        }
    });
}
