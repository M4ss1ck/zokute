use gtk::prelude::*;
use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub fn create(app: &AppHandle, label: &str, interactive: bool) -> tauri::Result<WebviewWindow> {
    let window = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .visible(false)
        .inner_size(96.0, 72.0)
        .transparent(true)
        .decorations(false)
        .resizable(true)
        .shadow(false)
        .skip_taskbar(true)
        .build()?;
    #[cfg(target_os = "linux")]
    let _ = window.with_webview(|webview| {
        webview.inner().set_size_request(1, 1);
    });
    let _ = window.set_always_on_bottom(true);
    #[cfg(target_os = "linux")]
    if let Ok(gtk_window) = window.gtk_window() {
        gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Desktop);
        gtk_window.stick();
        gtk_window.realize();
    }
    if !interactive {
        let _ = window.set_ignore_cursor_events(true);
    }
    let handle = app.clone();
    let moved_label = label.to_string();
    window.on_window_event(move |event| {
        if let tauri::WindowEvent::Moved(position) = event {
            crate::guides_drag::on_moved(&handle, &moved_label, *position);
        }
    });
    Ok(window)
}
