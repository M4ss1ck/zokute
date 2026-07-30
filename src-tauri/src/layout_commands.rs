use crate::{
    config::{Config, Profile}, config_write,
    edit_mode::{self, EditTransaction, prepare_window, restore_after_edit},
    edit_touched,
};
use std::sync::{atomic::Ordering, Arc, RwLock};
use tauri::{AppHandle, Manager};

fn open_layout_editor(app: &AppHandle) {
    if app.get_webview_window("layout-editor").is_some() { return; }
    // GTK only builds windows on the main thread, and the tray and the CLI both
    // reach this from other threads. Building in place fails there, and the
    // error used to be discarded, so Edit Layout silently did nothing.
    let handle = app.clone();
    let open = move || {
        if handle.get_webview_window("layout-editor").is_some() { return; }
        let built = tauri::WebviewWindowBuilder::new(&handle, "layout-editor", tauri::WebviewUrl::App("index.html".into()))
            .title("Edit Layout")
            .inner_size(360.0, 560.0)
            .resizable(true)
            .decorations(true)
            .transparent(false)
            .center()
            .build();
        match built {
            Ok(window) => {
                let close_handle = handle.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        if edit_mode::is_active(&close_handle) {
                            api.prevent_close();
                            let _ = cancel_layout(close_handle.clone());
                        }
                    }
                });
            }
            Err(error) => eprintln!("layout-editor: {error}"),
        }
    };
    if let Err(error) = app.run_on_main_thread(open) {
        eprintln!("layout-editor: {error}");
    }
}

#[tauri::command]
pub fn enter_edit_layout(app: AppHandle) {
    let Some(profile_state) = app.try_state::<Arc<RwLock<Profile>>>() else { return };
    let profile = profile_state.read().ok().map(|guard| guard.clone());
    let Some(profile) = profile else { return };
    if let Some(tx) = app.try_state::<EditTransaction>() {
        if let Ok(mut snapshot) = tx.profile.lock() { *snapshot = Some(profile.clone()); }
        tx.active.store(true, Ordering::Relaxed);
    }
    if let Some(state) = app.try_state::<edit_mode::EditMode>() {
        state.0.store(true, Ordering::Relaxed);
    }
    edit_touched::clear(&app);
    open_layout_editor(&app);
    for label in app.webview_windows().keys()
        .filter(|label| !crate::window::is_control_window(label))
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
        .and_then(|tx| tx.profile.lock().ok().and_then(|mut s| s.take()));
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
        if let Ok(mut s) = tx.profile.lock() { *s = None; }
    }
    if let Some(state) = app.try_state::<edit_mode::EditMode>() {
        state.0.store(false, Ordering::Relaxed);
    }
    if let Some(window) = app.get_webview_window("layout-editor") { let _ = window.close(); }
    restore_after_edit(app);
}
