use crate::config::{load, load_or_create, parse, serialize};
use std::fs;
use tempfile::TempDir;

const FIXTURES: &str = "tests/fixtures/config";

fn read_fixture(name: &str) -> String {
    fs::read_to_string(format!("{FIXTURES}/{name}")).expect("fixture")
}

#[test]
fn v1_config_parses_via_migration() {
    let source = read_fixture("v1.toml");
    // v1 has sections, so it won't parse as v3 directly.
    // It should be handled by migration.
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("zokute.toml");
    fs::write(&path, &source).unwrap();
    let (config, profile) = load_or_create(&path, &[]).expect("migrate v1");
    assert!((config.opacity - 0.92).abs() < 0.001);
    // Profile should have sections from v1 migration
    assert!(!profile.sections.is_empty());
}

#[test]
fn legacy_config_migrates_to_v3() {
    let temp = TempDir::new().unwrap();
    let legacy_path = temp.path().join("config.toml");
    let source = read_fixture("legacy.toml");
    fs::write(&legacy_path, &source).unwrap();
    let detected = vec!["sda".into()];
    let (config, _profile) = load_or_create(&legacy_path, &detected).expect("migrate");
    assert!((config.opacity - 0.85).abs() < 0.001);
    let output = fs::read_to_string(&legacy_path).unwrap();
    assert!(output.contains("schema_version"));
    let reparsed = parse(&output).expect("reparse migrated");
    assert_eq!(reparsed.schema_version, 3);
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
fn v3_round_trips_through_disk() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("zokute.toml");
    let source = "schema_version = 3\nactive_profile = \"default\"\nopacity = 0.92\n";
    fs::write(&path, &source).unwrap();
    let loaded = load(&path).expect("load v3");
    let output = serialize(&loaded);
    assert!(output.contains("schema_version = 3"));
}
