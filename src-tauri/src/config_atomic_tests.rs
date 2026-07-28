use crate::config::{load, serialize};
use std::fs;
use tempfile::TempDir;

const FIXTURES: &str = "tests/fixtures/config";

fn read_fixture(name: &str) -> String {
    fs::read_to_string(format!("{FIXTURES}/{name}")).expect("fixture")
}

#[test]
fn atomic_write_preserves_unknown_fields_on_disk() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("zokute.toml");
    let source = read_fixture("unknown_fields.toml");
    fs::write(&path, &source).unwrap();
    let loaded = load(&path).expect("load");
    crate::atomic_file::write(&path, &serialize(&loaded)).unwrap();
    let on_disk = fs::read_to_string(&path).unwrap();
    assert!(on_disk.contains("custom_global_setting"));
    assert!(on_disk.contains("experimental_flag"));
}

#[test]
fn backup_previous_creates_a_recoverable_copy() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("zokute.toml");
    let source = read_fixture("v1.toml");
    fs::write(&path, &source).unwrap();
    let backups = temp.path().join("backups");
    crate::atomic_file::backup_previous(&path, &backups).unwrap();
    let dest = backups.join("zokute.toml.previous");
    assert!(dest.exists());
    let content = fs::read_to_string(&dest).unwrap();
    assert!(content.contains("schema_version"));
    let config = crate::config::parse(&content).expect("backup should be valid");
    assert_eq!(config.schema_version, 1);
}

#[test]
fn failed_write_leaves_original_unchanged() {
    let temp = TempDir::new().unwrap();
    let parent = temp.path().join("parent");
    fs::write(&parent, "not a directory").unwrap();
    let path = parent.join("zokute.toml");
    let source = read_fixture("v1.toml");
    let result = crate::atomic_file::write(&path, &source);
    assert!(result.is_err());
}

#[test]
fn interrupted_temp_file_does_not_corrupt_load() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("zokute.toml");
    let source = read_fixture("v1.toml");
    fs::write(&path, &source).unwrap();
    let tmp = path.with_extension("tmp");
    fs::write(&tmp, "garbage").unwrap();
    let loaded = load(&path).expect("load with stale tmp");
    assert_eq!(loaded.schema_version, 1);
}

#[test]
fn fresh_config_with_no_legacy_creates_defaults() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("zokute.toml");
    let config = crate::config::load_or_create(&path, &["nvme0".into()]).expect("fresh");
    assert_eq!(config.schema_version, 1);
    assert!(config.sections.iter().any(|s| s.enabled));
    assert!(config.disk_preference("nvme0").is_some());
    let output = fs::read_to_string(&path).unwrap();
    assert!(output.contains("schema_version = 1"));
}
