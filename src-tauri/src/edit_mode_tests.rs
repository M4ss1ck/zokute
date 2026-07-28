use crate::config::{Config, DiskPreference, SectionConfig};
use crate::edit_geometry::{apply_placement, reset_box};
use crate::window::position::Placement;
use std::collections::BTreeMap;

fn config() -> Config {
        Config {
        schema_version: 1,
        opacity: 1.0,
        text_opacity: 1.0,
        text_color: "#292824".into(),
        graph_color: None,
        icon_color: None,
        show_background: true,
        sections: vec![
        SectionConfig { id: "cpu".into(), instance: "cpu".into(), enabled: true, show_header: true, position: None, monitor: 1, x: 1, y: 2, width: 300, height: None, scale: 1.0, color_mode: None, color_a: None, color_b: None, gradient_direction: None, clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None, date_weekday: true, date_format: None, date_color: None, extra: BTreeMap::new() },
        SectionConfig { id: "disk".into(), instance: "disk".into(), enabled: true, show_header: true, position: None, monitor: 3, x: 3, y: 4, width: 300, height: None, scale: 1.0, color_mode: None, color_a: None, color_b: None, gradient_direction: None, clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None, date_weekday: true, date_format: None, date_color: None, extra: BTreeMap::new() },
        ],
        system_fields: vec![],
        show_cpu_cores: true,
        disks: vec![DiskPreference { id: "a".into(), enabled: true, label: None, extra: BTreeMap::new() }],
        monitor_catalog: BTreeMap::new(),
        extra: BTreeMap::new(),
    }
}

#[test]
fn writes_a_placement_onto_its_own_section_only() {
    let updated = apply_placement(config(), "cpu", Placement { monitor_identity: "1".into(), x: 40, y: 50, width: 420, height: 240 });
    let cpu = updated.section("cpu").expect("cpu");
    assert_eq!((cpu.width), (420));
    let disk = updated.section("disk").expect("disk");
    assert_eq!((disk.width), (300));
}

#[test]
fn records_the_dragged_height_on_its_own_section_only() {
    let updated = apply_placement(config(), "cpu", Placement { monitor_identity: "1".into(), x: 40, y: 50, width: 420, height: 240 });
    assert_eq!(updated.section("cpu").expect("cpu").height, Some(240));
    assert_eq!(updated.section("disk").expect("disk").height, None);
}

#[test]
fn ignores_a_placement_for_a_section_that_is_not_configured() {
    let updated = apply_placement(config(), "network", Placement { monitor_identity: "1".into(), x: 9, y: 9, width: 9, height: 100 });
    assert!(updated.section("network").is_none());
    assert_eq!(updated.sections.len(), 2);
}

#[test]
fn resetting_a_box_sets_the_width_and_lets_the_height_refit() {
    let mut config = config();
    config.sections[0].height = Some(240);
    config.sections[0].scale = 1.5;
    assert!(reset_box(&mut config, "cpu", 160));
    let cpu = config.section("cpu").expect("cpu");
    assert_eq!((cpu.width, cpu.height), (160, None));
    assert_eq!(cpu.scale, 1.5);
    assert_eq!(config.section("disk").expect("disk").width, 300);
}

#[test]
fn resetting_the_box_of_an_unknown_instance_changes_nothing() {
    let mut config = config();
    assert!(!reset_box(&mut config, "network", 160));
    assert_eq!(config.section("cpu").expect("cpu").width, 300);
}

#[test]
fn preserves_the_enabled_flag_while_moving_a_section() {
    let mut disabled = config();
    disabled.sections[0].enabled = false;
    let updated = apply_placement(disabled, "cpu", Placement { monitor_identity: "0".into(), x: 7, y: 8, width: 200, height: 100 });
    assert!(!updated.section("cpu").expect("cpu").enabled);
}
