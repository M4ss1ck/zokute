use crate::{
    config::{Config, Profile},
    config_write,
    edit_geometry, edit_mode::{self, EditMode, EditTransaction, prepare_window, restore_after_edit},
    edit_touched,
};
use std::sync::{atomic::Ordering, Arc, RwLock};
use tauri::{AppHandle, Manager};

/// Returns (merge_geometry_first, next_flag). Turning Arrange off captures live
/// geometry before the windows are restored to their normal interaction mode.
pub fn arrange_transition(enabled: bool) -> (bool, bool) {
    (!enabled, enabled)
}

pub(crate) fn snapshot(app: &AppHandle) {
    let config = app.try_state::<Arc<RwLock<Config>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone()));
    let profile = app.try_state::<Arc<RwLock<Profile>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone()));
    if let Some(tx) = app.try_state::<EditTransaction>() {
        if let Ok(mut slot) = tx.config.lock() { *slot = config; }
        if let Ok(mut slot) = tx.profile.lock() { *slot = profile; }
    }
    edit_touched::clear(app);
}

#[tauri::command]
pub fn begin_settings_session(app: AppHandle) -> bool {
    snapshot(&app);
    let armed = app.try_state::<EditTransaction>()
        .map(|tx| {
            tx.active.store(true, Ordering::Relaxed);
            tx.arm_arrange.swap(false, Ordering::Relaxed)
        })
        .unwrap_or(false);
    if armed {
        let _ = set_arrange(app, true);
    }
    armed
}

#[tauri::command]
pub fn save_settings(app: AppHandle) -> Result<(), String> {
    let config = app.try_state::<Arc<RwLock<Config>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone())).ok_or("no config")?;
    let profile = app.try_state::<Arc<RwLock<Profile>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone())).ok_or("no profile")?;
    let merged = edit_geometry::merge_live_geometry(&app, profile);
    config_write::apply(&app, config, merged).map_err(|error| error.to_string())?;
    // The dialog stays open on Save, so the session continues against a fresh
    // baseline rather than ending.
    snapshot(&app);
    Ok(())
}

#[tauri::command]
pub fn discard_settings(app: AppHandle) -> Result<(), String> {
    let restored = app.try_state::<EditTransaction>().and_then(|tx| {
        let config = tx.config.lock().ok().and_then(|slot| slot.clone());
        let profile = tx.profile.lock().ok().and_then(|slot| slot.clone());
        config.zip(profile)
    });
    let Some((config, profile)) = restored else { return Ok(()) };
    if let Some(state) = app.try_state::<Arc<RwLock<Config>>>() {
        if let Ok(mut guard) = state.write() { *guard = config; }
    }
    if let Some(state) = app.try_state::<Arc<RwLock<Profile>>>() {
        if let Ok(mut guard) = state.write() { *guard = profile.clone(); }
    }
    crate::audio::sync(&app, &profile);
    edit_touched::clear(&app);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || crate::window::reconcile(&handle, &profile));
    Ok(())
}

#[tauri::command]
pub fn set_arrange(app: AppHandle, enabled: bool) -> Result<Profile, String> {
    let (merge_first, next) = arrange_transition(enabled);
    let mut profile = app.try_state::<Arc<RwLock<Profile>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone())).ok_or("no profile")?;
    if merge_first {
        profile = edit_geometry::merge_live_geometry(&app, profile);
        if let Some(state) = app.try_state::<Arc<RwLock<Profile>>>() {
            if let Ok(mut guard) = state.write() { *guard = profile.clone(); }
        }
    }
    if let Some(state) = app.try_state::<EditMode>() {
        state.0.store(next, Ordering::Relaxed);
    }
    if next {
        for label in widget_labels(&app) {
            let Some(window) = app.get_webview_window(&label) else { continue };
            let _ = window.show();
            prepare_window(&window);
        }
    } else {
        restore_after_edit(&app);
    }
    Ok(profile)
}

/// Opens the dialog with Arrange already on, for the tray item and `zokute edit`.
#[tauri::command]
pub fn open_settings_arranging(app: AppHandle) {
    if let Some(tx) = app.try_state::<EditTransaction>() {
        tx.arm_arrange.store(true, Ordering::Relaxed);
    }
    if edit_mode::session_active(&app) {
        // Dialog already open: arm is never read again, so flip directly.
        let _ = set_arrange(app.clone(), true);
    }
    crate::settings::open(&app);
}

/// Called when the dialog window is destroyed. Any unsaved draft is already
/// gone from memory; this only restores the widgets and clears the flags.
pub fn end(app: &AppHandle) {
    let _ = set_arrange(app.clone(), false);
    if let Some(tx) = app.try_state::<EditTransaction>() {
        tx.active.store(false, Ordering::Relaxed);
        if let Ok(mut slot) = tx.config.lock() { *slot = None; }
        if let Ok(mut slot) = tx.profile.lock() { *slot = None; }
    }
    edit_touched::clear(app);
}

fn widget_labels(app: &AppHandle) -> Vec<String> {
    app.webview_windows().keys()
        .filter(|label| !crate::window::is_control_window(label))
        .cloned().collect()
}
