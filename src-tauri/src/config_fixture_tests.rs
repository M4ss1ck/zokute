use crate::config::{load, load_or_create, parse, serialize};
use std::fs;
use tempfile::TempDir;

const FIXTURES: &str = "tests/fixtures/config";

fn read_fixture(name: &str) -> String {
    fs::read_to_string(format!("{FIXTURES}/{name}")).expect("fixture")
}

#[test]
fn v1_config_parses_and_round_trips() {
    let source = read_fixture("v1.toml");
    let config = parse(&source).expect("v1 parse");
    assert_eq!(config.schema_version, 1);
    assert!((config.opacity - 0.92).abs() < 0.001);
    assert_eq!(config.sections.len(), 2);
    let output = serialize(&config);
    let reparsed = parse(&output).expect("reparse");
    assert_eq!(reparsed.sections.len(), 2);
}

#[test]
fn legacy_config_migrates_to_v1() {
    let temp = TempDir::new().unwrap();
    let legacy_path = temp.path().join("config.toml");
    let source = read_fixture("legacy.toml");
    fs::write(&legacy_path, &source).unwrap();
    let detected = vec!["sda".into()];
    let config = load_or_create(&legacy_path, &detected).expect("migrate");
    assert!((config.opacity - 0.85).abs() < 0.001);
    assert!(config.sections.iter().filter(|s| s.enabled).count() >= 3);
    let output = fs::read_to_string(&legacy_path).unwrap();
    assert!(output.contains("schema_version"));
    let v2 = parse(&output).expect("reparse migrated");
    assert_eq!(v2.schema_version, 2);
}

#[test]
fn unknown_fields_survive_round_trip() {
    let source = read_fixture("unknown_fields.toml");
    let config = parse(&source).expect("parse");
    assert!(config.extra.contains_key("custom_global_setting"));
    assert!(config.extra.contains_key("experimental_flag"));
    assert_eq!(config.sections[0].extra.get("future_section_field").and_then(|v| v.as_integer()), Some(42));
    let output = serialize(&config);
    assert!(output.contains("custom_global_setting"));
    assert!(output.contains("experimental_flag"));
    assert!(output.contains("future_section_field"));
    assert!(output.contains("extra_disk_prop"));
}

#[test]
fn malformed_toml_is_rejected() {
    let source = read_fixture("malformed.toml");
    assert!(parse(&source).is_err());
}

#[test]
fn newer_schema_version_is_rejected() {
    let source = read_fixture("newer_schema.toml");
    assert!(parse(&source).is_err());
}

#[test]
fn duplicate_instances_get_normalized() {
    let mut source = read_fixture("duplicate_instances.toml");
    source.push_str("\n[[disks]]\nid = \"sda\"\nenabled = true\n");
    let config = parse(&source).expect("parse");
    let instances: Vec<&str> = config.sections.iter().map(|s| s.instance.as_str()).collect();
    assert_ne!(instances[0], instances[1]);
    assert_eq!(instances.len(), 2);
}

#[test]
fn v1_round_trips_through_disk() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("zokute.toml");
    let source = read_fixture("v1.toml");
    fs::write(&path, &source).unwrap();
    let loaded = load(&path).expect("load v1");
    let output = serialize(&loaded);
    assert!(output.contains("schema_version = 1"));
}
