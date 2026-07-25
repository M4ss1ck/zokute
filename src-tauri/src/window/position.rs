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

fn logical_to_physical(value: i32, scale_factor: f64) -> i32 {
    (f64::from(value) * scale_factor).round() as i32
}

fn physical_to_logical(value: u32, scale_factor: f64) -> f64 {
    f64::from(value) / scale_factor
}

pub fn apply(window: &WebviewWindow, section: &SectionConfig) -> tauri::Result<()> {
    let Some(monitor) = monitor(window, section) else { return Ok(()); };
    let inner_size = window.inner_size()?;
    let window_scale_factor = window.scale_factor()?;
    let position = monitor.position();
    let monitor_scale_factor = monitor.scale_factor();
    window.set_position(PhysicalPosition::new(
        position.x + logical_to_physical(section.x, monitor_scale_factor),
        position.y + logical_to_physical(section.y, monitor_scale_factor),
    ))?;
    let height = physical_to_logical(inner_size.height, window_scale_factor);
    window.set_size(LogicalSize::new(section.width as f64, height))
}

#[cfg(test)]
mod tests {
    use super::{logical_to_physical, physical_to_logical};

    #[test]
    fn converts_logical_offsets_to_physical_positions() {
        assert_eq!(logical_to_physical(11, 1.5), 17);
        assert_eq!(logical_to_physical(10, 1.5), 15);
    }

    #[test]
    fn converts_physical_heights_to_logical_sizes() {
        assert_eq!(physical_to_logical(180, 1.5), 120.0);
    }
}
