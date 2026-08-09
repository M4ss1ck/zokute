use crate::config::{Config, DiskPreference, ConfigError, Profile};
use crate::config_validate;
use crate::monitor::MonitorCatalog;
use serde::Deserialize;
use std::collections::BTreeMap;

/// Internal representation of a v2 config file (has sections, system_fields, etc.)
#[derive(Deserialize)]
struct V2Config {
    #[serde(default)]
    schema_version: u32,
    opacity: f64,
    #[serde(default = "default_text_opacity")]
    text_opacity: f64,
    #[serde(default = "default_text_color")]
    text_color: String,
    #[serde(default)]
    graph_color: Option<String>,
    #[serde(default)]
    icon_color: Option<String>,
    #[serde(default = "default_true")]
    show_background: bool,
    #[serde(default = "default_theme")]
    theme: String,
    #[serde(default)]
    accent_color: Option<String>,
    #[serde(default = "default_density")]
    density: String,
    #[serde(default = "default_font_scale")]
    font_scale: f64,
    #[serde(default)]
    sans_font: Option<String>,
    #[serde(default)]
    mono_font: Option<String>,
    #[serde(default = "default_byte_format")]
    byte_format: String,
    #[serde(default = "default_temperature_unit")]
    temperature_unit: String,
    #[serde(default)]
    locale: Option<String>,
    #[serde(default)]
    sections: Vec<crate::config::SectionConfig>,
    #[serde(default)]
    system_fields: Vec<String>,
    #[serde(default = "default_true")]
    show_cpu_cores: bool,
    #[serde(default)]
    disks: Vec<DiskPreference>,
    #[serde(default)]
    monitor_catalog: MonitorCatalog,
    #[serde(flatten)]
    extra: BTreeMap<String, toml::Value>,
}

fn default_text_opacity() -> f64 { 1.0 }
fn default_text_color() -> String { "#292824".into() }
fn default_true() -> bool { true }
fn default_theme() -> String { "light".into() }
fn default_density() -> String { "compact".into() }
fn default_font_scale() -> f64 { 1.0 }
fn default_byte_format() -> String { "binary".into() }
fn default_temperature_unit() -> String { "celsius".into() }

/// Parse a v2 source string and migrate to (v3 Config, Profile).
pub fn migrate_v2_source(source: &str, _detected_disks: &[String]) -> Result<(Config, Profile), ConfigError> {
    let v2: V2Config = toml::from_str(source)
        .map_err(|e| ConfigError::Parse(e.to_string()))?;
    // Must have sections to be a valid v2 config
    if v2.sections.is_empty() && v2.system_fields.is_empty() && v2.disks.is_empty() {
        return Err(ConfigError::Migration("not a v2 config".into()));
    }
    // If schema_version is already 3+, this isn't a v2 migration
    if v2.schema_version >= 3 {
        return Err(ConfigError::Migration("already v3".into()));
    }
    let profile = Profile {
        profile_schema_version: crate::config::profile_mod::PROFILE_SCHEMA_VERSION,
        sections: v2.sections,
        system_fields: v2.system_fields,
        show_cpu_cores: v2.show_cpu_cores,
        disks: v2.disks,
        collect_interval_ms: 1000, monitor_catalog: v2.monitor_catalog,
        fullscreen: Default::default(), extra: BTreeMap::new(),
    };
    // Remove section-related and legacy fields from extra before creating Config
    let mut extra = v2.extra;
    for key in &["sections", "system_fields", "show_cpu_cores", "disks", "monitor_catalog",
                 "height", "monitor", "widgets", "x", "y", "width"] {
        extra.remove(*key);
    }
    let config = Config {
        schema_version: config_validate::CURRENT_SCHEMA_VERSION,
        active_profile: "default".into(),
        opacity: v2.opacity,
        text_opacity: v2.text_opacity,
        text_color: v2.text_color,
        graph_color: v2.graph_color,
        icon_color: v2.icon_color,
        show_background: v2.show_background,
        theme: v2.theme,
        accent_color: v2.accent_color,
        density: v2.density,
        font_scale: v2.font_scale,
        sans_font: v2.sans_font,
        mono_font: v2.mono_font,
        byte_format: v2.byte_format,
        temperature_unit: v2.temperature_unit,
        locale: v2.locale,
        extra,
    };
    Ok((config, profile))
}
