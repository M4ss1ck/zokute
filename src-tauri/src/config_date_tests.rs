use crate::config::parse;
use crate::config_write::sanitize;

const DATE_WITHOUT_KEYS: &str = r#"
opacity = 0.9
system_fields = []
show_cpu_cores = true
disks = []

[[sections]]
id = "date"
instance = "date"
enabled = true
monitor = 0
x = 0
y = 0
width = 360
"#;

#[test]
fn a_section_without_date_keys_takes_the_documented_defaults() {
    let config = parse(DATE_WITHOUT_KEYS).unwrap();
    let section = &config.sections[0];
    assert!(section.date_weekday);
    assert_eq!(section.date_format, None);
    assert_eq!(section.date_color, None);
}

#[test]
fn date_keys_round_trip_through_serialization() {
    let mut config = parse(DATE_WITHOUT_KEYS).unwrap();
    config.sections[0].date_weekday = false;
    config.sections[0].date_format = Some("numeric".to_string());
    let reparsed = parse(&crate::config::serialize(&config)).unwrap();
    assert!(!reparsed.sections[0].date_weekday);
    assert_eq!(reparsed.sections[0].date_format.as_deref(), Some("numeric"));
}

#[test]
fn sanitize_drops_a_malformed_date_color_and_keeps_a_valid_one() {
    let mut config = parse(DATE_WITHOUT_KEYS).unwrap();
    config.sections[0].date_color = Some("red".to_string());
    assert_eq!(sanitize(config).sections[0].date_color, None);

    let mut config = parse(DATE_WITHOUT_KEYS).unwrap();
    config.sections[0].date_color = Some("#c07100".to_string());
    assert_eq!(
        sanitize(config).sections[0].date_color.as_deref(),
        Some("#c07100")
    );
}

#[test]
fn sanitize_keeps_date_sections() {
    let config = parse(DATE_WITHOUT_KEYS).unwrap();
    assert_eq!(sanitize(config).sections.len(), 1);
}
