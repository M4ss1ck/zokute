use crate::config::Profile;
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
    pub snapshot: Mutex<Option<Profile>>,
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
        gtk_window.set_keep_below(false);
    }
}

pub fn restore_window(window: &WebviewWindow, interactive: bool) {
    let _ = window.set_resizable(false);
    if !interactive {
        let _ = window.set_ignore_cursor_events(true);
    }
    let _ = window.set_always_on_bottom(true);
    #[cfg(target_os = "linux")]
    if let Ok(gtk_window) = window.gtk_window() {
        gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Desktop);
        gtk_window.set_keep_below(true);
        gtk_window.stick();
    }
}

pub fn restore_after_edit(app: &AppHandle) {
    let profile = app.try_state::<std::sync::Arc<std::sync::RwLock<crate::config::Profile>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone()));
    let labels = app.webview_windows().keys()
        .filter(|label| !crate::window::is_control_window(label))
        .cloned().collect::<Vec<_>>();
    for label in labels {
        let Some(window) = app.get_webview_window(&label) else { continue };
        let interactive = profile.as_ref().and_then(|p| p.section(&label)).map(|s| s.interactive).unwrap_or(false);
        restore_window(&window, interactive);
    }
}
