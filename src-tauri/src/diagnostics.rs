use crate::{
    config::Config, config::Profile, paths, platform,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct DiagnosticsReport {
    pub version: &'static str,
    pub platform: PlatformReport,
    pub monitors: Vec<MonitorReport>,
    pub config: Option<ConfigReport>,
    pub profile: Option<ProfileReport>,
    pub sections: Vec<SectionReport>,
    pub audio: AudioReport,
    pub collector: CollectorReport,
    pub fullscreen: FullscreenReport,
    pub errors: Vec<String>,
}

#[derive(Serialize)]
pub struct PlatformReport {
    pub session: String,
    pub os: String,
    pub arch: String,
    pub hostname: Option<String>,
}

#[derive(Serialize)]
pub struct MonitorReport {
    pub connector: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub geometry: Option<String>,
}

#[derive(Serialize)]
pub struct ConfigReport {
    pub path: String,
    pub schema_version: u32,
    pub theme: String,
    pub opacity: f64,
    pub density: String,
}

#[derive(Serialize)]
pub struct ProfileReport {
    pub name: String,
    pub schema_version: u32,
    pub section_count: usize,
    pub system_field_count: usize,
}

#[derive(Serialize)]
pub struct SectionReport {
    pub id: String,
    pub instance: String,
    pub enabled: bool,
    pub is_plugin: bool,
    pub plugin_id: Option<String>,
    pub width: u32,
    pub scale: f64,
}

#[derive(Serialize)]
pub struct AudioReport {
    pub active: bool,
}

/// How fullscreen state is being tracked. `DegradedPoll` means EWMH event
/// subscription failed and the capped fallback is running instead.
#[derive(Clone, Copy, PartialEq)]
pub enum DetectionMode {
    Events,
    DegradedPoll,
}

#[derive(Serialize)]
pub struct FullscreenReport {
    pub behavior: String,
    pub dim_opacity: f64,
    pub detection: &'static str,
    pub degraded: bool,
}

#[derive(Serialize)]
pub struct CollectorReport {
    pub interval_ms: u64,
}

pub fn collect(config: Option<&Config>, profile: Option<&Profile>, detection: DetectionMode) -> DiagnosticsReport {
    let session = platform::detect();
    DiagnosticsReport {
        version: "0.1.0",
        platform: PlatformReport {
            session: format!("{session:?}"),
            os: std::env::consts::OS.to_string(),
            arch: std::env::consts::ARCH.to_string(),
            hostname: std::fs::read_to_string("/proc/sys/kernel/hostname").ok()
                .map(|s| s.trim().to_string()),
        },
        monitors: collect_monitors(),
        config: config.map(|c| ConfigReport {
            path: paths::config_path().to_string_lossy().to_string(),
            schema_version: c.schema_version,
            theme: c.theme.clone(),
            opacity: c.opacity,
            density: c.density.clone(),
        }),
        profile: profile.map(|p| ProfileReport {
            name: "active".into(),
            schema_version: p.profile_schema_version,
            section_count: p.sections.len(),
            system_field_count: p.system_fields.len(),
        }),
        sections: profile.map(|p| p.sections.iter().map(|s| SectionReport {
            id: s.id.clone(),
            instance: s.instance.clone(),
            enabled: s.enabled,
            is_plugin: s.is_plugin(),
            plugin_id: s.plugin_id.clone(),
            width: s.width,
            scale: s.scale,
        }).collect()).unwrap_or_default(),
        audio: AudioReport { active: profile.is_some_and(|p| {
            p.sections.iter().any(|s| s.enabled && (s.id == "spectrum" || s.id == "ring"))
        })},
        fullscreen: FullscreenReport {
            behavior: profile.map_or_else(|| "show".into(), |p| p.fullscreen.behavior.clone()),
            dim_opacity: profile.map_or(0.25, |p| p.fullscreen.dim_opacity),
            detection: if detection == DetectionMode::Events { "events" } else { "poll" },
            degraded: detection == DetectionMode::DegradedPoll,
        },
        collector: CollectorReport { interval_ms: profile.map_or(1000, |p| p.collect_interval_ms) },
        errors: Vec::new(),
    }
}

fn collect_monitors() -> Vec<MonitorReport> {
    let monitors = crate::monitor_linux::enumerate_monitors();
    monitors.iter().map(|(connector, id)| MonitorReport {
        connector: connector.clone(),
        manufacturer: id.manufacturer.clone(),
        model: id.model.clone(),
        geometry: id.last_geometry.map(|g| format!("{}x{}@{},{}", g.width, g.height, g.x, g.y)),
    }).collect()
}
