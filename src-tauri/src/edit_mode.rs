use crate::config::Config;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{AppHandle, Manager, WebviewWindow};
#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

use crate::edit_history::EditHistory;

pub struct EditTransaction {
    pub active: AtomicBool,
    pub snapshot: Mutex<Option<Config>>,
    pub history: Mutex<EditHistory>,
}

impl EditTransaction {
    pub fn new() -> Self {
        EditTransaction {
            active: AtomicBool::new(false),
            snapshot: Mutex::new(None),
            history: Mutex::new(EditHistory::new()),
        }
    }
}

#[derive(Default)]
pub struct EditMode(pub AtomicBool);

pub fn is_active(app: &AppHandle) -> bool {
    app.try_state::<EditMode>()
        .map(|state| state.0.load(Ordering::Relaxed))
        .unwrap_or(false)
}

pub fn prepare_window(window: &WebviewWindow) {
    let _ = window.set_ignore_cursor_events(false);
    let _ = window.set_always_on_bottom(false);
    let _ = window.set_resizable(true);
    #[cfg(target_os = "linux")]
    if let Ok(gtk_window) = window.gtk_window() {
        gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Normal);
        gtk_window.set_keep_below(false);
    }
}

pub fn restore_after_edit(app: &AppHandle) {
    let labels = app.webview_windows().keys()
        .filter(|label| label.as_str() != "settings" && label.as_str() != "layout-editor")
        .cloned().collect::<Vec<_>>();
    for label in labels {
        let Some(window) = app.get_webview_window(&label) else { continue };
        let _ = window.set_resizable(false);
        let _ = window.set_ignore_cursor_events(true);
        let _ = window.set_always_on_bottom(true);
        #[cfg(target_os = "linux")]
        if let Ok(gtk_window) = window.gtk_window() {
            gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Desktop);
            gtk_window.set_keep_below(true);
            gtk_window.stick();
        }
    }
}
