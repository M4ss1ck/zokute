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
fn later_duplicate_sections_win() {
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
    let section = config.section("system").unwrap();
    assert_eq!(section.enabled, false);
    assert_eq!(section.monitor, 1);
    assert_eq!(section.x, 48);
    assert_eq!(section.y, 9);
    assert_eq!(section.width, 444);
    assert_eq!(config.known_sections().len(), 1);
    assert!(config.first_enabled_known_section().is_none());
}
