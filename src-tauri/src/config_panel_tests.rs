use crate::config::{Config, Profile, SectionConfig};
use std::collections::BTreeMap;
use tempfile::TempDir;

fn section(id: &str, width: u32) -> SectionConfig {
    SectionConfig { id: id.into(), instance: id.into(), enabled: true, show_header: true, position: None, monitor: 0, x: 0, y: 0, width, height: None, scale: 1.0, color_mode: None, color_a: None, color_b: None, gradient_direction: None, clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None, date_weekday: true, date_format: None, date_color: None, interactive: false, timezone: None, plugin_id: None, plugin_interval: 30, plugin_config: None, children: vec![], panel_gap: 0, panel_padding: 0, panel_dividers: true, accent_color: None, transparent_surface: None, opacity_override: None, border_visible: None, radius_override: None, padding_override: None, font_scale: None, chart_colors: None, viz_bar_count: None, viz_min_hz: None, viz_max_hz: None, viz_gain: None, viz_smoothing: None, viz_decay: None, viz_mirror: None, viz_gap: None, viz_rounded_caps: None, viz_fps: None, extra: BTreeMap::new() }
}

fn config() -> Config {
    Config {
        schema_version: 3,
        active_profile: "default".into(),
        opacity: 0.9,
        text_opacity: 1.0,
        text_color: "#292824".into(),
        graph_color: None,
        icon_color: None,
        show_background: true,
        theme: "dark".into(),
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

#[test]
fn panel_children_and_layout_survive_a_write_read_round_trip() {
    let temp = TempDir::new().unwrap();
    let config_path = temp.path().join("zokute.toml");

    let mut panel = section("panel", 340);
    panel.instance = "panel-1".into();
    panel.panel_gap = 8;
    panel.panel_padding = 12;
    panel.panel_dividers = false;
    let mut cpu = section("cpu", 340);
    cpu.instance = "cpu-1".into();
    let mut memory = section("memory", 340);
    memory.instance = "memory-1".into();
    panel.children = vec![cpu, memory];

    let profile = Profile {
        profile_schema_version: 1,
        sections: vec![panel],
        system_fields: vec![],
        show_cpu_cores: true,
        disks: vec![],
        collect_interval_ms: 1000,
        monitor_catalog: BTreeMap::new(),
        fullscreen: Default::default(),
        extra: BTreeMap::new(),
    };

    // persist() writes the profile through serialize_profile + atomic_file::write;
    // load_or_create reads it back through load_profile's toml::from_str.
    crate::atomic_file::write(&config_path, &crate::config::serialize(&config())).unwrap();
    crate::atomic_file::write(
        &temp.path().join("profiles/default.toml"),
        &crate::config::serialize_profile(&profile),
    )
    .unwrap();

    let (_, loaded) = crate::config::load_or_create(&config_path, &[]).unwrap();
    let panel = loaded.section("panel-1").expect("panel section present");
    assert_eq!(panel.id, "panel");
    assert_eq!(panel.children.len(), 2);
    assert_eq!(panel.children[0].id, "cpu");
    assert_eq!(panel.children[0].instance, "cpu-1");
    assert_eq!(panel.children[1].id, "memory");
    assert_eq!(panel.children[1].instance, "memory-1");
    assert_eq!(panel.panel_gap, 8);
    assert_eq!(panel.panel_padding, 12);
    assert_eq!(panel.panel_dividers, false);
}
