use std::collections::HashMap;

use crate::{config::{Config, SectionConfig}, edit_mode, window::{flags, position, LABELS}};
use tauri::{AppHandle, Manager, WebviewWindow};

#[derive(Debug)]
pub enum WindowAction<'a> {
    Create(&'a SectionConfig),
    Close,
    Update(&'a SectionConfig),
}

pub fn desired_action(section: &SectionConfig, exists: bool) -> Option<WindowAction<'_>> {
    if !LABELS.contains(&section.id.as_str()) {
        return None;
    }
    match (section.enabled, exists) {
        (true, false) => Some(WindowAction::Create(section)),
        (true, true) => Some(WindowAction::Update(section)),
        (false, true) => Some(WindowAction::Close),
        (false, false) => None,
    }
}

pub fn reconcile(app: &AppHandle, config: &Config) {
    let windows: HashMap<String, WebviewWindow> = app.webview_windows();
    for section in &config.sections {
        let label = &section.instance;
        let existing = windows.get(label);
        match desired_action(section, existing.is_some()) {
            Some(WindowAction::Create(section)) => match flags::create(app, label) {
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
        if label == "settings" || config.section(label).is_some() {
            continue;
        }
        if let Err(error) = window.close() {
            eprintln!("{label}: {error}");
        }
    }
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
}

#[cfg(test)]
mod tests {
    use super::{desired_action, WindowAction};
    use crate::{config::SectionConfig, window::LABELS};
    use std::collections::BTreeMap;

    fn section(id: &str, enabled: bool) -> SectionConfig {
        SectionConfig { id: id.into(), instance: format!("{id}-1"), enabled, show_header: true, position: None, monitor: 0, x: 0, y: 0, width: 360, height: None, scale: 1.0, color_mode: None, color_a: None, color_b: None, gradient_direction: None, clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None, date_weekday: true, date_format: None, date_color: None, plugin_id: None, plugin_interval: 30, plugin_config: None, extra: BTreeMap::new() }
    }

    #[test]
    fn unknown_labels_are_ignored() {
        assert!(desired_action(&section("bogus", true), false).is_none());
    }

    #[test]
    fn enabled_known_labels_create_or_update() {
        match desired_action(&section("system", true), false) {
            Some(WindowAction::Create(_)) => {}
            other => panic!("unexpected: {other:?}"),
        }
        match desired_action(&section("system", true), true) {
            Some(WindowAction::Update(_)) => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn disabled_known_labels_close_when_present() {
        assert!(matches!(desired_action(&section("cpu", false), true), Some(WindowAction::Close)));
        assert!(desired_action(&section("cpu", false), false).is_none());
    }

    #[test]
    fn capabilities_cover_dynamic_instance_labels() {
        let capability = include_str!("../../capabilities/default.json");
        for label in LABELS {
            assert!(capability.contains(&format!("\"{label}-*\"")));
        }
    }
}
