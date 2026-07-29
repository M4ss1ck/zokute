use crate::config::{Config, Profile};
use crate::onboarding::WorkArea;
use crate::position_model::{Anchor, Position};
use std::collections::BTreeMap;

const SECTION_WIDTH: u32 = 360;

/// Builds the starting config and profile for a preset. Anchors come from the
/// caller's detected work area (M10 Task 2.4) so this stays free of GTK and can
/// be exercised without a display.
pub fn snapshot_for(preset: &str, theme: &str, area: &WorkArea) -> (Config, Profile) {
    let first = area.clone();

    let config = Config {
        schema_version: 3,
        active_profile: "default".into(),
        opacity: 0.92,
        text_opacity: 1.0,
        text_color: "#292824".into(),
        graph_color: None,
        icon_color: None,
        show_background: true,
        theme: theme.into(),
        accent_color: None,
        density: "compact".into(),
        font_scale: 1.0,
        sans_font: None,
        mono_font: None,
        byte_format: "binary".into(),
        temperature_unit: "celsius".into(),
        locale: None,
        extra: BTreeMap::new(),
    };

    let profile = match preset {
        "minimal" => Profile {
            profile_schema_version: 1,
            sections: vec![
                mk_section("clock", &first, 0),
                mk_section("date", &first, 1),
            ],
            system_fields: vec![],
            show_cpu_cores: false,
            disks: vec![],
            collect_interval_ms: 1000,
            monitor_catalog: BTreeMap::new(),
            fullscreen: Default::default(), extra: BTreeMap::new(),
        },
        "system_monitor" => {
            let ids = ["clock", "date", "cpu", "memory", "disk", "network"];
            let sections: Vec<_> = ids.iter().enumerate().map(|(i, id)| {
                mk_section(id, &first, i as i32)
            }).collect();
            Profile {
                profile_schema_version: 1,
                sections,
                system_fields: crate::config::DEFAULT_SYSTEM_FIELDS.iter().map(|f| f.to_string()).collect(),
                show_cpu_cores: true,
                disks: vec![],
                collect_interval_ms: 1000,
                monitor_catalog: BTreeMap::new(),
                fullscreen: Default::default(), extra: BTreeMap::new(),
            }
        }
        _ => Profile {
            profile_schema_version: 1,
            sections: vec![],
            system_fields: vec![],
            show_cpu_cores: false,
            disks: vec![],
            collect_interval_ms: 1000,
            monitor_catalog: BTreeMap::new(),
            fullscreen: Default::default(), extra: BTreeMap::new(),
        },
    };

    (config, normalize_onboarding_profile(profile))
}

fn mk_section(id: &str, area: &WorkArea, index: i32) -> crate::config::SectionConfig {
    let y_offset = 24 + index * 120;
    crate::config::SectionConfig {
        id: id.to_string(),
        instance: id.to_string(),
        enabled: true,
        show_header: true,
        position: Some(Position::Anchored {
            monitor_identity: area.identity.clone(),
            anchor: Anchor::TopLeft,
            offset_x: 24,
            offset_y: y_offset,
        }),
        monitor: 0, x: 24, y: y_offset,
        width: SECTION_WIDTH, height: None, scale: 1.0,
        color_mode: None, color_a: None, color_b: None, gradient_direction: None,
        clock_font: None, clock_color: None, clock_seconds: false,
        clock_24h: false, clock_ampm: true, clock_pad: true,
        clock_layout: None, clock_align: None,
        date_weekday: true, date_format: None, date_color: None,
        interactive: false, timezone: None,
        plugin_id: None, plugin_interval: 30, plugin_config: None,
        children: vec![], panel_gap: 0, panel_padding: 0,
        accent_color: None, transparent_surface: None, opacity_override: None,
        border_visible: None, radius_override: None, padding_override: None,
        font_scale: None, chart_colors: None,
        viz_bar_count: None, viz_min_hz: None, viz_max_hz: None,
        viz_gain: None, viz_smoothing: None, viz_decay: None,
        viz_mirror: None, viz_gap: None, viz_rounded_caps: None, viz_fps: None,
        extra: BTreeMap::new(),
    }
}

fn normalize_onboarding_profile(mut profile: Profile) -> Profile {
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

