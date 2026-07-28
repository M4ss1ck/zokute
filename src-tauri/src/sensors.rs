use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct SensorReading {
    pub id: String,
    pub label: String,
    pub celsius: f32,
    pub kind: SensorKind,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub enum SensorKind {
    Cpu,
    Gpu,
    Storage,
}

pub fn discover_all() -> Vec<SensorReading> {
    crate::sensors_linux::collect_sensors()
}

pub fn select_cpu(sensors: &[SensorReading]) -> Option<SensorReading> {
    let cpu = sensors.iter().filter(|s| matches!(s.kind, SensorKind::Cpu));
    cpu.clone().min_by(|a, b| a.celsius.partial_cmp(&b.celsius).unwrap_or(std::cmp::Ordering::Equal)).cloned()
}

pub fn select_storage(sensors: &[SensorReading], block_names: &[String]) -> Option<SensorReading> {
    sensors.iter().find(|s| {
        matches!(s.kind, SensorKind::Storage) && block_names.iter().any(|b| s.id.contains(b))
    }).cloned()
}
