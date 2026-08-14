use super::{
    Config, DiskPreference, SectionConfig, DEFAULT_SYSTEM_FIELDS,
};
use crate::monitor::MonitorCatalog;
use crate::position_model::{Anchor, Position};
use crate::config::Profile;
use std::collections::BTreeMap;

const SECTION_Y_OFFSETS: [i32; 5] = [0, 216, 376, 480, 640];
const DEFAULT_SECTION_IDS: [&str; 5] = ["system", "cpu", "memory", "disk", "network"];

pub(super) fn default_schema_version() -> u32 { 3 }
pub(super) fn default_active_profile() -> String { "default".into() }
pub(super) fn default_scale() -> f64 { 1.0 }
pub(super) fn default_text_opacity() -> f64 { 1.0 }
pub(super) fn default_text_color() -> String { "#292824".into() }
pub(super) fn default_theme() -> String { "light".into() }
pub(super) fn default_density() -> String { "compact".into() }
pub(super) fn default_font_scale() -> f64 { 1.0 }
pub(super) fn default_byte_format() -> String { "binary".into() }
pub(super) fn default_temperature_unit() -> String { "celsius".into() }
pub(super) fn default_true() -> bool { true }

pub fn normalize_profile_instances(mut profile: Profile) -> Profile {
    let mut labels: Vec<String> = Vec::new();
    for section in &mut profile.sections {
        let base = if section.instance.is_empty() { &section.id } else { &section.instance };
        let mut label = base.clone();
        let mut suffix = 2;
        while labels.contains(&label) { label = format!("{base}-{suffix}"); suffix += 1; }
        section.instance = label.clone();
        labels.push(label);
    }
    profile
}

pub(super) fn fresh(detected_disks: &[String]) -> Config {
    let _ = detected_disks;
    Config {
        schema_version: 3,
        active_profile: "default".into(),
        opacity: 0.92,
        text_opacity: default_text_opacity(),
        text_color: default_text_color(),
        graph_color: None,
        icon_color: None,
        show_background: default_true(),
        theme: default_theme(),
        accent_color: None,
        density: "compact".into(),
        font_scale: 1.0,
        sans_font: None,
        mono_font: None,
        byte_format: "binary".into(),
        temperature_unit: "celsius".into(),
        locale: None,
        extra: BTreeMap::new(),
    }
}

pub(super) fn fresh_profile(detected_disks: &[String]) -> Profile {
    Profile {
        profile_schema_version: crate::config::profile_mod::PROFILE_SCHEMA_VERSION,
        sections: DEFAULT_SECTION_IDS.iter().zip(SECTION_Y_OFFSETS).map(|(id, y)| SectionConfig {
                id: id.to_string(),
                instance: id.to_string(),
                enabled: true,
                show_header: true,
                position: Some(Position::Anchored {
                    monitor_identity: "0".into(),
                    anchor: Anchor::TopLeft,
                    offset_x: 24,
                    offset_y: 24 + y,
                }),
                monitor: 0,
                x: 24,
                y: 24 + y,
                width: 360,
                height: None,
                scale: default_scale(),
                color_mode: None,
                color_a: None,
                color_b: None,
                gradient_direction: None,
                clock_font: None,
                clock_color: None,
                clock_seconds: false,
                clock_24h: false,
                clock_ampm: true,
                clock_pad: true,
                clock_layout: None,
                clock_align: None,
                date_weekday: true,
                date_format: None,
                date_color: None,
                interactive: false, timezone: None, plugin_id: None, plugin_interval: 30, plugin_config: None, children: vec![], panel_gap: 0, panel_padding: 0, panel_dividers: true, accent_color: None, transparent_surface: None, opacity_override: None, border_visible: None, radius_override: None, padding_override: None, font_scale: None, chart_colors: None,
                viz_bar_count: None, viz_min_hz: None, viz_max_hz: None, viz_gain: None, viz_smoothing: None, viz_decay: None, viz_mirror: None, viz_gap: None, viz_rounded_caps: None, viz_fps: None,
                extra: BTreeMap::new(),
            })
            .collect(),
        system_fields: DEFAULT_SYSTEM_FIELDS.iter().map(|field| field.to_string()).collect(),
        show_cpu_cores: true,
        disks: detected_disks
            .iter()
            .map(|id| DiskPreference { id: id.clone(), enabled: true, label: None, extra: BTreeMap::new() })
            .collect(),
        collect_interval_ms: 1000,
        monitor_catalog: MonitorCatalog::new(), fullscreen: Default::default(),
        extra: BTreeMap::new(),
    }
}
