mod atomic_file;
mod autostart;
mod audio;
mod audio_spectrum;
mod config;
mod config_error;
mod config_validate;
mod config_write;
mod disk;
mod disk_linux;
mod collect;
mod edit_mode;
mod edit_touched;
mod fastfetch;
mod paths;
mod recovery;
mod settings;
mod startup;
mod system_info;
mod temperature;
mod tick;
mod tray;
mod watch;
mod window;

#[cfg(test)]
mod autostart_tests;
#[cfg(test)]
mod audio_tests;
#[cfg(test)]
mod config_visualizer_tests;
#[cfg(test)]
mod config_duplicate_tests;
#[cfg(test)]
mod config_height_tests;
#[cfg(test)]
mod config_clock_tests;
#[cfg(test)]
mod config_date_tests;
#[cfg(test)]
mod config_tests;
#[cfg(test)]
mod config_fixture_tests;
#[cfg(test)]
mod config_atomic_tests;
#[cfg(test)]
mod config_write_tests;
#[cfg(test)]
mod disk_tests;
#[cfg(test)]
mod edit_mode_tests;
#[cfg(test)]
mod fastfetch_tests;
#[cfg(test)]
mod system_info_tests;
#[cfg(test)]
mod tick_tests;
#[cfg(test)]
mod tray_tests;

use std::sync::{Arc, RwLock};
use tauri::Manager;

pub fn run() {
    let safe = startup::is_safe_mode(&std::env::args().collect::<Vec<_>>());
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![autostart::FLAG]),
        ))
        .invoke_handler(tauri::generate_handler![
            autostart::autostart_enabled,
            autostart::set_autostart,
            config_write::update_config,
            config_write::preview_opacity,
            config_write::preview_text_opacity,
            config_write::update_widget_scale,
            edit_mode::resize_widget,
            edit_touched::mark_widget_moved,
            config_write::remove_widget,
            startup::open_path,
            startup::recovery_info,
            startup::recovery_action
        ])
        .setup(move |app| {
            app.manage(startup::SafeMode(safe));
            app.manage(startup::RecoveryState::new());
            let detected_disks = {
                let disks = sysinfo::Disks::new_with_refreshed_list();
                crate::disk::discover(&disks).into_iter().map(|disk| disk.id).collect::<Vec<_>>()
            };
            let (config, config_error) = startup::load_config(&detected_disks);
            if let Some(error) = &config_error {
                if let Some(state) = app.try_state::<startup::RecoveryState>() {
                    if let Ok(mut guard) = state.0.lock() {
                        *guard = Some(recovery::info_from(error));
                    }
                }
            }
            let config = config.unwrap_or_else(|| config::fresh_defaults(&detected_disks));
            let config_loaded = config_error.is_none();
            let config_state = Arc::new(RwLock::new(config.clone()));
            app.manage(config_state.clone());
            app.manage(config_write::LastWrite::default());
            app.manage(edit_mode::EditMode::default());
            app.manage(edit_touched::Touched::default());
            if autostart::launched_by_autostart(std::env::args()) {
                let handle = app.handle().clone();
                let delayed = config.clone();
                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(autostart::STARTUP_DELAY_MS)).await;
                    let reconcile_handle = handle.clone();
                    let _ = handle.run_on_main_thread(move || {
                        if config_loaded { window::reconcile(&reconcile_handle, &delayed); }
                    });
                });
            } else if config_loaded {
                window::reconcile(app.handle(), &config);
            }
            watch::start(app.handle().clone(), config_state.clone());
            if config_loaded && !safe {
                audio::start(app.handle().clone(), &config);
            }
            tauri::async_runtime::spawn(collect::run(app.handle().clone(), config_state));
            tray::init(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
