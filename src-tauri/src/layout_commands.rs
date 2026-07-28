use crate::{
    config::{Config, Profile}, config_write,
    edit_mode::{self, EditTransaction, prepare_window, restore_after_edit},
    edit_touched,
};
use std::sync::{atomic::Ordering, Arc, RwLock};
use tauri::{AppHandle, Manager};

fn open_layout_editor(app: &AppHandle) {
    if app.get_webview_window("layout-editor").is_some() { return; }
    let _ = tauri::WebviewWindowBuilder::new(app, "layout-editor", tauri::WebviewUrl::App("index.html".into()))
        .title("Edit Layout")
        .inner_size(320.0, 400.0)
        .resizable(true)
        .decorations(true)
        .transparent(false)
        .center()
        .build();
}

#[tauri::command]
pub fn enter_edit_layout(app: AppHandle) {
    let Some(profile_state) = app.try_state::<Arc<RwLock<Profile>>>() else { return };
    let profile = profile_state.read().ok().map(|guard| guard.clone());
    let Some(profile) = profile else { return };
    if let Some(tx) = app.try_state::<EditTransaction>() {
        if let Ok(mut snapshot) = tx.snapshot.lock() { *snapshot = Some(profile.clone()); }
        if let Ok(mut history) = tx.history.lock() { history.clear(); }
        tx.active.store(true, Ordering::Relaxed);
    }
    if let Some(state) = app.try_state::<edit_mode::EditMode>() {
        state.0.store(true, Ordering::Relaxed);
    }
    edit_touched::clear(&app);
    open_layout_editor(&app);
    for label in app.webview_windows().keys()
        .filter(|l| l.as_str() != "settings" && l.as_str() != "layout-editor")
        .cloned().collect::<Vec<_>>() {
        if let Some(window) = app.get_webview_window(&label) {
            let _ = window.show();
            prepare_window(&window);
        }
    }
}

#[tauri::command]
pub fn save_layout(app: AppHandle) -> Result<(), String> {
    let config = app.try_state::<Arc<RwLock<Config>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone())).ok_or("no config")?;
    let profile = app.try_state::<Arc<RwLock<Profile>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone())).ok_or("no profile")?;
    let merged_profile = crate::edit_geometry::merge_live_geometry(&app, profile);
    config_write::apply(&app, config, merged_profile);
    finish_transaction(&app);
    Ok(())
}

#[tauri::command]
pub fn cancel_layout(app: AppHandle) -> Result<(), String> {
    let snapshot = app.try_state::<EditTransaction>()
        .and_then(|tx| tx.snapshot.lock().ok().and_then(|mut s| s.take()));
    let Some(original) = snapshot else { return Ok(()) };
    if let Some(profile_state) = app.try_state::<Arc<RwLock<Profile>>>() {
        if let Ok(mut guard) = profile_state.write() { *guard = original.clone(); }
    }
    finish_transaction(&app);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || crate::window::reconcile(&handle, &original));
    Ok(())
}

fn finish_transaction(app: &AppHandle) {
    if let Some(tx) = app.try_state::<EditTransaction>() {
        tx.active.store(false, Ordering::Relaxed);
        if let Ok(mut s) = tx.snapshot.lock() { *s = None; }
        if let Ok(mut h) = tx.history.lock() { h.clear(); }
    }
    if let Some(state) = app.try_state::<edit_mode::EditMode>() {
        state.0.store(false, Ordering::Relaxed);
    }
    if let Some(window) = app.get_webview_window("layout-editor") { let _ = window.close(); }
    restore_after_edit(app);
}
