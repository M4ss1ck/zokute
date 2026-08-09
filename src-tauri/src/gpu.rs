use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct GpuReading {
    pub id: String,
    pub name: String,
    pub gpu_busy_percent: Option<f32>,
    pub vram_used_bytes: Option<u64>,
    pub vram_total_bytes: Option<u64>,
    pub temperature_celsius: Option<f32>,
    pub power_watts: Option<f32>,
    pub fan_percent: Option<f32>,
}

pub fn discover_amdgpu() -> Vec<GpuReading> {
    crate::gpu_linux::discover_amdgpu()
}
