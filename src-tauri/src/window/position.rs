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

pub fn logical_to_physical(value: i32, scale_factor: f64) -> i32 {
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
    let height = section
        .height
        .map(f64::from)
        .unwrap_or_else(|| physical_to_logical(inner_size.height, window_scale_factor));
    if section.id == "clock" {
        eprintln!("[dbg] position::apply {}: set {}x{}", section.instance, section.width, height);
    }
    window.set_size(LogicalSize::new(section.width as f64, height))
}

#[derive(Debug, PartialEq)]
pub struct Placement {
    pub monitor: usize,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub fn monitor_index_for(x: i32, y: i32, bounds: &[(i32, i32, u32, u32)]) -> Option<usize> {
    bounds.iter().position(|(origin_x, origin_y, width, height)| {
        x >= *origin_x
            && y >= *origin_y
            && x < origin_x + *width as i32
            && y < origin_y + *height as i32
    })
}

pub fn placement_from(
    monitor: usize,
    origin: (i32, i32),
    monitor_scale_factor: f64,
    window_origin: (i32, i32),
    inner_width: u32,
    inner_height: u32,
    window_scale_factor: f64,
) -> Placement {
    Placement {
        monitor,
        x: (f64::from(window_origin.0 - origin.0) / monitor_scale_factor).round() as i32,
        y: (f64::from(window_origin.1 - origin.1) / monitor_scale_factor).round() as i32,
        width: (f64::from(inner_width) / window_scale_factor).round() as u32,
        height: (f64::from(inner_height) / window_scale_factor).round() as u32,
    }
}

pub fn capture(window: &WebviewWindow) -> Option<Placement> {
    let monitors = window.available_monitors().ok()?;
    let bounds: Vec<(i32, i32, u32, u32)> = monitors
        .iter()
        .map(|monitor| {
            let position = monitor.position();
            let size = monitor.size();
            (position.x, position.y, size.width, size.height)
        })
        .collect();
    let position = window.outer_position().ok()?;
    let index = monitor_index_for(position.x, position.y, &bounds).unwrap_or(0);
    let monitor = monitors.get(index)?;
    let inner = window.inner_size().ok()?;
    Some(placement_from(
        index,
        (monitor.position().x, monitor.position().y),
        monitor.scale_factor(),
        (position.x, position.y),
        inner.width,
        inner.height,
        window.scale_factor().ok()?,
    ))
}

#[cfg(test)]
mod tests {
    use super::{logical_to_physical, physical_to_logical, monitor_index_for, placement_from};

    #[test]
    fn converts_logical_offsets_to_physical_positions() {
        assert_eq!(logical_to_physical(11, 1.5), 17);
        assert_eq!(logical_to_physical(10, 1.5), 15);
    }

    #[test]
    fn converts_physical_heights_to_logical_sizes() {
        assert_eq!(physical_to_logical(180, 1.5), 120.0);
    }

    #[test]
    fn finds_the_monitor_containing_the_window_origin() {
        let monitors = [(0, 0, 1920, 1080), (1920, 0, 2560, 1440)];
        assert_eq!(monitor_index_for(10, 10, &monitors), Some(0));
        assert_eq!(monitor_index_for(2000, 700, &monitors), Some(1));
        assert_eq!(monitor_index_for(-5, 0, &monitors), None);
        assert_eq!(monitor_index_for(1920, 0, &monitors), Some(1));
    }

    #[test]
    fn converts_a_physical_window_origin_back_to_monitor_relative_logical() {
        let placement = placement_from(1, (1920, 0), 1.5, (1965, 30), 540, 300, 1.5);
        assert_eq!(placement.monitor, 1);
        assert_eq!(placement.x, 30);
        assert_eq!(placement.y, 20);
        assert_eq!(placement.width, 360);
        assert_eq!(placement.height, 200);
    }

    #[test]
    fn round_trips_a_placement_through_apply_s_conversions() {
        let placement = placement_from(0, (0, 0), 1.5, (logical_to_physical(24, 1.5), 0), 540, 300, 1.5);
        assert_eq!(placement.x, 24);
    }
}
