use crate::config::load;
use std::{fs, path::Path};
use tempfile::TempDir;

fn write(path: &Path, source: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, source).unwrap();
}

#[test]
fn duplicate_widget_types_receive_distinct_instance_labels() {
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
id = "system"
enabled = false
monitor = 1
x = 48
y = 9
width = 444
"#,
    );
    let config = load(&path).unwrap();
    let first = config.section("system").unwrap();
    let second = config.section("system-2").unwrap();
    assert!(first.enabled);
    assert_eq!(second.monitor, 1);
    assert_eq!(second.x, 48);
    assert_eq!(second.y, 9);
    assert_eq!(second.width, 444);
    assert_eq!(config.known_sections().len(), 2);
    assert!(config.first_enabled_known_section().is_some());
}
