use gtk::prelude::*;
use tauri::{AppHandle, WebviewUrl, WebviewWindow, WebviewWindowBuilder};

pub fn create(app: &AppHandle, label: &str) -> tauri::Result<WebviewWindow> {
    let window = WebviewWindowBuilder::new(app, label, WebviewUrl::App("index.html".into()))
        .visible(false)
        .inner_size(96.0, 72.0)
        .transparent(true)
        .decorations(false)
        .resizable(true)
        .shadow(false)
        .skip_taskbar(true)
        .build()?;
    let _ = window.set_always_on_bottom(true);
    #[cfg(target_os = "linux")]
    if let Ok(gtk_window) = window.gtk_window() {
        gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Desktop);
        gtk_window.stick();
        gtk_window.realize();
    }
    let _ = window.set_ignore_cursor_events(true);
    Ok(window)
}
