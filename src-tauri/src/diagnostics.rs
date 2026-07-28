use crate::{
    config, config::Config, config::Profile,
    config_error::ConfigError, config_validate, paths, platform,
};
use serde::Serialize;
use std::collections::BTreeMap;

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

#[derive(Serialize)]
pub struct CollectorReport {
    pub interval_ms: u64,
}

pub fn collect(config: Option<&Config>, profile: Option<&Profile>) -> DiagnosticsReport {
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
        audio: AudioReport { active: profile.map_or(false, |p| {
            p.sections.iter().any(|s| s.enabled && (s.id == "spectrum" || s.id == "ring"))
        })},
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

pub fn validate_config(path: Option<&str>) -> Result<String, String> {
    let config_path = path.map_or_else(paths::config_path, |p| std::path::PathBuf::from(p));
    let source = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("cannot read {}: {}", config_path.display(), e))?;
    let parsed = config::parse(&source)
        .map_err(|e| format!("parse error: {e}"))?;
    config_validate::check(&parsed)
        .map_err(|e| format!("validation error: {e}"))?;
    Ok(format!("{}: valid (schema v{})", config_path.display(), parsed.schema_version))
}

pub fn show_config(redact_plugin_config: bool, config: &Config, profile: &Profile) -> String {
    let mut lines = Vec::new();

    lines.push("=== Global config ===".into());
    lines.push(format!("  schema_version = {}", config.schema_version));
    lines.push(format!("  active_profile = \"{}\"", config.active_profile));
    lines.push(format!("  theme = \"{}\"", config.theme));
    lines.push(format!("  opacity = {}", config.opacity));
    lines.push(format!("  text_opacity = {}", config.text_opacity));
    lines.push(format!("  text_color = \"{}\"", config.text_color));
    lines.push(format!("  density = \"{}\"", config.density));
    lines.push(format!("  font_scale = {}", config.font_scale));
    lines.push(format!("  byte_format = \"{}\"", config.byte_format));
    lines.push(format!("  temperature_unit = \"{}\"", config.temperature_unit));

    lines.push("".into());
    lines.push("=== Profile ===".into());
    lines.push(format!("  schema_version = {}", profile.profile_schema_version));
    lines.push(format!("  sections = {}", profile.sections.len()));
    lines.push(format!("  system_fields = {}", profile.system_fields.len()));
    lines.push(format!("  show_cpu_cores = {}", profile.show_cpu_cores));
    lines.push(format!("  disks = {}", profile.disks.len()));
    lines.push(format!("  collect_interval_ms = {}", profile.collect_interval_ms));

    lines.push("".into());
    lines.push("=== Sections ===".into());
    for section in &profile.sections {
        lines.push(format!("  [{}.{}] enabled={} plugin={}",
            section.id, section.instance, section.enabled, section.is_plugin()));
        if redact_plugin_config && section.plugin_config.is_some() {
            lines.push("    plugin_config = <redacted>".into());
        }
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::Config;
    use crate::config::Profile;
    use std::collections::BTreeMap;

    fn sample_config() -> Config {
        Config {
            schema_version: 3, active_profile: "default".into(),
            opacity: 0.9, text_opacity: 1.0, text_color: "#000".into(),
            graph_color: None, icon_color: None, show_background: true,
            theme: "light".into(), accent_color: None, density: "compact".into(),
            font_scale: 1.0, sans_font: None, mono_font: None,
            byte_format: "binary".into(), temperature_unit: "celsius".into(),
            locale: None, extra: BTreeMap::new(),
        }
    }

    fn sample_profile() -> Profile {
        Profile {
            profile_schema_version: 1, sections: vec![], system_fields: vec![],
            show_cpu_cores: true, disks: vec![], collect_interval_ms: 1000,
            monitor_catalog: BTreeMap::new(), extra: BTreeMap::new(),
        }
    }

    #[test]
    fn show_config_contains_expected_sections() {
        let output = show_config(false, &sample_config(), &sample_profile());
        assert!(output.contains("Global config"));
        assert!(output.contains("Profile"));
        assert!(output.contains("Sections"));
    }

    #[test]
    fn show_config_redacts_plugin_config() {
        let config = sample_config();
        let mut profile = sample_profile();
        profile.sections.push(crate::config::SectionConfig {
            id: "plugin".into(), instance: "plugin-1".into(), enabled: true,
            plugin_config: Some(toml::Value::Table(toml::map::Map::new())),
            position: None, show_header: true, monitor: 0, x: 0, y: 0,
            width: 360, height: None, scale: 1.0,
            color_mode: None, color_a: None, color_b: None, gradient_direction: None,
            clock_font: None, clock_color: None, clock_seconds: false,
            clock_24h: false, clock_ampm: true, clock_pad: true,
            clock_layout: None, clock_align: None,
            date_weekday: true, date_format: None, date_color: None,
            interactive: false, timezone: None, plugin_id: None, plugin_interval: 30,
            children: vec![], panel_gap: 0, panel_padding: 0,
            accent_color: None, transparent_surface: None, opacity_override: None,
            border_visible: None, radius_override: None, padding_override: None,
            font_scale: None, chart_colors: None,
            viz_bar_count: None, viz_min_hz: None, viz_max_hz: None,
            viz_gain: None, viz_smoothing: None, viz_decay: None,
            viz_mirror: None, viz_gap: None, viz_rounded_caps: None, viz_fps: None,
            extra: BTreeMap::new(),
        });
        let output = show_config(true, &config, &profile);
        assert!(output.contains("<redacted>"));
    }
}
