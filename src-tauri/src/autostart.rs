use crate::startup::SafeMode;
use tauri::{AppHandle, Manager};
use tauri_plugin_autostart::ManagerExt;

pub const FLAG: &str = "--autostart";

pub const STARTUP_DELAY_MS: u64 = 3000;

pub fn launched_by_autostart(args: impl Iterator<Item = String>) -> bool {
    args.skip(1).any(|arg| arg == FLAG)
}

#[tauri::command]
pub fn autostart_enabled(app: AppHandle) -> bool {
    app.autolaunch().is_enabled().unwrap_or(false)
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) -> Result<(), String> {
    if app.try_state::<SafeMode>().is_some_and(|s| s.0) {
        return Ok(());
    }
    let manager = app.autolaunch();
    if enabled {
        manager.enable().map_err(|error| error.to_string())
    } else {
        manager.disable().map_err(|error| error.to_string())
    }
}
