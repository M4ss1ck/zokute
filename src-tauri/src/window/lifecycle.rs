use std::collections::HashMap;

use crate::{config::{Config, SectionConfig}, window::{flags, position, LABELS}};
use tauri::{AppHandle, Manager, WebviewWindow};

#[derive(Debug)]
pub enum WindowAction<'a> {
    Create(&'a SectionConfig),
    Close,
    Update(&'a SectionConfig),
}

pub fn desired_action<'a>(label: &'a str, section: Option<&'a SectionConfig>, exists: bool) -> Option<WindowAction<'a>> {
    if !LABELS.contains(&label) {
        return None;
    }
    let Some(section) = section else {
        return if exists { Some(WindowAction::Close) } else { None };
    };
    match (section.enabled, exists) {
        (true, false) => Some(WindowAction::Create(section)),
        (true, true) => Some(WindowAction::Update(section)),
        (false, true) => Some(WindowAction::Close),
        (false, false) => None,
    }
}

pub fn reconcile(app: &AppHandle, config: &Config) {
    let windows: HashMap<String, WebviewWindow> = app.webview_windows();
    for label in LABELS {
        let section = config.section(label);
        let existing = windows.get(label);
        match desired_action(label, section, existing.is_some()) {
            Some(WindowAction::Create(section)) => match flags::create(app, label) {
                Ok(window) => {
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
}

pub fn toggle_visibility(app: &AppHandle) {
    let windows: HashMap<String, WebviewWindow> = app.webview_windows();
    let any_visible = LABELS
        .iter()
        .filter_map(|label| windows.get(*label))
        .any(|window| window.is_visible().unwrap_or(false));
    for label in LABELS {
        let Some(window) = windows.get(label) else { continue };
        let result = if any_visible { window.hide() } else { window.show() };
        if let Err(error) = result {
            eprintln!("{label}: {error}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{desired_action, WindowAction};
    use crate::config::SectionConfig;

    fn section(id: &str, enabled: bool) -> SectionConfig {
        SectionConfig { id: id.into(), enabled, monitor: 0, x: 0, y: 0, width: 360, scale: 1.0 }
    }

    #[test]
    fn unknown_labels_are_ignored() {
        assert!(desired_action("bogus", Some(&section("bogus", true)), false).is_none());
        assert!(desired_action("bogus", None, false).is_none());
    }

    #[test]
    fn enabled_known_labels_create_or_update() {
        match desired_action("system", Some(&section("system", true)), false) {
            Some(WindowAction::Create(_)) => {}
            other => panic!("unexpected: {other:?}"),
        }
        match desired_action("system", Some(&section("system", true)), true) {
            Some(WindowAction::Update(_)) => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn disabled_known_labels_close_when_present() {
        assert!(matches!(desired_action("cpu", Some(&section("cpu", false)), true), Some(WindowAction::Close)));
        assert!(desired_action("cpu", Some(&section("cpu", false)), false).is_none());
    }

    #[test]
    fn missing_known_sections_close_existing_windows() {
        assert!(matches!(desired_action("memory", None, true), Some(WindowAction::Close)));
        assert!(desired_action("memory", None, false).is_none());
    }
}
