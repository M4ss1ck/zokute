use crate::config::Config;
use crate::config::Profile;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Mutex,
};
use tauri::{AppHandle, Manager, WebviewWindow};
#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

pub struct EditTransaction {
    pub active: AtomicBool,
    pub config: Mutex<Option<Config>>,
    pub profile: Mutex<Option<Profile>>,
    /// Set by the tray and CLI entry points so the dialog opens with Arrange
    /// already on. Read and cleared once, when the session begins.
    pub arm_arrange: AtomicBool,
}

impl EditTransaction {
    pub fn new() -> Self {
        EditTransaction {
            active: AtomicBool::new(false),
            config: Mutex::new(None),
            profile: Mutex::new(None),
            arm_arrange: AtomicBool::new(false),
        }
    }
}

/// True while the settings dialog holds an unsaved draft. Distinct from
/// `is_active`, which reports whether Arrange is on.
pub fn session_active(app: &AppHandle) -> bool {
    app.try_state::<EditTransaction>()
        .map(|tx| tx.active.load(Ordering::Relaxed))
        .unwrap_or(false)
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
