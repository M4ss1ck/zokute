use crate::{
    config::{self, Config, Profile}, disk::DiskReading as RawDiskReading,
    disk_io::{DiskIoBaselines, DiskIoReading},
    gpu::GpuReading, network::{NetworkBaselines, NetworkReading},
    sensors::SensorReading, system_info,
};
use serde::Serialize;
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use sysinfo::{CpuRefreshKind, Disks, System};
use tauri::{AppHandle, Emitter};

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

pub async fn run(app: AppHandle, config_state: Arc<RwLock<Config>>, profile_state: Arc<RwLock<Profile>>) {
    let static_system_fields = tauri::async_runtime::spawn_blocking(|| {
        crate::fastfetch::collect().unwrap_or_else(system_info::fallback)
    }).await.unwrap_or_else(|_| system_info::fallback());
    let catalog_order = system_info::catalog_order(&static_system_fields, &config::DEFAULT_SYSTEM_FIELDS);
    let mut system = System::new();
    let mut disks = Disks::new_with_refreshed_list();
    let mut network_baselines: NetworkBaselines = HashMap::new();
    let mut disk_io_baselines: DiskIoBaselines = HashMap::new();
    let start = tokio::time::Instant::now() + crate::tick::until_next_second(crate::tick::now_ms());
    let mut interval = tokio::time::interval_at(start, Duration::from_secs(1));
    system.refresh_cpu_list(CpuRefreshKind::everything());
    system.refresh_cpu_usage();
    interval.tick().await;
    let mut last = Instant::now();
    loop {
        interval.tick().await;
        let elapsed = last.elapsed().as_secs_f64().max(0.001);
        last = Instant::now();
        system.refresh_cpu_usage();
        system.refresh_memory();
        disks.refresh(true);
        let config = config_state.read().expect("config").clone();
        let profile = profile_state.read().expect("profile").clone();
        let cpu = CpuStats {
            aggregate_percent: system.global_cpu_usage(),
            core_percents: system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        };
        let memory = MemoryStats {
            used_bytes: system.used_memory(), total_bytes: system.total_memory(),
            swap_used_bytes: system.used_swap(), swap_total_bytes: system.total_swap(),
        };
        let disks_vec = crate::disk::discover(&disks).into_iter()
            .filter_map(|disk: RawDiskReading| {
                let pref = profile.disk_preference(&disk.id)?;
                pref.enabled.then(|| DiskStat {
                    id: disk.id, name: disk.name, mount: disk.mount,
                    used_bytes: disk.used_bytes, total_bytes: disk.total_bytes,
                    temperature_celsius: disk.temperature_celsius,
                    display_label: pref.label.clone(),
                })
            }).collect();
        let network = crate::network::read_and_diff(&mut network_baselines, elapsed);
        let cpu_temp = crate::sensors::select_cpu(&crate::sensors_linux::collect_sensors());
        let load_avg = loadavg_read();
        let sensors = crate::sensors_linux::collect_sensors();
        let gpu = crate::gpu::discover_amdgpu();
        let disk_io = disk_io_read(&mut disk_io_baselines, elapsed);
        let uptime = System::uptime();
        let stats = Stats {
            cpu, memory, disks: disks_vec, disk_io, network, gpu, sensors,
            load_avg: Some(load_avg),
            cpu_temperature: cpu_temp,
            uptime, now_ms: crate::tick::now_ms(),
            system_fields: system_info::filter_and_order(&static_system_fields, &catalog_order, uptime),
            config, profile, edit_mode: crate::edit_mode::is_active(&app),
        };
        let _ = app.emit("stats", &stats);
    }
}

fn loadavg_read() -> LoadAvg {
    let content = std::fs::read_to_string("/proc/loadavg").unwrap_or_default();
    let parts: Vec<&str> = content.split_whitespace().collect();
    LoadAvg {
        one: parts.first().and_then(|s| s.parse().ok()).unwrap_or(0.0),
        five: parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0.0),
        fifteen: parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0.0),
    }
}

fn disk_io_read(baselines: &mut DiskIoBaselines, elapsed: f64) -> Vec<DiskIoReading> {
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
