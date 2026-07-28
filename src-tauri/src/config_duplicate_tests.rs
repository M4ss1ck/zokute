use crate::config::Profile;
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
    let path = temp.path().join("profile.toml");
    write(
        &path,
        r#"
profile_schema_version = 1
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
    let source = fs::read_to_string(&path).unwrap();
    let mut profile: Profile = toml::from_str(&source).unwrap();
    profile = crate::config::normalize_profile_instances(profile);
    let first = profile.section("system").unwrap();
    let second = profile.section("system-2").unwrap();
    assert!(first.enabled);
    assert_eq!(second.monitor, 1);
    assert_eq!(second.x, 48);
    assert_eq!(second.y, 9);
    assert_eq!(second.width, 444);
    assert_eq!(profile.known_sections().len(), 2);
    assert!(profile.first_enabled_known_section().is_some());
}
