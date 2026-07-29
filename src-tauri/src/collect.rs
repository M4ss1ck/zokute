use crate::{
    config::{Config, Profile}, disk::DiskReading as RawDiskReading,
    disk_io::DiskIoBaselines,
    network::NetworkBaselines, system_info,
    stats_types::{
        CpuStats, DiskStat, MemoryStats, Stats,
        disk_io_read, loadavg_read,
    },
};
use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};
use sysinfo::{CpuRefreshKind, Disks, System};
use tauri::{AppHandle, Emitter, Manager};

fn read_interval(profile_state: &Arc<RwLock<Profile>>) -> u64 {
    profile_state.read().ok().map(|p| p.collect_interval_ms.clamp(250, 60000)).unwrap_or(1000)
}

pub async fn run(app: AppHandle, config_state: Arc<RwLock<Config>>, profile_state: Arc<RwLock<Profile>>) {
    let static_system_fields = tauri::async_runtime::spawn_blocking(|| {
        crate::fastfetch::collect().unwrap_or_else(system_info::fallback)
    }).await.unwrap_or_else(|_| system_info::fallback());
    let catalog_order = system_info::catalog_order(&static_system_fields, &crate::config::DEFAULT_SYSTEM_FIELDS);
    let mut system = System::new();
    let mut disks = Disks::new_with_refreshed_list();
    let mut network_baselines: NetworkBaselines = HashMap::new();
    let mut disk_io_baselines: DiskIoBaselines = HashMap::new();
    let mut cadence = crate::cadence::Cadence::new(Instant::now());
    system.refresh_cpu_list(CpuRefreshKind::everything());
    system.refresh_cpu_usage();
    let start = tokio::time::Instant::now() + Duration::from_millis(50);
    let mut interval = tokio::time::interval_at(start, Duration::from_millis(read_interval(&profile_state)));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    interval.tick().await;
    loop {
        interval.tick().await;
        let interval_ms = read_interval(&profile_state);
        let new_interval = Duration::from_millis(interval_ms);
        if interval.period() != new_interval {
            interval = tokio::time::interval_at(tokio::time::Instant::now() + new_interval, new_interval);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
        }
        let visible = app.try_state::<crate::visibility::VisibilityState>()
            .map(|s| s.is_visible())
            .unwrap_or(true);
        let elapsed = match cadence.step(Instant::now(), visible, interval_ms) {
            crate::cadence::Tick::Idle => continue,
            crate::cadence::Tick::Resume => {
                network_baselines.clear();
                disk_io_baselines.clear();
                continue;
            }
            crate::cadence::Tick::Emit { elapsed } => elapsed,
        };
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
            fullscreen: app.try_state::<crate::fullscreen::FullscreenState>()
                .map(|s| s.is_fullscreen()).unwrap_or(false),
        };
        let _ = app.emit("stats", &stats);
    }
}
