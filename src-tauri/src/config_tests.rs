use crate::config::{load, load_or_create};
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
    let config = load_or_create(&path, &detected);
    assert_eq!(config.opacity, 0.73);
    assert_eq!(
        config.sections.iter().map(|section| section.id.as_str()).collect::<Vec<_>>(),
        vec!["system", "cpu", "memory", "disk", "network"]
    );
    assert_eq!(config.sections.iter().map(|section| section.enabled).collect::<Vec<_>>(), vec![true, false, true, true, true]);
    assert_eq!(config.sections.iter().map(|section| section.monitor).collect::<Vec<_>>(), vec![2, 2, 2, 2, 2]);
    assert_eq!(config.sections.iter().map(|section| section.x).collect::<Vec<_>>(), vec![11, 11, 11, 11, 11]);
    assert_eq!(config.sections.iter().map(|section| section.y).collect::<Vec<_>>(), vec![7, 223, 383, 487, 647]);
    assert_eq!(config.sections.iter().map(|section| section.width).collect::<Vec<_>>(), vec![500, 500, 500, 500, 500]);
    assert_eq!(config.system_fields, vec!["os", "host", "kernel", "uptime", "packages", "shell", "display", "desktop", "window_manager", "theme", "terminal", "locale"]);
    assert!(config.show_cpu_cores);
    assert_eq!(config.disks.iter().map(|disk| disk.id.as_str()).collect::<Vec<_>>(), vec!["disk-a", "disk-b"]);
    assert!(config.disks.iter().all(|disk| disk.enabled && disk.label.is_none()));
    let source = fs::read_to_string(&path).unwrap();
    assert!(!source.contains("height"));
    assert!(!source.contains("temperatures"));
}

#[test]
fn fresh_config_enables_initial_disks_and_keeps_new_detections_disabled() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let config = load_or_create(&path, &["disk-a".to_string()]);
    assert_eq!(config.system_fields, vec!["os", "host", "kernel", "uptime", "packages", "shell", "display", "desktop", "window_manager", "theme", "terminal", "locale"]);
    assert!(config.show_cpu_cores);
    assert_eq!(config.disks.iter().map(|disk| (disk.id.as_str(), disk.enabled, disk.label.as_deref())).collect::<Vec<_>>(), vec![("disk-a", true, None)]);

    let fresh = load_or_create(
        &path,
        &["disk-a".to_string(), "disk-b".to_string()],
    );
    assert!(fresh.disk_preference("disk-b").is_none());
}

#[test]
fn malformed_new_config_errors() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    write(&path, "opacity = 0.92\nsections = [");
    assert!(load(&path).is_err());
}

#[test]
fn unknown_sections_deserialize_but_do_not_join_known_reconciliation() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    write(
        &path,
        r#"
opacity = 0.92
system_fields = ["os", "host", "kernel", "uptime", "packages", "shell", "display", "desktop", "window_manager", "theme", "terminal", "locale"]
show_cpu_cores = true
disks = []

[[sections]]
id = "system"
enabled = true
monitor = 0
x = 24
y = 0
width = 360

[[sections]]
id = "custom"
enabled = true
monitor = 0
x = 24
y = 1
width = 111
"#,
    );
    let config = load(&path).unwrap();
    assert_eq!(config.known_sections().iter().map(|section| section.id.as_str()).collect::<Vec<_>>(), vec!["system"]);
    assert!(config.section("custom").is_some());
}
