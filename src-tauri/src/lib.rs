mod disk;
mod disk_linux;
mod collect;
mod config;
mod temperature;
mod watch;
mod window;

#[cfg(test)]
mod disk_tests;
#[cfg(test)]
mod config_tests;

use std::sync::{Arc, RwLock};
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let detected_disks = {
                let disks = sysinfo::Disks::new_with_refreshed_list();
                crate::disk::discover(&disks).into_iter().map(|disk| disk.id).collect::<Vec<_>>()
            };
            let config_path = config::path();
            let config = config::load_or_create(&config_path, &detected_disks);
            let config_state = Arc::new(RwLock::new(config.clone()));
            if let Some(window) = app.get_webview_window("main") {
                window::configure(&window);
                window::apply_config(&window, &config);
                app.manage(config_state.clone());
                watch::start(app.handle().clone(), window.clone(), config_state.clone());
                tauri::async_runtime::spawn(collect::run(app.handle().clone(), config_state));
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
