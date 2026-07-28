use serde::{de::Error as _, Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;
#[path = "config_migration.rs"]
mod config_migration;
#[path = "config_defaults.rs"]
mod config_defaults;
#[path = "config_fields.rs"] mod config_fields;
#[path = "config_section.rs"] mod config_section;
pub use config_defaults::normalize_instances;
pub use config_section::SectionConfig;
use crate::{atomic_file, config_error::ConfigError, config_validate, monitor::MonitorCatalog, paths};

pub const KNOWN_SECTION_IDS: &[&str] = &["system", "cpu", "memory", "disk", "network", "spectrum", "ring", "clock", "date", "plugin", "panel"];
pub(crate) const DEFAULT_SYSTEM_FIELDS: [&str; 22] = ["os", "host", "kernel", "uptime", "packages", "shell", "display", "de", "wm", "wm_theme", "theme", "icons", "font", "cursor", "terminal", "cpu", "gpu", "memory", "swap", "disk", "local_ip", "locale"];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "config_defaults::default_schema_version")]
    pub schema_version: u32,
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
    pub sections: Vec<SectionConfig>,
    pub system_fields: Vec<String>,
    pub show_cpu_cores: bool,
    pub disks: Vec<DiskPreference>,
    #[serde(default)]
    pub monitor_catalog: MonitorCatalog,
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
    pub fn known_sections(&self) -> Vec<&SectionConfig> {
        self.sections.iter().filter(|s| KNOWN_SECTION_IDS.contains(&s.id.as_str())).collect()
    }
    pub fn first_enabled_known_section(&self) -> Option<&SectionConfig> {
        self.known_sections().into_iter().find(|s| s.enabled)
    }
    pub fn section(&self, instance: &str) -> Option<&SectionConfig> {
        self.sections.iter().find(|s| s.instance == instance)
    }
    pub fn disk_preference(&self, id: &str) -> Option<&DiskPreference> {
        self.disks.iter().find(|d| d.id == id)
    }
}

pub fn load(path: &Path) -> Result<Config, ConfigError> {
    let source = fs::read_to_string(path)?;
    parse(&source).map_err(|e| ConfigError::Parse(e.to_string()))
}

pub fn load_or_create(path: &Path, detected_disks: &[String]) -> Result<Config, ConfigError> {
    if !path.exists() { return create_or_migrate_legacy(path, detected_disks); }
    let source = fs::read_to_string(path)?;
    match parse(&source) {
        Ok(config) => { config_validate::check(&config)?; Ok(config) }
        Err(_) => {
            if let Ok(config) = config_migration::migrate(&source, detected_disks) {
                atomic_file::write(path, &serialize(&config))?;
                config_validate::check(&config)?;
                return Ok(config);
            }
            Err(ConfigError::Parse("unable to parse or migrate config".into()))
        }
    }
}

fn create_or_migrate_legacy(new_path: &Path, detected_disks: &[String]) -> Result<Config, ConfigError> {
    let legacy = paths::legacy_config_path();
    if legacy.exists() {
        let source = fs::read_to_string(&legacy)?;
        for attempt in [parse(&source), config_migration::migrate(&source, detected_disks)] {
            if let Ok(config) = attempt {
                atomic_file::backup_previous(&legacy, &paths::backups_dir())?;
                atomic_file::write(new_path, &serialize(&config))?;
                let _ = fs::remove_file(&legacy);
                return Ok(config);
            }
        }
    }
    let config = config_defaults::fresh(detected_disks);
    atomic_file::write(new_path, &serialize(&config))?;
    Ok(config)
}

pub fn parse(source: &str) -> Result<Config, toml::de::Error> {
    let value: toml::Value = toml::from_str(source)?;
    let version = value.get("schema_version").and_then(|v| v.as_integer());
    if version.is_some_and(|v| v > config_validate::CURRENT_SCHEMA_VERSION as i64) {
        return Err(toml::de::Error::custom(format!("schema version {} is newer than supported {}", version.unwrap(), config_validate::CURRENT_SCHEMA_VERSION)));
    }
    if version == Some(0) { return Err(toml::de::Error::custom("schema version 0 is invalid")); }
    let mut config: Config = toml::from_str(source)?;
    for section in &mut config.sections {
        if section.position.is_none() {
            section.position = Some(crate::position_model::Position::Anchored {
                monitor_identity: section.monitor.to_string(),
                anchor: crate::position_model::Anchor::TopLeft,
                offset_x: section.x, offset_y: section.y,
            });
        }
    }
    Ok(config_defaults::normalize_instances(config_fields::upgrade_system_fields(config)))
}

pub fn serialize(config: &Config) -> String {
    toml::to_string_pretty(config).expect("config")
}

pub fn fresh_defaults(detected_disks: &[String]) -> Config {
    config_defaults::fresh(detected_disks)
}
