use std::{collections::HashSet, sync::Mutex};
use tauri::{AppHandle, Manager};
#[cfg(target_os = "linux")]
use gtk::prelude::GtkWindowExt;

// Only geometry the user deliberately changed belongs in the saved layout.
// Merely opening the editor must leave every untouched widget exactly where it
// was, including positions outside a monitor's normal work area.
#[derive(Default)]
pub struct Touched(pub Mutex<HashSet<String>>);

pub fn clear(app: &AppHandle) {
    if let Some(state) = app.try_state::<Touched>() {
        if let Ok(mut guard) = state.0.lock() {
            guard.clear();
        }
    }
}

pub fn mark(app: &AppHandle, instance: &str) {
    if let Some(state) = app.try_state::<Touched>() {
        if let Ok(mut guard) = state.0.lock() {
            guard.insert(instance.to_string());
        }
    }
}

pub fn was_grabbed(app: &AppHandle, instance: &str) -> bool {
    app.try_state::<Touched>()
        .and_then(|state| state.0.lock().ok().map(|guard| guard.contains(instance)))
        .unwrap_or(false)
}

pub fn any_touched(app: &AppHandle) -> bool {
    app.try_state::<Touched>()
        .and_then(|state| state.0.lock().ok().map(|guard| !guard.is_empty()))
        .unwrap_or(false)
}

#[tauri::command]
pub fn mark_widget_moved(app: AppHandle, id: String) {
    mark(&app, &id);
    #[cfg(target_os = "linux")]
    if let Some(window) = app.get_webview_window(&id) {
        if let Ok(gtk_window) = window.gtk_window() {
            gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Normal);
        }
    }
}
