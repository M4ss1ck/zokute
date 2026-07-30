use crate::{paths};
use serde::{Deserialize, Serialize};
use std::{
    fs,
    sync::{Arc, Mutex},
};
use tauri::{AppHandle, Emitter, Manager, WebviewUrl, WebviewWindowBuilder, WindowEvent};

pub const LABEL: &str = "settings";
const MIN_WIDTH: u32 = 640;
const MIN_HEIGHT: u32 = 480;

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Serialize)]
struct SettingsSize {
    width: u32,
    height: u32,
}

impl Default for SettingsSize {
    fn default() -> Self {
        Self { width: 760, height: 680 }
    }
}

fn clamp(size: SettingsSize) -> SettingsSize {
    SettingsSize {
        width: size.width.max(MIN_WIDTH),
        height: size.height.max(MIN_HEIGHT),
    }
}

fn load_size() -> SettingsSize {
    let path = paths::settings_window_path();
    let size = fs::read_to_string(path)
        .ok()
        .and_then(|source| toml::from_str::<SettingsSize>(&source).ok())
        .unwrap_or_default();
    clamp(size)
}

fn save_size(size: SettingsSize) {
    let path = paths::settings_window_path();
    if let Ok(source) = toml::to_string(&size) {
        let _ = crate::atomic_file::write(&path, &source);
    }
}

pub fn open(app: &AppHandle) {
    if let Some(existing) = app.get_webview_window(LABEL) {
        let _ = existing.set_focus();
        return;
    }
    let size = load_size();
    let built = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::App("index.html".into()))
        .title("Zokute Settings")
        .inner_size(f64::from(size.width), f64::from(size.height))
        .min_inner_size(f64::from(MIN_WIDTH), f64::from(MIN_HEIGHT))
        .resizable(true)
        .decorations(true)
        .transparent(false)
        .center()
        .build();
    match built {
        Ok(window) => {
            let tracked = window.clone();
            let close_handle = app.clone();
            let latest = Arc::new(Mutex::new(size));
            window.on_window_event(move |event| match event {
                WindowEvent::Resized(physical) => {
                    let scale = tracked.scale_factor().unwrap_or(1.0);
                    if let Ok(mut current) = latest.lock() {
                        current.width = (f64::from(physical.width) / scale).round() as u32;
                        current.height = (f64::from(physical.height) / scale).round() as u32;
                    }
                }
                WindowEvent::CloseRequested { api, .. } => {
                    if crate::edit_mode::session_active(&close_handle) {
                        // The draft's dirty state lives in the dialog, not here,
                        // so the frontend decides: prompt, or destroy the window.
                        api.prevent_close();
                        let _ = tracked.emit("settings-close-requested", ());
                    }
                }
                WindowEvent::Destroyed => {
                    if let Ok(current) = latest.lock() {
                        save_size(*current);
                    }
                    crate::settings_session::end(&close_handle);
                }
                _ => {}
            });
        }
        Err(error) => eprintln!("{LABEL}: {error}"),
    }
}

#[cfg(test)]
mod tests {
    use super::{clamp, SettingsSize, MIN_HEIGHT, MIN_WIDTH};

    #[test]
    fn settings_size_round_trips() {
        let source = toml::to_string(&SettingsSize { width: MIN_WIDTH, height: MIN_HEIGHT }).unwrap();
        let parsed: SettingsSize = toml::from_str(&source).unwrap();
        assert_eq!((parsed.width, parsed.height), (MIN_WIDTH, MIN_HEIGHT));
    }

    #[test]
    fn the_default_size_fits_the_sidebar_and_the_pane() {
        let size = SettingsSize::default();
        assert_eq!((size.width, size.height), (760, 680));
    }

    #[test]
    fn a_size_saved_before_the_sidebar_existed_is_clamped_up() {
        // The old default was 420x640, which cannot hold a 200px sidebar.
        assert_eq!(clamp(SettingsSize { width: 420, height: 640 }), SettingsSize { width: 640, height: 640 });
    }

    #[test]
    fn a_size_larger_than_the_minimum_is_left_alone() {
        assert_eq!(clamp(SettingsSize { width: 900, height: 800 }), SettingsSize { width: 900, height: 800 });
    }
}
