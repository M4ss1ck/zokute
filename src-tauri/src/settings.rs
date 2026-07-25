use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

pub const LABEL: &str = "settings";

pub fn open(app: &AppHandle) {
    if let Some(existing) = app.get_webview_window(LABEL) {
        let _ = existing.set_focus();
        return;
    }
    let built = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("index.html".into()))
        .title("Zokute Settings")
        .inner_size(420.0, 640.0)
        .min_inner_size(360.0, 420.0)
        .resizable(true)
        .decorations(true)
        .transparent(false)
        .center()
        .build();
    if let Err(error) = built {
        eprintln!("{LABEL}: {error}");
    }
}
