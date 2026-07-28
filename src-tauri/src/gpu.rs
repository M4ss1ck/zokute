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

pub fn select<'a>(gpus: &'a [GpuReading], selection: &str) -> Vec<&'a GpuReading> {
    match selection {
        "auto" => select_auto(gpus),
        _ => gpus.iter().filter(|g| g.id == selection).collect(),
    }
}

fn select_auto<'a>(gpus: &'a [GpuReading]) -> Vec<&'a GpuReading> {
    let discrete = gpus.iter().filter(|g| g.name.contains("DGPU") || g.gpu_busy_percent.is_some());
    let measured = discrete.clone().filter(|g| g.gpu_busy_percent.unwrap_or(0.0) > 0.0);
    let first: Vec<&GpuReading> = measured.chain(discrete).chain(gpus.iter()).collect();
    first.into_iter().take(1).collect()
}
