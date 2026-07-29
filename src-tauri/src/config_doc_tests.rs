use crate::config;
use crate::config::Profile;

fn read_example(name: &str) -> String {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap().join("docs").join("examples").join(name);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {name}: {e}"))
}

fn parse_example(name: &str) -> config::Config {
    config::parse(&read_example(name)).unwrap_or_else(|e| panic!("parse {name}: {e}"))
}

fn parse_profile(name: &str) -> Profile {
    toml::from_str(&read_example(name)).unwrap_or_else(|e| panic!("parse {name}: {e}"))
}

#[test]
fn minimal_example_parses() {
    let config = parse_example("minimal-config.toml");
    assert_eq!(config.schema_version, 3);
    assert_eq!(config.opacity, 0.92);
    assert_eq!(config.theme, "dark");
}

#[test]
fn performance_panel_example_parses() {
    let config = parse_example("performance-panel.toml");
    assert_eq!(config.density, "compact");
    assert_eq!(config.byte_format, "decimal");
}

#[test]
fn gaming_example_parses() {
    let config = parse_example("gaming.toml");
    assert!(!config.show_background);
    assert_eq!(config.accent_color.as_deref(), Some("#ff4444"));
}

#[test]
fn dark_transparent_example_parses() {
    let config = parse_example("dark-transparent.toml");
    assert_eq!(config.opacity, 0.6);
    assert!(!config.show_background);
}

// M10 Task 5.2: each example must actually demonstrate its subject.
#[test]
fn the_performance_panel_example_contains_a_panel() {
    let profile = parse_profile("performance-panel-profile.toml");
    let panel = profile.sections.iter().find(|s| s.id == "panel").expect("a panel section");
    assert!(!panel.children.is_empty(), "a panel example needs children");
}

#[test]
fn the_multi_monitor_example_places_widgets_on_more_than_one_monitor() {
    let profile = parse_profile("multi-monitor-profile.toml");
    let monitors: std::collections::BTreeSet<_> = profile.sections.iter().map(|s| s.monitor).collect();
    assert!(monitors.len() > 1, "a multi-monitor example must span monitors, saw {monitors:?}");
}

#[test]
fn the_fullscreen_hide_example_configures_the_fullscreen_policy() {
    let profile = parse_profile("fullscreen-hide-profile.toml");
    assert_eq!(profile.fullscreen.behavior, "hide");
    assert_eq!(profile.fullscreen.enter_delay_ms, 150);
}

#[test]
fn the_maibuk_example_configures_the_plugin_widget() {
    let profile = parse_profile("maibuk-profile.toml");
    let section = profile.sections.iter().find(|s| s.plugin_id.is_some()).expect("a plugin section");
    assert_eq!(section.plugin_id.as_deref(), Some("maibuk-notes"));
    assert!(section.plugin_interval >= 2, "plugin interval must respect the manifest floor");
}

#[test]
fn the_interactive_action_example_enables_interaction() {
    let profile = parse_profile("interactive-action-profile.toml");
    assert!(profile.sections.iter().any(|s| s.interactive), "an interaction example needs an interactive section");
}

#[test]
fn the_gaming_example_dims_rather_than_hiding() {
    let profile = parse_profile("gaming-profile.toml");
    assert_eq!(profile.fullscreen.behavior, "dim");
    assert_eq!(profile.fullscreen.dim_opacity, 0.25);
}

#[test]
fn every_profile_example_holds_unique_instance_ids() {
    for name in ["performance-panel-profile.toml", "multi-monitor-profile.toml",
                 "fullscreen-hide-profile.toml", "maibuk-profile.toml",
                 "interactive-action-profile.toml", "gaming-profile.toml"] {
        let profile = parse_profile(name);
        let mut instances: Vec<_> = profile.sections.iter().map(|s| s.instance.clone()).collect();
        let total = instances.len();
        instances.sort();
        instances.dedup();
        assert_eq!(instances.len(), total, "{name} has duplicate instance ids");
    }
}
