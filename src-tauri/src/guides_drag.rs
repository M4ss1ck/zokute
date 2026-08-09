use crate::guides_geometry::{display_for, widget_rects};
use crate::guides_overlay::{self, LABEL_PREFIX};
use crate::snap::{snap_position, Rect, TOLERANCE};
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, PhysicalPosition};

const SETTLE: Duration = Duration::from_millis(180);

#[derive(Default)]
pub struct Drag {
    pub label: Option<String>,
    pub target: Option<(i32, i32)>,
    pub generation: u64,
    pub settling: bool,
}

#[derive(Default)]
pub struct DragState(pub Mutex<Drag>);

/// Only widgets sharing the dragged widget's display are alignment targets, and
/// a widget can never align to itself.
pub fn other_rects(all: &[(String, Rect)], dragged: &str, display: Rect) -> Vec<Rect> {
    all.iter()
        .filter(|(label, _)| label.as_str() != dragged)
        .map(|(_, rect)| *rect)
        .filter(|rect| {
            rect.x >= display.x
                && rect.y >= display.y
                && rect.x < display.x + display.width
                && rect.y < display.y + display.height
        })
        .collect()
}

pub fn on_moved(app: &AppHandle, label: &str, position: PhysicalPosition<i32>) {
    if !crate::edit_mode::is_active(app) {
        return;
    }
    let Some((index, display, scale)) = display_for(app, position.x, position.y) else { return };
    let all = widget_rects(app);
    let Some((_, dragged)) = all.iter().find(|(name, _)| name == label) else { return };
    let widget = Rect { x: position.x, y: position.y, ..*dragged };
    let others = other_rects(&all, label, display);
    let result = snap_position(widget, display, &others, scale_tolerance(scale));

    let lines: Vec<_> = result
        .guides
        .iter()
        .map(|guide| guides_overlay::to_local(*guide, (display.x, display.y), scale))
        .collect();
    emit_guides(app, index, lines);
    arm_settle(app, label, (result.x, result.y));
}

fn scale_tolerance(scale: f64) -> i32 {
    (f64::from(TOLERANCE) * scale).round() as i32
}

fn emit_guides(app: &AppHandle, index: usize, lines: Vec<guides_overlay::GuideLine>) {
    let active = format!("{LABEL_PREFIX}{index}");
    let windows = app.webview_windows();
    for label in windows.keys() {
        if !guides_overlay::is_overlay(label) {
            continue;
        }
        let payload = if label == &active { lines.clone() } else { Vec::new() };
        let _ = app.emit_to(label.as_str(), "alignment-guides", payload);
    }
}

/// The window manager owns the window during the drag, so the corrected
/// position is written once the moves stop rather than on every event.
fn arm_settle(app: &AppHandle, label: &str, target: (i32, i32)) {
    let Some(state) = app.try_state::<DragState>() else { return };
    let start = {
        let Ok(mut drag) = state.0.lock() else { return };
        drag.label = Some(label.to_string());
        drag.target = Some(target);
        drag.generation = drag.generation.wrapping_add(1);
        let start = !drag.settling;
        drag.settling = true;
        start
    };
    if !start {
        return;
    }
    let handle = app.clone();
    std::thread::spawn(move || settle(handle));
}

fn generation(app: &AppHandle) -> Option<u64> {
    let state = app.try_state::<DragState>()?;
    let drag = state.0.lock().ok()?;
    Some(drag.generation)
}

/// Waits for one full quiet interval after the last move, then lands the widget.
fn settle(app: AppHandle) {
    let Some(mut seen) = generation(&app) else { return };
    loop {
        std::thread::sleep(SETTLE);
        let Some(state) = app.try_state::<DragState>() else { return };
        let Ok(mut drag) = state.0.lock() else { return };
        if drag.generation != seen {
            seen = drag.generation;
            continue;
        }
        let landing = drag.label.take().zip(drag.target.take());
        drag.settling = false;
        drop(drag);
        let Some((label, (x, y))) = landing else { return };
        let handle = app.clone();
        let _ = app.run_on_main_thread(move || {
            if let Some(window) = handle.get_webview_window(&label) {
                let _ = window.set_position(PhysicalPosition::new(x, y));
            }
            let windows = handle.webview_windows();
            for overlay in windows.keys() {
                if guides_overlay::is_overlay(overlay) {
                    let empty = Vec::<guides_overlay::GuideLine>::new();
                    let _ = handle.emit_to(overlay.as_str(), "alignment-guides", empty);
                }
            }
        });
        return;
    }
}

#[cfg(test)]
#[path = "guides_drag_tests.rs"]
mod tests;
