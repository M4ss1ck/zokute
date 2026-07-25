use crate::config::SectionConfig;
use tauri::{LogicalSize, PhysicalPosition, WebviewWindow};

fn monitor(window: &WebviewWindow, section: &SectionConfig) -> Option<tauri::Monitor> {
    let monitors = window.available_monitors().ok()?;
    monitors
        .get(section.monitor)
        .cloned()
        .or_else(|| window.primary_monitor().ok().flatten())
        .or_else(|| monitors.first().cloned())
}

pub fn apply(window: &WebviewWindow, section: &SectionConfig) -> tauri::Result<()> {
    let Some(monitor) = monitor(window, section) else { return Ok(()); };
    let position = monitor.position();
    window.set_position(PhysicalPosition::new(position.x + section.x, position.y + section.y))?;
    let height = window.inner_size()?.height;
    window.set_size(LogicalSize::new(section.width as f64, height as f64))
}
