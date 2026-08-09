use crate::snap::Rect;
use crate::window::position::monitor_index_for;
use tauri::{AppHandle, Manager};

pub fn widget_rects(app: &AppHandle) -> Vec<(String, Rect)> {
    app.webview_windows()
        .into_iter()
        .filter(|(label, _)| !crate::window::is_control_window(label))
        .filter_map(|(label, window)| {
            let position = window.outer_position().ok()?;
            let size = window.inner_size().ok()?;
            let rect = Rect {
                x: position.x,
                y: position.y,
                width: size.width as i32,
                height: size.height as i32,
            };
            Some((label, rect))
        })
        .collect()
}

pub fn display_for(app: &AppHandle, x: i32, y: i32) -> Option<(usize, Rect, f64)> {
    let anchor = app.get_webview_window(crate::settings::LABEL)?;
    let monitors = anchor.available_monitors().ok()?;
    let bounds: Vec<(i32, i32, u32, u32)> = monitors
        .iter()
        .map(|monitor| {
            let origin = monitor.position();
            let size = monitor.size();
            (origin.x, origin.y, size.width, size.height)
        })
        .collect();
    let index = monitor_index_for(x, y, &bounds)?;
    let monitor = monitors.get(index)?;
    let origin = monitor.position();
    let size = monitor.size();
    let rect = Rect {
        x: origin.x,
        y: origin.y,
        width: size.width as i32,
        height: size.height as i32,
    };
    Some((index, rect, monitor.scale_factor()))
}
