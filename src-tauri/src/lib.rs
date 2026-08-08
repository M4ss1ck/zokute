mod actions; mod atomic_file; mod config_section_defaults; mod background; mod onboarding_presets; mod audio_bands; mod baselines; mod cadence; mod diagnostics_config; mod fullscreen_driver; mod fullscreen_x11; mod autostart; mod collector_control;
#[cfg(test)] mod paths_tests; #[cfg(test)] mod background_tests; #[cfg(test)] mod actions_tests; #[cfg(test)] mod plugin_manifest_tests; #[cfg(test)] mod onboarding_tests; #[cfg(test)] mod audio_bands_tests;
#[cfg(test)] mod cli_tests; #[cfg(test)] mod plugin_protocol_tests; #[cfg(test)] mod visibility_tests; #[cfg(test)] mod baselines_tests;
#[cfg(test)] mod cadence_tests; #[cfg(test)] mod collector_control_tests; #[cfg(test)] mod fullscreen_tests; #[cfg(test)] mod diagnostics_tests;
#[cfg(test)] mod diagnostics_config_tests; #[cfg(test)] mod autostart_tests; #[cfg(test)] mod audio_tests; #[cfg(test)] mod config_visualizer_tests;
#[cfg(test)] mod config_duplicate_tests; #[cfg(test)] mod config_height_tests; #[cfg(test)] mod config_clock_tests; #[cfg(test)] mod config_date_tests;
#[cfg(test)] mod config_tests; #[cfg(test)] mod config_fixture_tests; #[cfg(test)] mod config_atomic_tests; #[cfg(test)] mod config_doc_tests;
#[cfg(test)] mod config_write_tests; #[cfg(test)] mod disk_tests; #[cfg(test)] mod edit_mode_tests; #[cfg(test)] mod fastfetch_tests;
#[cfg(test)] mod system_info_tests; #[cfg(test)] mod tick_tests; #[cfg(test)] mod tray_tests;
#[cfg(test)] mod sensors_tests; #[cfg(test)] mod settings_session_tests;
pub mod cli; mod diagnostics; mod ipc; mod audio; mod audio_spectrum;
mod collect; mod config; mod config_error; mod config_validate;
mod config_write; mod disk; mod disk_io; mod disk_linux;
mod edit_geometry; mod edit_mode; mod edit_touched;
mod fastfetch; mod fullscreen; mod gpu; mod gpu_linux;
mod logging; mod monitor; mod monitor_linux;
mod network; mod network_linux; mod onboarding;
mod monitor_cycle; mod paths; mod platform;
mod plugin_cache; mod plugin_discovery; mod plugin_manifest;
mod plugin_protocol; mod plugin_runner; mod plugin_scheduler;
mod plugin_status; mod position_model; mod profile_io;
mod recovery; mod sensors; mod sensors_linux; mod session_guard;
mod settings; mod settings_session; mod snap; mod startup; mod stats_types; mod system_info; mod tick;
mod tray; mod visibility; mod watch; mod watch_external; mod window;



use std::sync::{Arc, RwLock};
use tauri::Manager;

pub fn run() {
    let safe = startup::is_safe_mode(&std::env::args().collect::<Vec<_>>());
    let session = platform::detect();
    let skip_hud = session_guard::skip_hud_windows(&session);
    tauri::Builder::default()
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            Some(vec![autostart::FLAG]),
        ))
        .invoke_handler(tauri::generate_handler![
            autostart::autostart_enabled,
            autostart::set_autostart,
            config_write::draft_config,
            config_write::profile_commands_mod::draft_profile,
            config_write::profile_commands_mod::activate_profile,
            config_write::preview_opacity,
            config_write::preview_text_opacity,
            config_write::update_widget_scale,
            edit_geometry::resize_widget,
            edit_touched::mark_widget_moved,
            config_write::remove_widget,
            startup::open_path,
            startup::recovery_info,
            paths::config_location_cmd,
            startup::recovery_action,
            settings_session::begin_settings_session,
            settings_session::save_settings,
            settings_session::discard_settings,
            settings_session::set_arrange,
            settings_session::open_settings_arranging,
            watch_external::accept_external_config,
            watch_external::dismiss_external_config,
            actions::open_uri,
            actions::copy_text,
            profile_io::duplicate_profile,
            profile_io::export_profile,
            profile_io::import_profile,
            profile_io::delete_profile,
            onboarding::needs_onboarding_cmd,
            onboarding::onboarding_work_areas,
            onboarding::apply_onboarding,
        ])
        .setup(move |app| {
            app.manage(startup::SafeMode(safe));
            app.manage(startup::RecoveryState::new());
            app.manage(edit_mode::EditTransaction::new());
            app.manage(session.clone());
            let first_run = onboarding::needs_onboarding();
            let detected_disks = {
                let d = sysinfo::Disks::new_with_refreshed_list();
                crate::disk::discover(&d).into_iter().map(|disk| disk.id).collect::<Vec<_>>()
            };
            let (config, profile, config_loaded) = if first_run {
                let config = config::fresh_defaults(&detected_disks);
                let profile = config::fresh_profile(&detected_disks);
                (config, profile, false)
            } else {
                let (config, profile, config_error) = startup::load_config(&detected_disks);
                if let Some(error) = &config_error {
                    if let Some(state) = app.try_state::<startup::RecoveryState>() {
                        if let Ok(mut guard) = state.0.lock() { *guard = Some(recovery::info_from(error)); }
                    }
                }
                let config = config.unwrap_or_else(|| config::fresh_defaults(&detected_disks));
                let profile = profile.unwrap_or_else(|| config::fresh_profile(&detected_disks));
                (config, profile, config_error.is_none())
            };
            let config_state = Arc::new(RwLock::new(config.clone()));
            let profile_state = Arc::new(RwLock::new(profile.clone()));
            app.manage(config_state.clone());
            app.manage(profile_state.clone());
            app.manage(config_write::LastWrite::default());
            app.manage(config_write::LastProfileWrite::default());
            app.manage(edit_mode::EditMode::default());
            app.manage(edit_touched::Touched::default());
            app.manage(visibility::VisibilityState::default());
            app.manage(background::StartLatch::default());
            app.manage(onboarding::NeedsOnboarding(Arc::new(std::sync::atomic::AtomicBool::new(first_run))));
            let skip_hud_local = skip_hud || first_run;
            if !skip_hud_local {
                if autostart::launched_by_autostart(std::env::args()) {
                    let h = app.handle().clone(); let p = profile.clone(); let h2 = h.clone();
                    tauri::async_runtime::spawn(async move {
                        tokio::time::sleep(std::time::Duration::from_millis(autostart::STARTUP_DELAY_MS)).await;
                        let _ = h.run_on_main_thread(move || { if config_loaded { window::reconcile(&h2, &p); } });
                    });
                } else if config_loaded && !first_run { window::reconcile(app.handle(), &profile); }
            }
            // A first run has no config yet; onboarding starts this work once it
            // has written one.
            if !first_run && config_loaded && !safe && !crate::session_guard::suppress_background_work(&session) {
                background::start(app.handle(), config_state, profile_state);
            }
            fullscreen_driver::start(app.handle().clone());
            logging::init();
            ipc::start(app.handle().clone());
            tray::init(app.handle(), &session)?;
            if first_run {
                crate::settings::open(app.handle());
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}
