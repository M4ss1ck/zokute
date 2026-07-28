use crate::{
    config::{Config, Profile},
    disk_io::{DiskIoBaselines, DiskIoReading},
    gpu::GpuReading, network::NetworkReading,
    sensors::SensorReading, system_info,
};
use serde::Serialize;
use std::time::Instant;

#[derive(Clone, Serialize)]
pub struct Stats {
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub disks: Vec<DiskStat>,
    pub disk_io: Vec<DiskIoReading>,
    pub network: Vec<NetworkReading>,
    pub gpu: Vec<GpuReading>,
    pub sensors: Vec<SensorReading>,
    pub load_avg: Option<LoadAvg>,
    pub cpu_temperature: Option<SensorReading>,
    pub uptime: u64, pub now_ms: u64,
    pub system_fields: Vec<system_info::SystemField>,
    pub config: Config, pub profile: Profile, pub edit_mode: bool,
    pub fullscreen: bool,
}

#[derive(Clone, Serialize)]
pub struct LoadAvg {
    pub one: f64, pub five: f64, pub fifteen: f64,
}

#[derive(Clone, Serialize)]
pub struct DiskStat {
    pub id: String, pub name: String, pub mount: String,
    pub used_bytes: u64, pub total_bytes: u64,
    pub temperature_celsius: Option<f32>,
    pub display_label: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct CpuStats {
    pub aggregate_percent: f32, pub core_percents: Vec<f32>,
}

#[derive(Clone, Serialize)]
pub struct MemoryStats {
    pub used_bytes: u64, pub total_bytes: u64,
    pub swap_used_bytes: u64, pub swap_total_bytes: u64,
}

pub(crate) fn loadavg_read() -> LoadAvg {
    let content = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let parts: Vec<&str> = content.split_whitespace().collect();
    LoadAvg {
        one: parts.first().and_then(|s| s.parse().ok()).unwrap_or(0.0),
        five: parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        fifteen: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
    }
}

pub(crate) fn disk_io_read(baselines: &mut DiskIoBaselines, elapsed: f64) -> Vec<DiskIoReading> {
    let now = Instant::now();
    let stats = crate::disk_io::read_diskstats();
    stats.into_iter().filter_map(|(name, read_sectors, write_sectors)| {
        let sector_size = crate::disk_io::read_sector_size(&name);
        let entry = baselines.entry(name.clone()).or_insert(crate::disk_io::DiskIoBaseline {
            read_sectors, write_sectors, sector_size, timestamp: now,
        });
        let read_bps = if read_sectors >= entry.read_sectors {
            ((read_sectors - entry.read_sectors) * sector_size) as f64 / elapsed.max(0.001)
        } else { 0.0 };
        let write_bps = if write_sectors >= entry.write_sectors {
            ((write_sectors - entry.write_sectors) * sector_size) as f64 / elapsed.max(0.001)
        } else { 0.0 };
        entry.read_sectors = read_sectors;
        entry.write_sectors = write_sectors;
        entry.timestamp = now;
        Some(DiskIoReading { id: name, read_bytes_per_second: read_bps as u64, write_bytes_per_second: write_bps as u64 })
    }).collect()
}
