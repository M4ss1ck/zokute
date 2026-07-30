use crate::{cli, diagnostics, edit_mode, settings_session, window};
use serde::Serialize;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::sync::{Arc, RwLock};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};

#[derive(Serialize)]
struct StatusResponse {
    version: &'static str,
    running: bool,
    editing: bool,
}

pub fn start(app: AppHandle) {
    let app_clone = app.clone();
    thread::spawn(move || {
        let path = cli::socket_path();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let _ = fs::remove_file(&path);
        let Ok(listener) = UnixListener::bind(&path) else { return };
        fs::set_permissions(&path, std::os::unix::fs::PermissionsExt::from_mode(0o600)).ok();
        for incoming in listener.incoming() {
            let Ok(client) = incoming else { continue };
            handle_client(client, &app_clone);
        }
    });
}

fn handle_client(mut stream: UnixStream, app: &AppHandle) {
    let mut buf = vec![0u8; 8192];
    let Ok(n) = stream.read(&mut buf) else { return };
    let cmd_str = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    let response = if cmd_str.starts_with('{') {
        handle_json_command(&cmd_str, app)
    } else {
        handle_text_command(&cmd_str, app)
    };
    let _ = stream.write_all(response.as_bytes());
}

fn handle_text_command(cmd_str: &str, app: &AppHandle) -> String {
    match cmd_str {
        "show" => { window::show_all(app); "ok".into() }
        "hide" => { window::hide_all(app); "ok".into() }
        "toggle" => { window::toggle_visibility(app); "ok".into() }
        "edit" => { settings_session::open_settings_arranging(app.clone()); "ok".into() }
        "reload" => {
            if let Some(state) = app.try_state::<Arc<RwLock<crate::config::Profile>>>() {
                if let Ok(profile) = state.read().map(|g| g.clone()) {
                    let h = app.clone();
                    let _ = app.run_on_main_thread(move || window::reconcile(&h, &profile));
                }
            }
            "ok".into()
        }
        "status" => {
            let status = StatusResponse {
                version: "0.1.0",
                running: true,
                editing: edit_mode::session_active(app),
            };
            serde_json::to_string(&status).unwrap_or_else(|_| "{}".into())
        }
        _ => "error: unknown command".into(),
    }
}

fn handle_json_command(json_str: &str, app: &AppHandle) -> String {
    let value: serde_json::Value = match serde_json::from_str(json_str) {
        Ok(v) => v,
        Err(_) => return "error: invalid json".into(),
    };
    match value.get("command").and_then(|c| c.as_str()) {
        Some("config_show") => {
            let redact = value.get("redact_plugin_config").and_then(|v| v.as_bool()).unwrap_or(false);
            let config = app.try_state::<Arc<RwLock<crate::config::Config>>>()
                .and_then(|s| s.read().ok().map(|g| g.clone()));
            let profile = app.try_state::<Arc<RwLock<crate::config::Profile>>>()
                .and_then(|s| s.read().ok().map(|g| g.clone()));
            match (config, profile) {
                (Some(c), Some(p)) => crate::diagnostics_config::show_config(redact, &c, &p),
                _ => "error: config not loaded".into(),
            }
        }
        Some("diagnostics") => {
            let config = app.try_state::<Arc<RwLock<crate::config::Config>>>()
                .and_then(|s| s.read().ok().map(|g| g.clone()));
            let profile = app.try_state::<Arc<RwLock<crate::config::Profile>>>()
                .and_then(|s| s.read().ok().map(|g| g.clone()));
            let report = diagnostics::collect(config.as_ref(), profile.as_ref(), detection_mode(&app));
            serde_json::to_string_pretty(&report).unwrap_or_else(|_| "error: serialization".into())
        }
        _ => "error: unknown command".into(),
    }
}

fn detection_mode(app: &AppHandle) -> crate::diagnostics::DetectionMode {
    match app.try_state::<crate::fullscreen_driver::FullscreenState>() {
        Some(state) if state.is_degraded() => crate::diagnostics::DetectionMode::DegradedPoll,
        _ => crate::diagnostics::DetectionMode::Events,
    }
}
