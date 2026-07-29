use std::collections::HashMap;

use crate::{config::{SectionConfig, Profile}, edit_mode, visibility::VisibilityState, window::{flags, is_control_window, position, LABELS}};
use tauri::{AppHandle, Manager, WebviewWindow};

#[derive(Debug)]
pub enum WindowAction<'a> {
    Create(&'a SectionConfig),
    Close,
    Update(&'a SectionConfig),
}

pub fn desired_action(section: &SectionConfig, exists: bool) -> Option<WindowAction<'_>> {
    if !LABELS.contains(&section.id.as_str()) && section.id != "panel" && !section.is_plugin() {
        return None;
    }
    match (section.enabled, exists) {
        (true, false) => Some(WindowAction::Create(section)),
        (true, true) => Some(WindowAction::Update(section)),
        (false, true) => Some(WindowAction::Close),
        (false, false) => None,
    }
}

pub fn reconcile(app: &AppHandle, profile: &Profile) {
    let windows: HashMap<String, WebviewWindow> = app.webview_windows();
    for section in &profile.sections {
        let label = &section.instance;
        let existing = windows.get(label);
        match desired_action(section, existing.is_some()) {
            Some(WindowAction::Create(section)) => match flags::create(app, label, section.interactive) {
                Ok(window) => {
                    if edit_mode::is_active(app) {
                        edit_mode::prepare_window(&window);
                    }
                    if let Err(error) = position::apply(&window, section) {
                        eprintln!("{label}: {error}");
                    }
                    if let Err(error) = window.show() {
                        eprintln!("{label}: {error}");
                    }
                }
                Err(error) => eprintln!("{label}: {error}"),
            },
            Some(WindowAction::Update(section)) => {
                if let Some(window) = existing {
                    if let Err(error) = position::apply(window, section) {
                        eprintln!("{label}: {error}");
                    }
                }
            }
            Some(WindowAction::Close) => {
                if let Some(window) = existing {
                    if let Err(error) = window.close() {
                        eprintln!("{label}: {error}");
                    }
                }
            }
            None => {}
        }
    }
    for (label, window) in &windows {
        if is_control_window(label) || profile.section(label).is_some() {
            continue;
        }
        if let Err(error) = window.close() {
            eprintln!("{label}: {error}");
        }
    }
    set_demand(app, crate::visibility::demand(profile, false));
}

/// The one place the collector's demand signal is written, so hiding, showing,
/// and reconciling cannot disagree about whether work is needed.
fn set_demand(app: &AppHandle, wanted: bool) {
    if let Some(state) = app.try_state::<VisibilityState>() { state.set_visible(wanted); }
}

pub fn show_all(app: &AppHandle) {
    for (label, window) in &app.webview_windows() {
        if label != "settings" { let _ = window.show(); }
    }
    set_demand(app, true);
}

pub fn hide_all(app: &AppHandle) {
    for (label, window) in &app.webview_windows() {
        if label != "settings" { let _ = window.hide(); }
    }
    set_demand(app, false);
}

pub fn toggle_visibility(app: &AppHandle) {
    let windows: HashMap<String, WebviewWindow> = app.webview_windows();
    let widget_windows = windows.iter().filter(|(label, _)| label.as_str() != "settings").collect::<Vec<_>>();
    let any_visible = widget_windows
        .iter()
        .map(|(_, window)| window)
        .any(|window| window.is_visible().unwrap_or(false));
    for (label, window) in widget_windows {
        let result = if any_visible { window.hide() } else { window.show() };
        if let Err(error) = result {
            eprintln!("{label}: {error}");
        }
    }
    set_demand(app, !any_visible);
}
