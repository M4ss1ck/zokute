use crate::config::{Config, DiskPreference, SectionConfig};
use crate::edit_mode::apply_placement;
use crate::window::position::Placement;

fn config() -> Config {
    Config {
        opacity: 1.0,
        text_opacity: 1.0,
        text_color: "#292824".into(),
        graph_color: None,
        icon_color: None,
        show_background: true,
        sections: vec![
        SectionConfig { id: "cpu".into(), instance: "cpu".into(), enabled: true, show_header: true, monitor: 0, x: 1, y: 2, width: 300, scale: 1.0, color_mode: None, color_a: None, color_b: None },
        SectionConfig { id: "disk".into(), instance: "disk".into(), enabled: true, show_header: true, monitor: 0, x: 3, y: 4, width: 300, scale: 1.0, color_mode: None, color_a: None, color_b: None },
        ],
        system_fields: vec![],
        show_cpu_cores: true,
        disks: vec![DiskPreference { id: "a".into(), enabled: true, label: None }],
    }
}

#[test]
fn writes_a_placement_onto_its_own_section_only() {
    let updated = apply_placement(config(), "cpu", Placement { monitor: 1, x: 40, y: 50, width: 420 });
    let cpu = updated.section("cpu").expect("cpu");
    assert_eq!((cpu.monitor, cpu.x, cpu.y, cpu.width), (1, 40, 50, 420));
    let disk = updated.section("disk").expect("disk");
    assert_eq!((disk.monitor, disk.x, disk.y, disk.width), (0, 3, 4, 300));
}

#[test]
fn ignores_a_placement_for_a_section_that_is_not_configured() {
    let updated = apply_placement(config(), "network", Placement { monitor: 1, x: 9, y: 9, width: 9 });
    assert!(updated.section("network").is_none());
    assert_eq!(updated.sections.len(), 2);
}

#[test]
fn preserves_the_enabled_flag_while_moving_a_section() {
    let mut disabled = config();
    disabled.sections[0].enabled = false;
    let updated = apply_placement(disabled, "cpu", Placement { monitor: 0, x: 7, y: 8, width: 200 });
    assert!(!updated.section("cpu").expect("cpu").enabled);
}
