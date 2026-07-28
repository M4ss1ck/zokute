use crate::config::SectionConfig;
use crate::monitor::{resolve_monitor, MonitorCatalog, MonitorGeometry};
use crate::position_model::{Anchor, Position};
use tauri::{LogicalSize, PhysicalPosition, WebviewWindow};
use tauri::Manager;

fn monitor_by_index(window: &WebviewWindow, catalog: &MonitorCatalog, section: &SectionConfig) -> Option<tauri::Monitor> {
    let monitors = window.available_monitors().ok()?;
    let effective = section.effective_position();
    let identity = effective.monitor_identity().to_string();
    let current: Vec<(String, crate::monitor::MonitorIdentity)> = monitors.iter().enumerate().map(|(i, m)| {
        let pos = m.position();
        let size = m.size();
        let id = crate::monitor::MonitorIdentity {
            connector: format!("monitor-{i}"),
            edid_hash: String::new(),
            manufacturer: None,
            model: None,
            serial: None,
            name: format!("Monitor {i}"),
            last_geometry: Some(MonitorGeometry {
                x: pos.x, y: pos.y,
                width: size.width, height: size.height,
                scale_factor: m.scale_factor(),
            }),
            was_primary: i == 0,
        };
        (format!("monitor-{i}"), id)
    }).collect();
    let idx = resolve_monitor(&identity, catalog, &current);
    idx.and_then(|i| monitors.get(i).cloned())
}

pub fn compute_position(window: &WebviewWindow, section: &SectionConfig) -> Option<(i32, i32)> {
    let catalog_state = window.app_handle().try_state::<std::sync::Arc<std::sync::RwLock<crate::config::Config>>>();
    let catalog: MonitorCatalog = catalog_state
        .and_then(|state| state.read().ok().map(|guard| guard.monitor_catalog.clone()))
        .unwrap_or_default();
    let monitor = monitor_by_index(window, &catalog, section)?;
    let position = monitor.position();
    let work_size = monitor.size();
    let monitor_scale_factor = monitor.scale_factor();
    let pos = section.effective_position();
    let (wx, wy) = match &pos {
        Position::Absolute { x, y, .. } => {
            (position.x + logical_to_physical(*x, monitor_scale_factor),
             position.y + logical_to_physical(*y, monitor_scale_factor))
        }
        Position::Anchored { anchor, offset_x, offset_y, .. } => {
            let (ax, ay) = anchor_point(*anchor, work_size.width, work_size.height);
            (position.x + logical_to_physical(ax + offset_x, monitor_scale_factor),
             position.y + logical_to_physical(ay + offset_y, monitor_scale_factor))
        }
    };
    Some((wx, wy))
}

fn anchor_point(anchor: Anchor, width: u32, height: u32) -> (i32, i32) {
    anchor.anchor_point(width, height)
}

pub fn logical_to_physical(value: i32, scale_factor: f64) -> i32 {
    (f64::from(value) * scale_factor).round() as i32
}

fn physical_to_logical(value: u32, scale_factor: f64) -> f64 {
    f64::from(value) / scale_factor
}

pub fn apply(window: &WebviewWindow, section: &SectionConfig) -> tauri::Result<()> {
    let Some((wx, wy)) = compute_position(window, section) else { return Ok(()); };
    let inner_size = window.inner_size()?;
    let window_scale_factor = window.scale_factor()?;
    window.set_position(PhysicalPosition::new(wx, wy))?;
    let height = section
        .height
        .map(f64::from)
        .unwrap_or_else(|| physical_to_logical(inner_size.height, window_scale_factor));
    window.set_size(LogicalSize::new(section.width as f64, height))
}

#[derive(Debug, PartialEq)]
pub struct Placement {
    pub monitor_identity: String,
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
    let sf = monitor.scale_factor();
    let ox = monitor.position().x;
    let oy = monitor.position().y;
    Some(Placement {
        monitor_identity: index.to_string(),
        x: (f64::from(position.x - ox) / sf).round() as i32,
        y: (f64::from(position.y - oy) / sf).round() as i32,
        width: (f64::from(inner.width) / sf).round() as u32,
        height: (f64::from(inner.height) / sf).round() as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::{logical_to_physical, monitor_index_for};

    #[test]
    fn converts_logical_offsets_to_physical_positions() {
        assert_eq!(logical_to_physical(11, 1.5), 17);
        assert_eq!(logical_to_physical(10, 1.5), 15);
    }

    #[test]
    fn finds_the_monitor_containing_the_window_origin() {
        let monitors = [(0, 0, 1920, 1080), (1920, 0, 2560, 1440)];
        assert_eq!(monitor_index_for(10, 10, &monitors), Some(0));
        assert_eq!(monitor_index_for(2000, 700, &monitors), Some(1));
        assert_eq!(monitor_index_for(-5, 0, &monitors), None);
    }
}
