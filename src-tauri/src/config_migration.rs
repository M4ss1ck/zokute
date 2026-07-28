use super::{Config, DiskPreference, SectionConfig, DEFAULT_SYSTEM_FIELDS, KNOWN_SECTION_IDS};
use crate::monitor::MonitorCatalog;
use crate::position_model::{Anchor, Position};
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Deserialize)]
struct LegacyConfig {
    monitor: usize, x: i32, y: i32, width: u32, opacity: f64, widgets: Vec<String>,
}

#[derive(Deserialize)]
pub struct V1Section {
    pub id: String,
    #[serde(default)] pub instance: String,
    pub enabled: bool,
    #[serde(default = "t")] pub show_header: bool,
    pub monitor: usize, pub x: i32, pub y: i32, pub width: u32,
    #[serde(default)] pub height: Option<u32>,
    #[serde(default = "o")] pub scale: f64,
    #[serde(default)] pub color_mode: Option<String>,
    #[serde(default)] pub color_a: Option<String>,
    #[serde(default)] pub color_b: Option<String>,
    #[serde(default)] pub gradient_direction: Option<String>,
    #[serde(default)] pub clock_font: Option<String>,
    #[serde(default)] pub clock_color: Option<String>,
    #[serde(default)] pub clock_seconds: bool,
    #[serde(default)] pub clock_24h: bool,
    #[serde(default = "t")] pub clock_ampm: bool,
    #[serde(default = "t")] pub clock_pad: bool,
    #[serde(default)] pub clock_layout: Option<String>,
    #[serde(default)] pub clock_align: Option<String>,
    #[serde(default = "t")] pub date_weekday: bool,
    #[serde(default)] pub date_format: Option<String>,
    #[serde(default)] pub date_color: Option<String>,
    #[serde(flatten)] pub extra: BTreeMap<String, toml::Value>,
}

fn t() -> bool { true }
fn o() -> f64 { 1.0 }

impl From<V1Section> for SectionConfig {
    fn from(s: V1Section) -> Self {
        SectionConfig {
            id: s.id, instance: s.instance, enabled: s.enabled, show_header: s.show_header,
            position: Some(Position::Anchored {
                monitor_identity: s.monitor.to_string(), anchor: Anchor::TopLeft,
                offset_x: s.x, offset_y: s.y,
            }),
            monitor: s.monitor, x: s.x, y: s.y,
            width: s.width, height: s.height, scale: s.scale,
            color_mode: s.color_mode, color_a: s.color_a, color_b: s.color_b,
            gradient_direction: s.gradient_direction,
            clock_font: s.clock_font, clock_color: s.clock_color,
            clock_seconds: s.clock_seconds, clock_24h: s.clock_24h,
            clock_ampm: s.clock_ampm, clock_pad: s.clock_pad,
            clock_layout: s.clock_layout, clock_align: s.clock_align,
            date_weekday: s.date_weekday, date_format: s.date_format, date_color: s.date_color,
            interactive: false, timezone: None, plugin_id: None, plugin_interval: 30, plugin_config: None,
            extra: s.extra,
        }
    }
}

#[derive(Deserialize)]
struct V1Disk { id: String, #[serde(default = "t")] enabled: bool, #[serde(default)] label: Option<String>, #[serde(flatten)] extra: BTreeMap<String, toml::Value> }

fn v1_disk(d: V1Disk) -> DiskPreference {
    DiskPreference { id: d.id, enabled: d.enabled, label: d.label, extra: d.extra }
}

const SECTION_Y_OFFSETS: [i32; 5] = [0, 216, 376, 480, 640];

fn legacy_to_config(legacy: LegacyConfig, detected_disks: &[String]) -> Config {
    Config {
        schema_version: 2, opacity: legacy.opacity,
        text_opacity: 1.0, text_color: super::default_text_color(),
        graph_color: None, icon_color: None, show_background: true,
        sections: KNOWN_SECTION_IDS.iter().zip(SECTION_Y_OFFSETS).map(|(id, y)| SectionConfig {
            id: id.to_string(), instance: id.to_string(),
            enabled: legacy.widgets.iter().any(|widget| widget == id), show_header: true,
            position: Some(Position::Anchored {
                monitor_identity: legacy.monitor.to_string(),
                anchor: Anchor::TopLeft, offset_x: legacy.x, offset_y: legacy.y + y,
            }),
            monitor: legacy.monitor, x: legacy.x, y: legacy.y + y,
            width: legacy.width, height: None, scale: 1.0,
            color_mode: None, color_a: None, color_b: None,
            gradient_direction: None, clock_font: None, clock_color: None,
            clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true,
            clock_layout: None, clock_align: None,
            date_weekday: true, date_format: None, date_color: None,
            interactive: false, timezone: None, plugin_id: None, plugin_interval: 30, plugin_config: None,
            extra: BTreeMap::new(),
        }).collect(),
        system_fields: DEFAULT_SYSTEM_FIELDS.iter().map(|f| f.to_string()).collect(),
        show_cpu_cores: true,
        disks: detected_disks.iter().map(|id| DiskPreference { id: id.clone(), enabled: true, label: None, extra: BTreeMap::new() }).collect(),
        monitor_catalog: MonitorCatalog::new(), extra: BTreeMap::new(),
    }
}

pub(super) fn migrate(source: &str, detected_disks: &[String]) -> Result<Config, toml::de::Error> {
    if let Ok(legacy) = toml::from_str::<LegacyConfig>(source) {
        return Ok(legacy_to_config(legacy, detected_disks));
    }
    let v1: V1Config = toml::from_str(source)?;
    Ok(v1_to_config(v1))
}

#[derive(Deserialize)]
struct V1Config {
    opacity: f64, #[serde(default = "o")] text_opacity: f64,
    #[serde(default = "default_text_color")] text_color: String,
    #[serde(default)] graph_color: Option<String>, #[serde(default)] icon_color: Option<String>,
    #[serde(default = "t")] show_background: bool,
    sections: Vec<V1Section>, system_fields: Vec<String>, show_cpu_cores: bool,
    #[serde(default)] disks: Vec<V1Disk>,
    #[serde(flatten)] extra: BTreeMap<String, toml::Value>,
}

fn default_text_color() -> String { "#292824".into() }

fn v1_to_config(v1: V1Config) -> Config {
    Config {
        schema_version: 2,
        opacity: v1.opacity, text_opacity: v1.text_opacity, text_color: v1.text_color,
        graph_color: v1.graph_color, icon_color: v1.icon_color,
        show_background: v1.show_background,
        sections: v1.sections.into_iter().map(Into::into).collect(),
        system_fields: v1.system_fields, show_cpu_cores: v1.show_cpu_cores,
        disks: v1.disks.into_iter().map(v1_disk).collect(),
        monitor_catalog: MonitorCatalog::new(), extra: v1.extra,
    }
}
