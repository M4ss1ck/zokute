use crate::snap::{Guide, GuideOrientation};
#[cfg(target_os = "linux")]
use gtk::prelude::*;
use serde::Serialize;
use tauri::{AppHandle, Manager, PhysicalPosition, PhysicalSize, WebviewUrl, WebviewWindowBuilder};

pub const LABEL_PREFIX: &str = "guides-";

pub fn is_overlay(label: &str) -> bool {
    label.starts_with(LABEL_PREFIX)
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct GuideLine {
    pub orientation: &'static str,
    pub position: f64,
}

/// Guides are computed in global physical pixels; each overlay draws them in
/// CSS pixels local to its own monitor.
pub fn to_local(guide: Guide, display_origin: (i32, i32), scale_factor: f64) -> GuideLine {
    let (origin_x, origin_y) = display_origin;
    let (orientation, origin) = match guide.orientation {
        GuideOrientation::Vertical => ("Vertical", origin_x),
        GuideOrientation::Horizontal => ("Horizontal", origin_y),
    };
    GuideLine { orientation, position: f64::from(guide.position - origin) / scale_factor }
}

pub fn open(app: &AppHandle) {
    let Some(anchor) = app.get_webview_window(crate::settings::LABEL) else { return };
    let Ok(monitors) = anchor.available_monitors() else { return };
    for (index, monitor) in monitors.iter().enumerate() {
        let label = format!("{LABEL_PREFIX}{index}");
        if app.get_webview_window(&label).is_some() {
            continue;
        }
        let built = WebviewWindowBuilder::new(app, &label, WebviewUrl::App("index.html".into()))
            .visible(false)
            .transparent(true)
            .decorations(false)
            .resizable(false)
            .shadow(false)
            .skip_taskbar(true)
            .focused(false)
            .build();
        match built {
            Ok(overlay) => {
                #[cfg(target_os = "linux")]
                if let Ok(gtk_window) = overlay.gtk_window() {
                    gtk_window.realize();
                }
                let origin = monitor.position();
                let size = monitor.size();
                let _ = overlay.set_position(PhysicalPosition::new(origin.x, origin.y));
                let _ = overlay.set_size(PhysicalSize::new(size.width, size.height));
                let _ = overlay.set_ignore_cursor_events(true);
                let _ = overlay.set_always_on_top(true);
                let _ = overlay.show();
            }
            Err(error) => eprintln!("{label}: {error}"),
        }
    }
}

pub fn close(app: &AppHandle) {
    for (label, window) in app.webview_windows() {
        if is_overlay(&label) {
            let _ = window.close();
        }
    }
}

#[cfg(test)]
#[path = "guides_overlay_tests.rs"]
mod tests;
