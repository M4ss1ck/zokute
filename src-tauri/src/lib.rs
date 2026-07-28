mod actions; mod atomic_file; mod autostart; mod collector_control;
pub mod cli; mod ipc; mod audio; mod audio_spectrum;
mod collect; mod config; mod config_error; mod config_validate;
mod config_write; mod disk; mod disk_io; mod disk_linux;
mod edit_geometry; mod edit_history; mod edit_mode; mod edit_touched;
mod fastfetch; mod fullscreen; mod gpu; mod gpu_linux; mod history_commands;
mod layout_commands; mod monitor; mod monitor_linux;
mod network; mod network_linux; mod paths;
mod plugin_cache; mod plugin_discovery; mod plugin_manifest;
mod plugin_protocol; mod plugin_runner; mod plugin_scheduler;
mod plugin_status; mod position_model; mod profile_io;
mod recovery; mod sensors; mod sensors_linux; mod settings;
mod snap; mod startup; mod stats_types; mod system_info; mod tick;
mod tray; mod visibility; mod watch; mod watch_external; mod window;

#[cfg(test)] mod autostart_tests;
#[cfg(test)] mod audio_tests;
#[cfg(test)] mod config_visualizer_tests;
#[cfg(test)] mod config_duplicate_tests;
#[cfg(test)] mod config_height_tests;
#[cfg(test)] mod config_clock_tests;
#[cfg(test)] mod config_date_tests;
#[cfg(test)] mod config_tests;
#[cfg(test)] mod config_fixture_tests;
#[cfg(test)] mod config_atomic_tests;
#[cfg(test)] mod config_write_tests;
#[cfg(test)] mod disk_tests;
#[cfg(test)] mod edit_mode_tests;
#[cfg(test)] mod fastfetch_tests;
#[cfg(test)] mod system_info_tests;
#[cfg(test)] mod tick_tests;
#[cfg(test)] mod tray_tests;

use std::sync::{Arc, RwLock};
use tauri::Manager;
use config::Profile;

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
            config_write::profile_commands_mod::update_profile,
            config_write::profile_commands_mod::activate_profile,
            config_write::preview_opacity,
            config_write::preview_text_opacity,
            config_write::update_widget_scale,
            edit_geometry::resize_widget,
            edit_touched::mark_widget_moved,
            config_write::remove_widget,
            startup::open_path,
            startup::recovery_info,
            startup::recovery_action,
            layout_commands::enter_edit_layout,
            layout_commands::save_layout,
            layout_commands::cancel_layout,
            history_commands::can_undo_edit,
            history_commands::can_redo_edit,
            history_commands::undo_edit,
            history_commands::redo_edit,
            history_commands::edit_move_widget,
            history_commands::edit_resize_widget,
            history_commands::bring_all_onto_visible,
            watch_external::accept_external_config,
            watch_external::dismiss_external_config,
            actions::open_uri,
            actions::copy_text,
            profile_io::duplicate_profile,
            profile_io::export_profile,
            profile_io::import_profile,
            profile_io::delete_profile,
        ])
        .setup(move |app| {
            app.manage(startup::SafeMode(safe));
            app.manage(startup::RecoveryState::new());
            app.manage(edit_mode::EditTransaction::new());
            let detected_disks = {
                let d = sysinfo::Disks::new_with_refreshed_list();
                crate::disk::discover(&d).into_iter().map(|disk| disk.id).collect::<Vec<_>>()
            };
            let (config, profile, config_error) = startup::load_config(&detected_disks);
            if let Some(error) = &config_error {
                if let Some(state) = app.try_state::<startup::RecoveryState>() {
                    if let Ok(mut guard) = state.0.lock() { *guard = Some(recovery::info_from(error)); }
                }
            }
            let config = config.unwrap_or_else(|| config::fresh_defaults(&detected_disks));
            let profile = profile.unwrap_or_else(|| config::fresh_profile(&detected_disks));
            let config_loaded = config_error.is_none();
            let config_state = Arc::new(RwLock::new(config.clone()));
            let profile_state = Arc::new(RwLock::new(profile.clone()));
            app.manage(config_state.clone());
            app.manage(profile_state.clone());
            app.manage(config_write::LastWrite::default());
            app.manage(edit_mode::EditMode::default());
            app.manage(edit_touched::Touched::default());
            app.manage(visibility::VisibilityState::default());
            if autostart::launched_by_autostart(std::env::args()) {
                let h = app.handle().clone(); let p = profile.clone(); let h2 = h.clone();
                tauri::async_runtime::spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_millis(autostart::STARTUP_DELAY_MS)).await;
                    let _ = h.run_on_main_thread(move || { if config_loaded { window::reconcile(&h2, &p); } });
                });
            } else if config_loaded { window::reconcile(app.handle(), &profile); }
            watch::start(app.handle().clone(), config_state.clone(), profile_state.clone());
            if config_loaded && !safe { audio::start(app.handle().clone(), &profile); }
            tauri::async_runtime::spawn(collect::run(app.handle().clone(), config_state, profile_state));
            fullscreen::start(app.handle().clone());
            ipc::start(app.handle().clone());
            tray::init(app.handle())?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
