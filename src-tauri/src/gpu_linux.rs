use crate::gpu::GpuReading;
use std::path::Path;

fn read_sysfs<T: std::str::FromStr>(path: &Path) -> Option<T> {
    let content = std::fs::read_to_string(path).ok()?;
    content.trim().parse().ok()
}

pub fn discover_amdgpu() -> Vec<GpuReading> {
    let mut gpus = Vec::new();
    let drm_dir = Path::new("/sys/class/drm");
    let Ok(entries) = std::fs::read_dir(drm_dir) else { return gpus };
    for entry in entries.flatten() {
        let name = entry.file_name();
        let name_str = name.to_string_lossy();
        if !name_str.starts_with("card") || !name_str.ends_with("-") {
            continue;
        }
        let card_dir = entry.path();
        let device_dir = card_dir.join("device");
        let uevent = std::fs::read_to_string(device_dir.join("uevent")).unwrap_or_default();
        if !uevent.contains("DRIVER=amdgpu") { continue; }

        let pci_id = std::fs::read_to_string(device_dir.join("modalias"))
            .unwrap_or_default()
            .trim()
            .to_string();

        let product_name = std::fs::read_to_string(device_dir.join("product_name")).ok()
            .map(|s| s.trim().to_string());

        let gpu_busy = read_sysfs::<f32>(&device_dir.join("gpu_busy_percent"));
        let mem_info_vram_total = read_sysfs::<u64>(&device_dir.join("mem_info_vram_total"));
        let mem_info_vram_used = read_sysfs::<u64>(&device_dir.join("mem_info_vram_used"));

        let mut temp_celsius = None;
        let mut power_watts = None;
        let mut fan_percent = None;
        let hwmon_dir = device_dir.join("hwmon");
        if let Ok(hwmon_entries) = std::fs::read_dir(&hwmon_dir) {
            for hwmon in hwmon_entries.flatten() {
                let hpath = hwmon.path();
                if read_sysfs::<f32>(&hpath.join("temp1_input")).is_some() {
                    temp_celsius = read_sysfs::<f32>(&hpath.join("temp1_input"))
                        .map(|v| v / 1000.0);
                }
                if hpath.join("power1_average").exists() {
                    power_watts = read_sysfs::<f32>(&hpath.join("power1_average"))
                        .map(|v| v / 1_000_000.0);
                }
                if hpath.join("fan1_input").exists() {
                    fan_percent = read_sysfs::<f32>(&hpath.join("fan1_input"));
                }
            }
        }

        gpus.push(GpuReading {
            id: pci_id,
            name: product_name.unwrap_or_else(|| format!("AMD GPU ({})", name_str)),
            gpu_busy_percent: gpu_busy,
            vram_used_bytes: mem_info_vram_used,
            vram_total_bytes: mem_info_vram_total,
            temperature_celsius: temp_celsius,
            power_watts,
            fan_percent,
        });
    }
    gpus
}
