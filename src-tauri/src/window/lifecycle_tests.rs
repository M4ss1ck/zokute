use super::{
    is_control_window,
    lifecycle::{desired_action, WindowAction},
    LABELS,
};
use crate::config::SectionConfig;
use std::collections::BTreeMap;

fn section(id: &str, enabled: bool) -> SectionConfig {
    SectionConfig { id: id.into(), instance: format!("{id}-1"), enabled, show_header: true, position: None, monitor: 0, x: 0, y: 0, width: 360, height: None, scale: 1.0, color_mode: None, color_a: None, color_b: None, gradient_direction: None, clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None, date_weekday: true, date_format: None, date_color: None, interactive: false, timezone: None, plugin_id: None, plugin_interval: 30, plugin_config: None, children: vec![], panel_gap: 0, panel_padding: 0, accent_color: None, transparent_surface: None, opacity_override: None, border_visible: None, radius_override: None, padding_override: None, font_scale: None, chart_colors: None, viz_bar_count: None, viz_min_hz: None, viz_max_hz: None, viz_gain: None, viz_smoothing: None, viz_decay: None, viz_mirror: None, viz_gap: None, viz_rounded_caps: None, viz_fps: None, extra: BTreeMap::new() }
}

#[test]
fn unknown_labels_are_ignored() {
    assert!(desired_action(&section("bogus", true), false).is_none());
}

#[test]
fn enabled_known_labels_create_or_update() {
    assert!(matches!(
        desired_action(&section("system", true), false),
        Some(WindowAction::Create(_)),
    ));
    assert!(matches!(
        desired_action(&section("system", true), true),
        Some(WindowAction::Update(_)),
    ));
}

#[test]
fn disabled_known_labels_close_when_present() {
    assert!(matches!(desired_action(&section("cpu", false), true), Some(WindowAction::Close)));
    assert!(desired_action(&section("cpu", false), false).is_none());
}

#[test]
fn capabilities_cover_dynamic_instance_labels() {
    let capability = include_str!("../../capabilities/default.json");
    for label in LABELS {
        assert!(capability.contains(&format!("\"{label}-*\"")));
    }
}

#[test]
fn control_windows_are_not_treated_as_orphaned_widgets() {
    assert!(is_control_window("settings"));
    assert!(is_control_window("layout-editor"));
    assert!(!is_control_window("cpu"));
}
