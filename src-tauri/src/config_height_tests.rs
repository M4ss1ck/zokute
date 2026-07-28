use crate::config::{load, load_or_create};
use std::fs;
use tempfile::TempDir;

#[test]
fn height_stays_out_of_the_file_until_a_widget_is_resized_vertically() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let config = load_or_create(&path, &[]).unwrap();
    assert!(config.sections.iter().all(|section| section.height.is_none()));
    assert!(!fs::read_to_string(&path).unwrap().contains("height"));
}

#[test]
fn height_round_trips_through_the_config_file() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let mut config = load_or_create(&path, &[]).unwrap();
    config.sections[0].height = Some(275);
    crate::atomic_file::write(&path, &crate::config::serialize(&config)).unwrap();
    assert_eq!(load(&path).unwrap().sections[0].height, Some(275));
}
