mod disk;
mod disk_linux;
mod collect;
mod config;
mod system_info;
mod temperature;
mod watch;
mod window;

#[cfg(test)]
mod disk_tests;
#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod system_info_tests;

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
                let display = if config.first_enabled_known_section().is_some() {
                    window
                        .current_monitor()
                        .ok()
                        .flatten()
                        .or_else(|| window.available_monitors().ok().and_then(|monitors| monitors.into_iter().next()))
                        .map(|monitor| {
                            let size = monitor.size();
                            let name = monitor.name().map(|name| name.to_string()).unwrap_or_default();
                            if name.is_empty() {
                                format!("{}x{}", size.width, size.height)
                            } else {
                                format!("{name} {}x{}", size.width, size.height)
                            }
                        })
                } else {
                    None
                };
                let system_fields = crate::system_info::collect_static(display);
                app.manage(config_state.clone());
                watch::start(app.handle().clone(), window.clone(), config_state.clone());
                tauri::async_runtime::spawn(collect::run(app.handle().clone(), config_state, system_fields));
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
