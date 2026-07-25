use crate::edit_mode;
use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

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
    match built {
        Ok(window) => {
            let handle = app.clone();
            window.on_window_event(move |event| {
                if matches!(event, WindowEvent::Destroyed) {
                    edit_mode::reconcile_after_exit(&handle);
                }
            });
            edit_mode::enter(app);
        }
        Err(error) => eprintln!("{LABEL}: {error}"),
    }
}
