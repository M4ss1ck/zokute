use crate::config::{load_or_create, serialize_profile};
use crate::config::Profile;
use std::fs;
use tempfile::TempDir;

#[test]
fn height_stays_out_of_the_file_until_a_widget_is_resized_vertically() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let (_config, profile) = load_or_create(&path, &[]).unwrap();
    assert!(profile.sections.iter().all(|section| section.height.is_none()));
    let profile_path = temp.path().join("profiles").join("default.toml");
    assert!(!fs::read_to_string(&profile_path).unwrap().contains("height"));
}

#[test]
fn height_round_trips_through_the_profile_file() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let (_config, mut profile) = load_or_create(&path, &[]).unwrap();
    profile.sections[0].height = Some(275);
    let profile_path = temp.path().join("profiles").join("default.toml");
    crate::atomic_file::write(&profile_path, &serialize_profile(&profile)).unwrap();
    let reparsed: Profile = toml::from_str(&fs::read_to_string(&profile_path).unwrap()).unwrap();
    assert_eq!(reparsed.sections[0].height, Some(275));
}
