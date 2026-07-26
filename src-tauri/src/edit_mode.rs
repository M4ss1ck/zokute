use crate::{
    config::Config,
    config_write,
    window::position,
};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, RwLock,
};
use tauri::{AppHandle, Manager, WebviewWindow};
#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

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

pub fn apply_placement(mut config: Config, instance: &str, placement: position::Placement) -> Config {
    if let Some(section) = config.sections.iter_mut().find(|section| section.instance == instance) {
        section.monitor = placement.monitor;
        section.x = placement.x;
        section.y = placement.y;
        section.width = placement.width;
        section.height = Some(placement.height);
    }
    config
}

pub fn merge_live_geometry(app: &AppHandle, config: Config) -> Config {
    if !is_active(app) {
        return config;
    }
    let mut merged = config;
    let current = app
        .try_state::<Arc<RwLock<Config>>>()
        .and_then(|state| state.read().ok().map(|guard| guard.clone()));
    let instances = merged.sections.iter().map(|section| section.instance.clone()).collect::<Vec<_>>();
    for label in instances {
        if let Some(scale) = current.as_ref().and_then(|config| config.section(&label)).map(|section| section.scale) {
            if let Some(section) = merged.sections.iter_mut().find(|section| section.instance == label) {
                section.scale = scale;
            }
        }
        let Some(window) = app.get_webview_window(&label) else { continue };
        let Some(placement) = position::capture(&window) else { continue };
        merged = apply_placement(merged, &label, placement);
    }
    merged
}

pub fn enter(app: &AppHandle) {
    if let Some(state) = app.try_state::<EditMode>() {
        state.0.store(true, Ordering::Relaxed);
    }
    let labels = app.webview_windows().keys().filter(|label| label.as_str() != "settings").cloned().collect::<Vec<_>>();
    for label in labels {
        let Some(window) = app.get_webview_window(&label) else { continue };
        let _ = window.show();
        prepare_window(&window);
    }
}

pub fn exit(app: &AppHandle) {
    let current = app
        .try_state::<Arc<RwLock<Config>>>()
        .and_then(|state| state.read().ok().map(|guard| guard.clone()));
    let Some(current) = current else { return };
    let next = merge_live_geometry(app, current);
    let labels = app.webview_windows().keys().filter(|label| label.as_str() != "settings").cloned().collect::<Vec<_>>();
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
    if let Some(state) = app.try_state::<EditMode>() {
        state.0.store(false, Ordering::Relaxed);
    }
    config_write::apply(app, next);
}

pub fn reconcile_after_exit(app: &AppHandle) {
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || exit(&handle));
}
