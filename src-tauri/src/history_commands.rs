use crate::{
    config::Config,
    edit_history::EditHistory,
    edit_mode::EditTransaction,
};
use std::sync::{atomic::Ordering, Arc, RwLock};
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn can_undo_edit(app: AppHandle) -> bool {
    app.try_state::<EditTransaction>()
        .map(|tx| tx.active.load(Ordering::Relaxed) && tx.history.lock().ok().map_or(false, |h| h.can_undo()))
        .unwrap_or(false)
}

#[tauri::command]
pub fn can_redo_edit(app: AppHandle) -> bool {
    app.try_state::<EditTransaction>()
        .map(|tx| tx.active.load(Ordering::Relaxed) && tx.history.lock().ok().map_or(false, |h| h.can_redo()))
        .unwrap_or(false)
}

#[tauri::command]
pub fn undo_edit(app: AppHandle) -> Result<(), String> {
    operate_history(app, |history, current| history.undo(current))
}

#[tauri::command]
pub fn redo_edit(app: AppHandle) -> Result<(), String> {
    operate_history(app, |history, current| history.redo(current))
}

fn operate_history<F>(app: AppHandle, op: F) -> Result<(), String>
where F: FnOnce(&mut EditHistory, Config) -> Option<Config> {
    let Some(tx) = app.try_state::<EditTransaction>() else { return Ok(()) };
    if !tx.active.load(Ordering::Relaxed) { return Ok(()); }
    let Some(config_state) = app.try_state::<Arc<RwLock<Config>>>() else { return Ok(()) };
    let current = config_state.read().ok().map(|guard| guard.clone());
    let Some(current) = current else { return Ok(()) };
    let restored = tx.history.lock().ok().and_then(|mut history| op(&mut history, current));
    let Some(restored) = restored else { return Ok(()) };
    if let Ok(mut guard) = config_state.write() { *guard = restored.clone(); }
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        crate::window::reconcile(&handle, &restored);
    });
    Ok(())
}

#[tauri::command]
pub fn edit_move_widget(app: AppHandle, instance: String, x: i32, y: i32) -> Result<(), String> {
    let Some(tx) = app.try_state::<EditTransaction>() else { return Ok(()) };
    if !tx.active.load(Ordering::Relaxed) { return Ok(()); }
    let Some(config_state) = app.try_state::<Arc<RwLock<Config>>>() else { return Ok(()) };
    let prev = config_state.read().ok().map(|guard| guard.clone());
    let Some(prev) = prev else { return Ok(()) };
    let (snapped_x, snapped_y, _) = crate::snap::snap_position(x, y, 0, 0, 0, 0, 1920, 1080, &[], crate::snap::DEFAULT_TOLERANCE, false);
    if let Ok(mut guard) = config_state.write() {
        if let Some(section) = guard.sections.iter_mut().find(|s| s.instance == instance) {
            let old_pos = section.effective_position();
            let identity = old_pos.monitor_identity().to_string();
            section.position = Some(crate::position_model::Position::Absolute {
                monitor_identity: identity, x: snapped_x, y: snapped_y,
            });
        }
    }
    if let Ok(mut history) = tx.history.lock() { history.push_undo(prev); }
    Ok(())
}

#[tauri::command]
pub fn edit_resize_widget(app: AppHandle, instance: String, width: u32) -> Result<(), String> {
    let Some(tx) = app.try_state::<EditTransaction>() else { return Ok(()) };
    if !tx.active.load(Ordering::Relaxed) { return Ok(()); }
    let Some(config_state) = app.try_state::<Arc<RwLock<Config>>>() else { return Ok(()) };
    let prev = config_state.read().ok().map(|guard| guard.clone());
    let Some(prev) = prev else { return Ok(()) };
    if let Ok(mut guard) = config_state.write() {
        if let Some(section) = guard.sections.iter_mut().find(|s| s.instance == instance) {
            section.width = width;
        }
    }
    if let Ok(mut history) = tx.history.lock() { history.push_undo(prev); }
    Ok(())
}

#[tauri::command]
pub fn bring_all_onto_visible(app: AppHandle) -> Result<(), String> {
    let Some(tx) = app.try_state::<EditTransaction>() else { return Ok(()) };
    if !tx.active.load(Ordering::Relaxed) { return Ok(()); }
    let Some(config_state) = app.try_state::<Arc<RwLock<Config>>>() else { return Ok(()) };
    let prev = config_state.read().ok().map(|guard| guard.clone());
    let Some(prev) = prev else { return Ok(()) };
    let count = app.webview_windows().values().next()
        .and_then(|w| w.available_monitors().ok())
        .map(|m| m.len()).unwrap_or(1);
    if let Ok(mut guard) = config_state.write() {
        for section in &mut guard.sections {
            let idx = section.monitor.min(count.saturating_sub(1));
            section.position = Some(crate::position_model::Position::Absolute {
                monitor_identity: idx.to_string(), x: 24, y: 24 + section.y,
            });
        }
    }
    if let Ok(mut history) = tx.history.lock() { history.push_undo(prev); }
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        if let Some(config) = handle.try_state::<Arc<RwLock<Config>>>().and_then(|s| s.read().ok().map(|g| g.clone())) {
            crate::window::reconcile(&handle, &config);
        }
    });
    Ok(())
}
