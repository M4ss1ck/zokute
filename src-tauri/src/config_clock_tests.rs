use crate::config::Profile;
use crate::config_write::sanitize_profile;

const CLOCK_WITHOUT_KEYS: &str = r#"
profile_schema_version = 1
system_fields = []
show_cpu_cores = true
disks = []

[[sections]]
id = "clock"
instance = "clock"
enabled = true
monitor = 0
x = 0
y = 0
width = 360
"#;

#[test]
fn a_section_without_clock_keys_takes_the_documented_defaults() {
    let profile: Profile = toml::from_str(CLOCK_WITHOUT_KEYS).unwrap();
    let section = &profile.sections[0];
    assert_eq!(section.clock_font, None);
    assert_eq!(section.clock_color, None);
    assert!(!section.clock_seconds);
    assert!(!section.clock_24h);
    assert!(section.clock_ampm);
    assert!(section.clock_pad);
    assert_eq!(section.clock_layout, None);
}

#[test]
fn clock_keys_round_trip_through_serialization() {
    let mut profile: Profile = toml::from_str(CLOCK_WITHOUT_KEYS).unwrap();
    profile.sections[0].clock_font = Some("sans".to_string());
    profile.sections[0].clock_seconds = true;
    profile.sections[0].clock_layout = Some("column".to_string());
    let reparsed: Profile = toml::from_str(&crate::config::serialize_profile(&profile)).unwrap();
    assert_eq!(reparsed.sections[0].clock_font.as_deref(), Some("sans"));
    assert!(reparsed.sections[0].clock_seconds);
    assert_eq!(reparsed.sections[0].clock_layout.as_deref(), Some("column"));
}

#[test]
fn sanitize_drops_a_malformed_clock_color_and_keeps_a_valid_one() {
    let mut profile: Profile = toml::from_str(CLOCK_WITHOUT_KEYS).unwrap();
    profile.sections[0].clock_color = Some("red".to_string());
    assert_eq!(sanitize_profile(profile).sections[0].clock_color, None);

    let mut profile: Profile = toml::from_str(CLOCK_WITHOUT_KEYS).unwrap();
    profile.sections[0].clock_color = Some("#c07100".to_string());
    assert_eq!(sanitize_profile(profile).sections[0].clock_color.as_deref(), Some("#c07100"));
}

#[test]
fn sanitize_keeps_clock_sections() {
    let profile: Profile = toml::from_str(CLOCK_WITHOUT_KEYS).unwrap();
    assert_eq!(sanitize_profile(profile).sections.len(), 1);
}
