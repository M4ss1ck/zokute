use crate::config::Config;
use crate::edit_mode::is_active;
use crate::edit_touched;
use crate::window::position;
use std::sync::{Arc, RwLock};
use tauri::{AppHandle, Manager, LogicalSize};

pub fn apply_placement(mut config: Config, instance: &str, placement: position::Placement) -> Config {
    if let Some(section) = config.sections.iter_mut().find(|section| section.instance == instance) {
        section.position = Some(crate::position_model::Position::Absolute {
            monitor_identity: placement.monitor_identity.clone(),
            x: placement.x,
            y: placement.y,
        });
        section.width = placement.width;
        section.height = Some(placement.height);
    }
    config
}

pub fn reset_box(config: &mut Config, instance: &str, width: u32) -> bool {
    let Some(section) = config.sections.iter_mut().find(|section| section.instance == instance) else {
        return false;
    };
    section.width = width;
    section.height = None;
    true
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
        if !edit_touched::was_grabbed(app, &label) {
            continue;
        }
        let Some(window) = app.get_webview_window(&label) else { continue };
        let Some(placement) = position::capture(&window) else { continue };
        merged = apply_placement(merged, &label, placement);
    }
    merged
}

#[tauri::command]
pub fn resize_widget(app: AppHandle, id: String, width: u32) {
    let Some(state) = app.try_state::<Arc<RwLock<Config>>>() else { return };
    if !state.write().ok().is_some_and(|mut guard| reset_box(&mut guard, &id, width)) {
        return;
    }
    edit_touched::mark(&app, &id);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || {
        let Some(window) = handle.get_webview_window(&id) else { return };
        if let Err(error) = window.set_size(LogicalSize::new(f64::from(width), 1.0)) {
            eprintln!("{id}: {error}");
        }
    });
}
