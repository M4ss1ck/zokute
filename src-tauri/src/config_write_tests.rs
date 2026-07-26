use crate::config::load_or_create;
use std::fs;
use tempfile::TempDir;

#[test]
fn load_or_create_errors_when_parent_is_a_file() {
    let temp = TempDir::new().unwrap();
    let parent = temp.path().join("parent");
    fs::write(&parent, "not a directory").unwrap();
    let path = parent.join("config.toml");
    let detected: Vec<String> = vec![];
    assert!(load_or_create(&path, &detected).is_err());
}

use crate::config::{Config, DiskPreference, SectionConfig};
use crate::config_write::{sanitize, should_reload};

fn section(id: &str, width: u32) -> SectionConfig {
        SectionConfig { id: id.into(), instance: id.into(), enabled: true, show_header: true, monitor: 0, x: 0, y: 0, width, scale: 1.0, color_mode: None, color_a: None, color_b: None }
    }

fn config() -> Config {
    Config {
        opacity: 0.9,
        text_opacity: 1.0,
        text_color: "#292824".into(),
        graph_color: None,
        icon_color: None,
        show_background: true,
        sections: vec![section("cpu", 360)],
        system_fields: vec!["os".into()],
        show_cpu_cores: true,
        disks: vec![DiskPreference { id: "a".into(), enabled: true, label: None }],
    }
}

#[test]
fn clamps_opacity_into_the_usable_range() {
    let mut too_low = config();
    too_low.opacity = 0.0;
    assert_eq!(sanitize(too_low).opacity, 0.1);
    let mut too_high = config();
    too_high.opacity = 4.0;
    assert_eq!(sanitize(too_high).opacity, 1.0);
    let mut not_a_number = config();
    not_a_number.opacity = f64::NAN;
    assert_eq!(sanitize(not_a_number).opacity, 1.0);
    let mut transparent_text = config();
    transparent_text.text_opacity = 0.0;
    assert_eq!(sanitize(transparent_text).text_opacity, 0.1);
}

#[test]
fn raises_widths_to_the_minimum() {
    let mut narrow = config();
    narrow.sections = vec![section("cpu", 4)];
    assert_eq!(sanitize(narrow).sections[0].width, 120);
}

#[test]
fn replaces_invalid_text_colors() {
    let mut invalid = config();
    invalid.text_color = "transparent; color: red".into();
    invalid.graph_color = Some("red".into());
    invalid.icon_color = Some("#12345g".into());
    let sanitized = sanitize(invalid);
    assert_eq!(sanitized.text_color, "#292824");
    assert_eq!(sanitized.graph_color, None);
    assert_eq!(sanitized.icon_color, None);
}

#[test]
fn clamps_widget_scales_into_the_usable_range() {
    let mut too_small = config();
    too_small.sections[0].scale = 0.1;
    assert_eq!(sanitize(too_small).sections[0].scale, 0.5);
    let mut too_large = config();
    too_large.sections[0].scale = 9.0;
    assert_eq!(sanitize(too_large).sections[0].scale, 3.0);
}

#[test]
fn drops_unknown_sections_and_keeps_duplicate_widget_types() {
    let mut messy = config();
    messy.sections = vec![section("cpu", 360), section("bogus", 360), section("cpu", 999)];
    messy.sections[2].instance = "cpu-2".into();
    let cleaned = sanitize(messy);
    assert_eq!(cleaned.sections.len(), 2);
    assert_eq!(cleaned.sections[0].width, 360);
    assert_eq!(cleaned.sections[1].width, 999);
}

#[test]
fn leaves_a_missing_section_missing() {
    let cleaned = sanitize(config());
    assert!(cleaned.section("system").is_none());
}

#[test]
fn dedupes_system_fields_and_disks_keeping_the_first() {
    let mut duplicated = config();
    duplicated.system_fields = vec!["os".into(), "kernel".into(), "os".into()];
    duplicated.disks = vec![
        DiskPreference { id: "a".into(), enabled: true, label: Some("First".into()) },
        DiskPreference { id: "a".into(), enabled: false, label: None },
    ];
    let cleaned = sanitize(duplicated);
    assert_eq!(cleaned.system_fields, vec!["os".to_string(), "kernel".to_string()]);
    assert_eq!(cleaned.disks.len(), 1);
    assert_eq!(cleaned.disks[0].label.as_deref(), Some("First"));
}

#[test]
fn skips_reloading_our_own_write_and_reloads_anything_else() {
    assert!(!should_reload("opacity = 0.9\n", "opacity = 0.9\n"));
    assert!(should_reload("opacity = 0.9\n", "opacity = 0.5\n"));
    assert!(should_reload("", "opacity = 0.9\n"));
}
