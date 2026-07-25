use tauri::AppHandle;
use tauri_plugin_autostart::ManagerExt;

pub const FLAG: &str = "--autostart";

// Cinnamon's compositor and desktop layer are not settled when a session
// autostart entry fires; creating desktop-hint windows before that leaves them
// stacked above the desktop or placed on the wrong monitor.
pub const STARTUP_DELAY_MS: u64 = 3000;

pub fn launched_by_autostart(args: impl Iterator<Item = String>) -> bool {
    args.skip(1).any(|arg| arg == FLAG)
}

#[tauri::command]
pub fn autostart_enabled(app: AppHandle) -> bool {
    app.autolaunch().is_enabled().unwrap_or(false)
}

#[tauri::command]
pub fn set_autostart(app: AppHandle, enabled: bool) {
    let manager = app.autolaunch();
    let result = if enabled { manager.enable() } else { manager.disable() };
    if let Err(error) = result {
        eprintln!("autostart: {error}");
    }
}
