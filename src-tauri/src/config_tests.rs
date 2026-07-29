use crate::config::{load, load_or_create, parse};
use std::collections::BTreeMap;
use std::{fs, path::Path};
use tempfile::TempDir;
fn write(path: &Path, source: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, source).unwrap();
}
#[test]
fn legacy_config_migrates_sections_and_disks() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    write(
        &path,
        r#"
monitor = 2
x = 11
y = 7
width = 500
height = 900
opacity = 0.73
widgets = ["system", "memory", "disk", "network", "temperatures"]
"#,
    );
    let detected = vec!["disk-a".to_string(), "disk-b".to_string()];
    let (config, profile) = load_or_create(&path, &detected).unwrap();
    assert_eq!(config.opacity, 0.73);
    assert_eq!(config.text_color, "#292824");
    // The widget list the user had chosen must survive the upgrade, along with
    // the geometry it was placed at (roadmap rule 4).
    let enabled: Vec<_> = profile.sections.iter().filter(|s| s.enabled).map(|s| s.id.as_str()).collect();
    assert_eq!(enabled, vec!["system", "memory", "disk", "network"]);
    assert!(!profile.sections.iter().any(|s| s.enabled && s.id == "cpu"), "cpu was not in the legacy widget list");
    let system = profile.section("system").expect("system section");
    assert_eq!(system.monitor, 2);
    assert_eq!(system.x, 11);
    assert_eq!(system.width, 500);
    assert!(!profile.system_fields.is_empty());
    assert!(profile.show_cpu_cores);
    let source = fs::read_to_string(&path).unwrap();
    assert!(!source.contains("height"));
    assert!(!source.contains("temperatures"));
}
#[test]
fn fresh_config_enables_initial_disks_and_keeps_new_detections_disabled() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let (_config, profile) = load_or_create(&path, &["disk-a".to_string()]).unwrap();
    assert_eq!(profile.system_fields, vec!["os", "host", "kernel", "uptime", "packages", "shell", "display", "de", "wm", "wm_theme", "theme", "icons", "font", "cursor", "terminal", "cpu", "gpu", "memory", "swap", "disk", "local_ip", "locale"]);
    assert!(profile.show_cpu_cores);
    assert_eq!(profile.disks.iter().map(|disk| (disk.id.as_str(), disk.enabled, disk.label.as_deref())).collect::<Vec<_>>(), vec![("disk-a", true, None)]);
    let (_fresh_config, fresh_profile) = load_or_create(
        &path,
        &["disk-a".to_string(), "disk-b".to_string()],
    )
    .unwrap();
    assert!(fresh_profile.disk_preference("disk-b").is_none());
}
#[test]
fn repairs_disk_preferences_written_by_the_broken_onboarding_preset() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let area = crate::onboarding::WorkArea {
        identity: "monitor-0".into(), width: 1920, height: 1080,
    };
    let (config, profile) = crate::onboarding_presets::snapshot_for(
        "system_monitor", "light", &area, &[],
    );
    write(&path, &crate::config::serialize(&config));
    write(
        &temp.path().join("profiles/default.toml"),
        &crate::config::serialize_profile(&profile),
    );
    let (_, repaired) = load_or_create(&path, &["uuid:root".to_string()]).unwrap();
    assert!(repaired.disk_preference("uuid:root").is_some_and(|disk| disk.enabled));
}
#[test]
fn repairs_disk_preferences_in_a_customized_profile() {
    // A profile whose widgets have been edited since onboarding still needs its
    // empty disk list seeded: the disk widget renders nothing without it, and
    // the settings pane has no rows to switch it back on.
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let area = crate::onboarding::WorkArea {
        identity: "monitor-0".into(), width: 1920, height: 1080,
    };
    let (config, mut profile) = crate::onboarding_presets::snapshot_for(
        "system_monitor", "light", &area, &[],
    );
    profile.sections.retain(|section| section.id != "cpu");
    write(&path, &crate::config::serialize(&config));
    write(
        &temp.path().join("profiles/default.toml"),
        &crate::config::serialize_profile(&profile),
    );
    let (_, repaired) = load_or_create(&path, &["uuid:root".to_string()]).unwrap();
    assert!(repaired.disk_preference("uuid:root").is_some_and(|disk| disk.enabled));
}
#[test]
fn malformed_new_config_errors() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    write(&path, "opacity = 0.92\nsections = [");
    assert!(load(&path).is_err());
}
#[test]
fn v3_config_parses_without_sections() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    write(
        &path,
        r#"
schema_version = 3
active_profile = "default"
opacity = 0.92
theme = "light"
"#,
    );
    let config = load(&path).unwrap();
    assert!(config.show_background);
    assert_eq!(config.text_color, "#292824");
    assert_eq!(config.active_profile, "default");
}
#[test]
fn renames_legacy_field_ids_and_adds_the_new_defaults() {
    // system_fields upgrade now operates on Profile
    let source = "profile_schema_version = 1\nshow_cpu_cores = true\ndisks = []\nsections = []\nsystem_fields = [\"os\", \"desktop\", \"window_manager\"]\n";
    let profile: crate::config::Profile = toml::from_str(source).expect("parse");
    let upgraded = crate::config::config_fields::upgrade_system_fields(profile);
    assert_eq!(&upgraded.system_fields[..3], &["os".to_string(), "de".to_string(), "wm".to_string()]);
    assert!(upgraded.system_fields.contains(&"gpu".to_string()));
    assert!(upgraded.system_fields.contains(&"wm_theme".to_string()));
    assert!(!upgraded.system_fields.contains(&"desktop".to_string()));
}
#[test]
fn leaves_a_post_rework_config_alone_so_hidden_fields_stay_hidden() {
    let source = "profile_schema_version = 1\nshow_cpu_cores = true\ndisks = []\nsections = []\nsystem_fields = [\"os\", \"kernel\"]\n";
    let profile: crate::config::Profile = toml::from_str(source).expect("parse");
    let upgraded = crate::config::config_fields::upgrade_system_fields(profile);
    assert_eq!(upgraded.system_fields, vec!["os".to_string(), "kernel".to_string()]);
}
