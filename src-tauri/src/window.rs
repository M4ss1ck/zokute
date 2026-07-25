use gtk::prelude::*;
use tauri::{PhysicalPosition, PhysicalSize, WebviewWindow};

use crate::config::Config;

const LEGACY_MAIN_HEIGHT: u32 = 760;

pub fn configure(window: &WebviewWindow) {
    let _ = window.set_always_on_bottom(true);
    let _ = window.set_ignore_cursor_events(true);
    #[cfg(target_os = "linux")]
    if let Ok(gtk_window) = window.gtk_window() {
        gtk_window.set_type_hint(gtk::gdk::WindowTypeHint::Desktop);
        gtk_window.stick();
    }
    // Cinnamon can hide normal sticky windows during Show Desktop, so Desktop hint is the fallback.
}

pub fn apply_config(window: &WebviewWindow, config: &Config) {
    if let Some(section) = config.known_sections().first() {
        let monitor = window.available_monitors().ok().and_then(|monitors| {
            monitors
                .get(section.monitor)
                .cloned()
                .or_else(|| monitors.first().cloned())
        });
        if let Some(monitor) = monitor {
        let position = monitor.position();
        let x = position.x + section.x;
        let y = position.y + section.y;
        let _ = window.set_position(PhysicalPosition::new(x, y));
        if let Ok(size) = window.outer_size() {
            // Keep the pre-Task 8 dashboard tall enough to avoid clipping.
            let height = size.height.max(LEGACY_MAIN_HEIGHT);
            let _ = window.set_size(PhysicalSize::new(section.width, height));
        }
        }
    }
}
