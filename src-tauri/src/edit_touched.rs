use std::{collections::HashSet, sync::Mutex};
use tauri::{AppHandle, Manager};

// Edit mode hands widget windows to the window manager as normal windows so
// they can be dragged, and the WM immediately shoves any window overlapping a
// panel strut back inside the work area. Capturing every window's geometry on
// exit would persist that shove as if it were the user's choice, so only the
// windows the user actually grabbed are captured.
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

#[tauri::command]
pub fn mark_widget_moved(app: AppHandle, id: String) {
    mark(&app, &id);
}
