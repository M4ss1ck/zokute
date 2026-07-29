use crate::paths::{config_location, config_path};

#[test]
fn the_config_location_is_the_real_config_file() {
    assert_eq!(config_location(), config_path().to_string_lossy());
}

#[test]
fn the_config_location_is_never_empty() {
    // Onboarding renders this straight into "Your config is at:".
    assert!(!config_location().is_empty());
    assert!(config_location().ends_with("zokute.toml"));
}
