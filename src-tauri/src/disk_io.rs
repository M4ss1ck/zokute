use serde::Serialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Serialize)]
pub struct DiskIoReading {
    pub id: String,
    pub read_bytes_per_second: u64,
    pub write_bytes_per_second: u64,
}

#[derive(Debug, Clone)]
pub struct DiskIoBaseline {
    pub read_sectors: u64,
    pub write_sectors: u64,
    pub sector_size: u64,
    pub timestamp: std::time::Instant,
}

pub type DiskIoBaselines = HashMap<String, DiskIoBaseline>;

pub fn discover_block_devices() -> Vec<String> {
    let mut devices = Vec::new();
    let Ok(entries) = std::fs::read_dir("/sys/class/block") else { return devices };
    for entry in entries.flatten() {
        let name = entry.file_name().to_string_lossy().to_string();
        let dev_path = entry.path();
        let has_queue = dev_path.join("queue").exists();
        if has_queue && !name.starts_with("loop") && !name.starts_with("ram") && !name.starts_with("dm-") {
            devices.push(name);
        }
    }
    devices
}

pub fn read_sector_size(device: &str) -> u64 {
    let path = Path::new("/sys/class/block").join(device).join("queue/logical_block_size");
    std::fs::read_to_string(&path).ok().and_then(|s| s.trim().parse::<u64>().ok()).unwrap_or(512)
}

pub fn read_diskstats() -> Vec<(String, u64, u64)> {
    let content = std::fs::read_to_string("/proc/diskstats").unwrap_or_default();
    content.lines().filter_map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 10 { return None; }
        let name = parts[2].to_string();
        if name.starts_with("loop") || name.starts_with("ram") || name.starts_with("dm-") { return None; }
        let read_sectors: u64 = parts[5].parse().ok()?;
        let write_sectors: u64 = parts[9].parse().ok()?;
        Some((name, read_sectors, write_sectors))
    }).collect()
}
