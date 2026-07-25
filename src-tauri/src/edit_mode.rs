use crate::{
    config::Config,
    config_write,
    window::{self, position, LABELS},
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use tauri::{AppHandle, Manager};
#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

#[derive(Default)]
pub struct EditMode(pub AtomicBool);

pub fn is_active(app: &AppHandle) -> bool {
    app.try_state::<EditMode>()
        .map(|state| state.0.load(Ordering::Relaxed))
        .unwrap_or(false)
}

pub fn apply_placement(mut config: Config, id: &str, placement: position::Placement) -> Config {
    if let Some(section) = config.sections.iter_mut().find(|section| section.id == id) {
        section.monitor = placement.monitor;
        section.x = placement.x;
        section.y = placement.y;
        section.width = placement.width;
    }
    config
}

pub fn merge_live_geometry(app: &AppHandle, config: Config) -> Config {
    if !is_active(app) {
        return config;
    }
    let mut merged = config;
    for label in LABELS {
        let Some(window) = app.get_webview_window(label) else { continue };
        let Some(placement) = position::capture(&window) else { continue };
        merged = apply_placement(merged, label, placement);
    }
    merged
}

pub fn enter(app: &AppHandle) {
    if let Some(state) = app.try_state::<EditMode>() {
        state.0.store(true, Ordering::Relaxed);
    }
    for label in LABELS {
        let Some(window) = app.get_webview_window(label) else { continue };
        let _ = window.show();
        let _ = window.set_ignore_cursor_events(false);
        let _ = window.set_always_on_bottom(false);
        let _ = window.set_resizable(true);
        #[cfg(target_os = "linux")]
        if let Ok(gtk_window) = window.gtk_window() {
            gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Normal);
            gtk_window.set_keep_below(false);
        }
    }
}

pub fn exit(app: &AppHandle) {
    let current = app
        .try_state::<Arc<RwLock<Config>>>()
        .and_then(|state| state.read().ok().map(|guard| guard.clone()));
    let Some(current) = current else { return };
    let next = merge_live_geometry(app, current);
    for label in LABELS {
        let Some(window) = app.get_webview_window(label) else { continue };
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
    if let Some(state) = app.try_state::<EditMode>() {
        state.0.store(false, Ordering::Relaxed);
    }
    config_write::apply(app, next);
}

pub fn reconcile_after_exit(app: &AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || exit(&handle));
}
