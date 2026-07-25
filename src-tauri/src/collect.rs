use crate::{config::Config, disk::DiskReading as RawDiskReading, temperature::Temperature};
use crate::temperature;
use serde::Serialize;
use std::{
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use sysinfo::{Components, CpuRefreshKind, Disks, Networks, System};
use tauri::{AppHandle, Emitter};

#[derive(Clone, Serialize)]
pub struct Stats {
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub disks: Vec<DiskStat>,
    pub network: NetworkStats,
    pub cpu_temperature: Option<Temperature>,
    pub gpu_temperatures: Vec<Temperature>,
    pub uptime: u64,
    pub hostname: String,
    pub config: Config,
}

#[derive(Clone, Serialize)]
pub struct DiskStat {
    pub id: String,
    pub name: String,
    pub mount: String,
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub temperature_celsius: Option<f32>,
    pub display_label: Option<String>,
}

#[derive(Clone, Serialize)]
pub struct CpuStats {
    pub aggregate_percent: f32,
    pub core_percents: Vec<f32>,
}

#[derive(Clone, Serialize)]
pub struct MemoryStats {
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub swap_used_bytes: u64,
    pub swap_total_bytes: u64,
}
#[derive(Clone, Serialize)]
pub struct NetworkStats {
    pub down_bytes_per_second: u64,
    pub up_bytes_per_second: u64,
}

pub async fn run(app: AppHandle, config_state: Arc<RwLock<Config>>) {
    let mut system = System::new();
    let mut disks = Disks::new_with_refreshed_list();
    let mut networks = Networks::new_with_refreshed_list();
    let mut components = Components::new_with_refreshed_list();
    let mut interval = tokio::time::interval(Duration::from_secs(1));
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
        networks.refresh(true);
        components.refresh(true);
        let config = config_state.read().expect("config").clone();
        let cpu = CpuStats {
            aggregate_percent: system.global_cpu_usage(),
            core_percents: system.cpus().iter().map(|cpu| cpu.cpu_usage()).collect(),
        };
        let memory = MemoryStats {
            used_bytes: system.used_memory(),
            total_bytes: system.total_memory(),
            swap_used_bytes: system.used_swap(),
            swap_total_bytes: system.total_swap(),
        };
        let disks = crate::disk::discover(&disks)
            .into_iter()
            .filter_map(|disk: RawDiskReading| {
                let preference = config.disk_preference(&disk.id)?;
                preference.enabled.then(|| DiskStat {
                    id: disk.id,
                    name: disk.name,
                    mount: disk.mount,
                    used_bytes: disk.used_bytes,
                    total_bytes: disk.total_bytes,
                    temperature_celsius: disk.temperature_celsius,
                    display_label: preference.label.clone(),
                })
            })
            .collect();
        let (received, transmitted) = networks.iter().fold((0, 0), |(down, up), (_, data)| {
            (down + data.received(), up + data.transmitted())
        });
        let network = NetworkStats {
            down_bytes_per_second: (received as f64 / elapsed) as u64,
            up_bytes_per_second: (transmitted as f64 / elapsed) as u64,
        };
        let (cpu_temperature, gpu_temperatures) = temperature::select(&components);
        let stats = Stats {
            cpu,
            memory,
            disks,
            network,
            cpu_temperature,
            gpu_temperatures,
            uptime: System::uptime(),
            hostname: System::host_name().unwrap_or_default(),
            config,
        };
        let _ = app.emit("stats", &stats);
    }
}
