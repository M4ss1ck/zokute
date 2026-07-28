use crate::config;

fn parse_example(name: &str) -> Result<config::Config, String> {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docs")
        .join("examples")
        .join(name);
    let source = std::fs::read_to_string(&path)
        .map_err(|e| format!("read {name}: {e}"))?;
    config::parse(&source).map_err(|e| format!("parse {name}: {e}"))
}

#[test]
fn minimal_example_parses() {
    let config = parse_example("minimal-config.toml").unwrap();
    assert_eq!(config.schema_version, 3);
    assert_eq!(config.opacity, 0.92);
    assert_eq!(config.theme, "dark");
}

#[test]
fn performance_panel_example_parses() {
    let config = parse_example("performance-panel.toml").unwrap();
    assert_eq!(config.opacity, 0.85);
    assert_eq!(config.density, "compact");
    assert_eq!(config.byte_format, "decimal");
}

#[test]
fn gaming_example_parses() {
    let config = parse_example("gaming.toml").unwrap();
    assert_eq!(config.opacity, 0.7);
    assert!(!config.show_background);
    assert_eq!(config.accent_color.as_deref(), Some("#ff4444"));
}

#[test]
fn multi_monitor_example_parses() {
    let config = parse_example("multi-monitor.toml").unwrap();
    assert_eq!(config.schema_version, 3);
    assert_eq!(config.active_profile, "multi");
}

#[test]
fn dark_transparent_example_parses() {
    let config = parse_example("dark-transparent.toml").unwrap();
    assert_eq!(config.opacity, 0.6);
    assert!(!config.show_background);
}

#[test]
fn fullscreen_hide_example_parses() {
    let config = parse_example("fullscreen-hide.toml").unwrap();
    assert_eq!(config.opacity, 0.92);
    assert_eq!(config.theme, "dark");
}
