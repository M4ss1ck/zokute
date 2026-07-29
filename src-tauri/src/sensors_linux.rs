use crate::sensors::{SensorKind, SensorReading};
use std::path::Path;

fn is_temp_input(name: &str) -> bool {
    let Some(core) = name.strip_suffix("_input") else { return false; };
    let Some(digits) = core.strip_prefix("temp") else { return false; };
    !digits.is_empty() && digits.chars().all(|ch| ch.is_ascii_digit())
}

fn read_temp(path: &Path, name: &str) -> Option<f32> {
    if !is_temp_input(name) { return None; }
    let value = std::fs::read_to_string(path).ok()?;
    let millicelsius: f32 = value.trim().parse().ok()?;
    if !millicelsius.is_finite() { return None; }
    Some(millicelsius / 1000.0)
}

fn device_kind(device_path: &Path) -> Option<SensorKind> {
    if let Ok(target) = std::fs::read_link(device_path) {
        let path_str = target.to_string_lossy();
        if path_str.contains("drm") || path_str.contains("gpu") { return Some(SensorKind::Gpu); }
    }
    let name_path = device_path.join("device");
    if name_path.exists() {
        if let Ok(target) = std::fs::read_link(&name_path) {
            let path_str = target.to_string_lossy();
            if path_str.contains("drm") || path_str.contains("gpu") { return Some(SensorKind::Gpu); }
            if path_str.contains("nvme") || path_str.contains("sd") { return Some(SensorKind::Storage); }
        }
    }
    Some(SensorKind::Cpu)
}

pub(crate) fn kind_for_name(name: &str) -> Option<SensorKind> {
    match name.trim() {
        "amdgpu" | "radeon" | "nouveau" => Some(SensorKind::Gpu),
        "nvme" | "drivetemp" => Some(SensorKind::Storage),
        _ => None,
    }
}

pub fn collect_sensors() -> Vec<SensorReading> {
    let mut readings = Vec::new();
    let root = Path::new("/sys/class/hwmon");
    let Ok(entries) = std::fs::read_dir(root) else { return readings };
    for entry in entries.flatten() {
        let hwmon_path = entry.path();
        let name = std::fs::read_to_string(hwmon_path.join("name"))
            .ok().map(|name| name.trim().to_string());
        let kind = name.as_deref().and_then(kind_for_name)
            .or_else(|| device_kind(&hwmon_path)).unwrap_or(SensorKind::Cpu);
        if let Ok(read_dir) = std::fs::read_dir(&hwmon_path) {
            for sensor_entry in read_dir.flatten() {
                let name_str = sensor_entry.file_name().to_string_lossy().to_string();
                if !is_temp_input(&name_str) { continue; }
                let Some(celsius) = read_temp(&sensor_entry.path(), &name_str) else { continue };
                let sensor_name = name.clone().unwrap_or_default();
                readings.push(SensorReading {
                    id: sensor_name.clone(), label: sensor_name, celsius, kind: kind,
                });
            }
        }
    }
    readings.sort_by(|a, b| b.celsius.partial_cmp(&a.celsius).unwrap_or(std::cmp::Ordering::Equal));
    readings
}
