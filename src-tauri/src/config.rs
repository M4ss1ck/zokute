use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
#[path = "config_migration.rs"] mod config_migration;
#[path = "config_defaults.rs"] mod config_defaults;
#[path = "config_fields.rs"] pub(crate) mod config_fields;
#[path = "config_section.rs"] mod config_section;
#[path = "config_write_ops.rs"] mod config_write_ops;
pub use config_write_ops::{fresh_defaults, fresh_profile, parse, serialize, serialize_profile};
pub use config_defaults::normalize_instances;
pub use config_defaults::normalize_profile_instances;
pub use config_section::SectionConfig;
use crate::{config_error::ConfigError, config_validate, monitor::MonitorCatalog, paths};
#[path = "profile.rs"] pub mod profile_mod;
pub use profile_mod::Profile;
#[path = "profile_migration.rs"] pub(crate) mod profile_migration;

pub const KNOWN_SECTION_IDS: &[&str] = &["system", "cpu", "memory", "disk", "network", "spectrum", "ring", "clock", "date", "plugin", "panel"];
pub(crate) const DEFAULT_SYSTEM_FIELDS: [&str; 22] = ["os", "host", "kernel", "uptime", "packages", "shell", "display", "de", "wm", "wm_theme", "theme", "icons", "font", "cursor", "terminal", "cpu", "gpu", "memory", "swap", "disk", "local_ip", "locale"];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "config_defaults::default_schema_version")]
    pub schema_version: u32,
    #[serde(default = "config_defaults::default_active_profile")]
    pub active_profile: String,
    pub opacity: f64,
    #[serde(default = "config_defaults::default_text_opacity")]
    pub text_opacity: f64,
    #[serde(default = "config_defaults::default_text_color")]
    pub text_color: String,
    #[serde(default)] pub graph_color: Option<String>,
    #[serde(default)] pub icon_color: Option<String>,
    #[serde(default = "config_defaults::default_true")]
    pub show_background: bool,
    #[serde(default = "config_defaults::default_theme")]
    pub theme: String,
    #[serde(default)]
    pub accent_color: Option<String>,
    #[serde(default = "config_defaults::default_density")]
    pub density: String,
    #[serde(default = "config_defaults::default_font_scale")]
    pub font_scale: f64,
    #[serde(default)]
    pub sans_font: Option<String>,
    #[serde(default)]
    pub mono_font: Option<String>,
    #[serde(default = "config_defaults::default_byte_format")]
    pub byte_format: String,
    #[serde(default = "config_defaults::default_temperature_unit")]
    pub temperature_unit: String,
    #[serde(default)]
    pub locale: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, toml::Value>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiskPreference {
    pub id: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, toml::Value>,
}

impl Config {
    pub fn profile_path(&self) -> std::path::PathBuf {
        paths::profile_path(&self.active_profile)
    }
}

/// Resolve profile path relative to a config file's parent directory.
/// This ensures tests using temp directories keep profiles local.
pub(super) fn resolve_profile_path(config_path: &Path, profile_name: &str) -> std::path::PathBuf {
    config_path.parent()
        .map(|p| p.join("profiles").join(format!("{profile_name}.toml")))
        .unwrap_or_else(|| paths::profile_path(profile_name))
}

pub fn load(path: &Path) -> Result<Config, ConfigError> {
    let source = fs::read_to_string(path)?;
    parse(&source).map_err(|e| ConfigError::Parse(e.to_string()))
}

pub fn load_or_create(path: &Path, detected_disks: &[String]) -> Result<(Config, Profile), ConfigError> {
    if !path.exists() { return config_write_ops::create_or_migrate_legacy(path, detected_disks); }
    let source = fs::read_to_string(path)?;
    match parse(&source) {
        Ok(config) => {
            config_validate::check(&config)?;
            let profile = config_write_ops::load_profile(path, &config, detected_disks)?;
            Ok((config, profile))
        }
        Err(_) => {
            if let Ok((config, profile)) = profile_migration::migrate_v2_source(&source, detected_disks) {
                crate::atomic_file::write(path, &serialize(&config))?;
                let profile_path = resolve_profile_path(path, &config.active_profile);
                let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
                crate::atomic_file::write(&profile_path, &serialize_profile(&profile))?;
                config_validate::check(&config)?;
                return Ok((config, profile));
            }
            if let Ok((config, data)) = config_migration::migrate_with_data(&source, detected_disks) {
                let profile = Profile {
                    profile_schema_version: profile_mod::PROFILE_SCHEMA_VERSION,
                    sections: data.sections,
                    system_fields: data.system_fields,
                    show_cpu_cores: data.show_cpu_cores,
                    disks: data.disks,
                    monitor_catalog: MonitorCatalog::new(),
                    extra: BTreeMap::new(),
                };
                crate::atomic_file::write(path, &serialize(&config))?;
                let profile_path = resolve_profile_path(path, &config.active_profile);
                let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
                crate::atomic_file::write(&profile_path, &serialize_profile(&profile))?;
                config_validate::check(&config)?;
                return Ok((config, profile));
            }
            Err(ConfigError::Parse("unable to parse or migrate config".into()))
        }
    }
}
