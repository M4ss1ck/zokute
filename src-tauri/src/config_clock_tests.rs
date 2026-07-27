use crate::config::parse;
use crate::config_write::sanitize;

const CLOCK_WITHOUT_KEYS: &str = r#"
opacity = 0.9
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
    let config = parse(CLOCK_WITHOUT_KEYS).unwrap();
    let section = &config.sections[0];
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
    let mut config = parse(CLOCK_WITHOUT_KEYS).unwrap();
    config.sections[0].clock_font = Some("sans".to_string());
    config.sections[0].clock_seconds = true;
    config.sections[0].clock_layout = Some("column".to_string());
    let reparsed = parse(&crate::config::serialize(&config)).unwrap();
    assert_eq!(reparsed.sections[0].clock_font.as_deref(), Some("sans"));
    assert!(reparsed.sections[0].clock_seconds);
    assert_eq!(reparsed.sections[0].clock_layout.as_deref(), Some("column"));
}

#[test]
fn sanitize_drops_a_malformed_clock_color_and_keeps_a_valid_one() {
    let mut config = parse(CLOCK_WITHOUT_KEYS).unwrap();
    config.sections[0].clock_color = Some("red".to_string());
    assert_eq!(sanitize(config).sections[0].clock_color, None);

    let mut config = parse(CLOCK_WITHOUT_KEYS).unwrap();
    config.sections[0].clock_color = Some("#c07100".to_string());
    assert_eq!(sanitize(config).sections[0].clock_color.as_deref(), Some("#c07100"));
}

#[test]
fn sanitize_keeps_clock_sections() {
    let config = parse(CLOCK_WITHOUT_KEYS).unwrap();
    assert_eq!(sanitize(config).sections.len(), 1);
}
