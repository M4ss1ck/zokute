use std::collections::HashMap;

use crate::{config::{Config, SectionConfig}, window::{flags, position, LABELS}};
use tauri::{AppHandle, Manager, WebviewWindow};

#[derive(Debug)]
pub enum WindowAction<'a> {
    Create(&'a SectionConfig),
    Close,
    Update(&'a SectionConfig),
}

pub fn desired_action(section: Option<&SectionConfig>, exists: bool) -> Option<WindowAction<'_>> {
    let section = section?;
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
    for label in LABELS {
        let section = config.section(label);
        let existing = windows.get(label);
        match desired_action(section, existing.is_some()) {
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

#[cfg(test)]
mod tests {
    use super::{desired_action, WindowAction};
    use crate::config::SectionConfig;

    fn section(id: &str, enabled: bool) -> SectionConfig {
        SectionConfig { id: id.into(), enabled, monitor: 0, x: 0, y: 0, width: 360 }
    }

    #[test]
    fn unknown_labels_are_ignored() {
        assert!(desired_action(Some(&section("bogus", true)), false).is_none());
        assert!(desired_action(None, false).is_none());
    }

    #[test]
    fn enabled_known_labels_create_or_update() {
        match desired_action(Some(&section("system", true)), false) {
            Some(WindowAction::Create(_)) => {}
            other => panic!("unexpected: {other:?}"),
        }
        match desired_action(Some(&section("system", true)), true) {
            Some(WindowAction::Update(_)) => {}
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn disabled_known_labels_close_when_present() {
        assert!(matches!(desired_action(Some(&section("cpu", false)), true), Some(WindowAction::Close)));
        assert!(desired_action(Some(&section("cpu", false)), false).is_none());
    }
}
