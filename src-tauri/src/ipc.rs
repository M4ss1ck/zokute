use crate::cli;
use crate::{edit_mode, layout_commands, window};
use serde::Serialize;
use std::fs;
use std::io::{Read, Write};
use std::os::unix::net::{UnixListener, UnixStream};
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
    let mut buf = vec![0u8; 4096];
    let Ok(n) = stream.read(&mut buf) else { return };
    let cmd_str = String::from_utf8_lossy(&buf[..n]).trim().to_string();
    let response = match cmd_str.as_str() {
        "show" => {
            window::show_all(app);
            "ok".into()
        }
        "hide" => {
            window::hide_all(app);
            "ok".into()
        }
        "toggle" => {
            window::toggle_visibility(app);
            "ok".into()
        }
        "edit" => {
            layout_commands::enter_edit_layout(app.clone());
            "ok".into()
        }
        "reload" => {
            if let Some(state) = app.try_state::<std::sync::Arc<std::sync::RwLock<crate::config::Profile>>>() {
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
                editing: edit_mode::is_active(app),
            };
            serde_json::to_string(&status).unwrap_or_else(|_| "{}".into())
        }
        _ => "error: unknown command".into(),
    };
    let _ = stream.write_all(response.as_bytes());
}
