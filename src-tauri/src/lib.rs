mod autostart;
mod disk;
mod disk_linux;
mod collect;
mod config;
mod config_write;
mod edit_mode;
mod settings;
mod system_info;
mod temperature;
mod tray;
mod watch;
mod window;

#[cfg(test)]
mod autostart_tests;
#[cfg(test)]
mod config_duplicate_tests;
#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod config_write_tests;
#[cfg(test)]
mod disk_tests;
#[cfg(test)]
mod edit_mode_tests;
#[cfg(test)]
mod system_info_tests;
#[cfg(test)]
mod tray_tests;

use std::sync::{Arc, RwLock};
use tauri::Manager;

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![autostart::FLAG]),
        ))
        .invoke_handler(tauri::generate_handler![
            autostart::autostart_enabled,
            autostart::set_autostart,
            config_write::update_config,
            config_write::preview_opacity
        ])
        .setup(|app| {
            let detected_disks = {
                let disks = sysinfo::Disks::new_with_refreshed_list();
                crate::disk::discover(&disks).into_iter().map(|disk| disk.id).collect::<Vec<_>>()
            };
            let config_path = config::path();
            let config = config::load_or_create(&config_path, &detected_disks).expect("failed to load or create config");
            let config_state = Arc::new(RwLock::new(config.clone()));
            app.manage(config_state.clone());
            app.manage(config_write::LastWrite::default());
            app.manage(edit_mode::EditMode::default());
            if autostart::launched_by_autostart(std::env::args()) {
                let handle = app.handle().clone();
                let delayed = config.clone();
                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(autostart::STARTUP_DELAY_MS)).await;
                    let reconcile_handle = handle.clone();
                    let _ = handle.run_on_main_thread(move || {
                        window::reconcile(&reconcile_handle, &delayed);
                    });
                });
            } else {
                window::reconcile(app.handle(), &config);
            }
            let display = window::LABELS.iter().find_map(|label| {
                app.get_webview_window(*label).and_then(|window| {
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
                })
            });
            let system_fields = crate::system_info::collect_static(display);
            watch::start(app.handle().clone(), config_state.clone());
            tauri::async_runtime::spawn(collect::run(app.handle().clone(), config_state, system_fields));
            tray::init(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
