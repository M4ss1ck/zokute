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
    let config = load_or_create(&path, &detected).unwrap();
    assert_eq!(config.opacity, 0.73);
    assert_eq!(config.text_color, "#292824");
    assert_eq!(
        config.sections.iter().map(|section| section.id.as_str()).collect::<Vec<_>>(),
        vec!["system", "cpu", "memory", "disk", "network"]
    );
    assert_eq!(config.sections.iter().map(|section| section.enabled).collect::<Vec<_>>(), vec![true, false, true, true, true]);
    assert_eq!(config.sections.iter().map(|section| section.monitor).collect::<Vec<_>>(), vec![2, 2, 2, 2, 2]);
    assert_eq!(config.sections.iter().map(|section| section.x).collect::<Vec<_>>(), vec![11, 11, 11, 11, 11]);
    assert_eq!(config.sections.iter().map(|section| section.y).collect::<Vec<_>>(), vec![7, 223, 383, 487, 647]);
    assert_eq!(config.sections.iter().map(|section| section.width).collect::<Vec<_>>(), vec![500, 500, 500, 500, 500]);
    assert_eq!(config.system_fields, vec!["os", "host", "kernel", "uptime", "packages", "shell", "display", "de", "wm", "wm_theme", "theme", "icons", "font", "cursor", "terminal", "cpu", "gpu", "memory", "swap", "disk", "local_ip", "locale"]);
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
    let config = load_or_create(&path, &["disk-a".to_string()]).unwrap();
    assert_eq!(config.system_fields, vec!["os", "host", "kernel", "uptime", "packages", "shell", "display", "de", "wm", "wm_theme", "theme", "icons", "font", "cursor", "terminal", "cpu", "gpu", "memory", "swap", "disk", "local_ip", "locale"]);
    assert!(config.show_cpu_cores);
    assert_eq!(config.sections.iter().map(|section| section.x).collect::<Vec<_>>(), vec![24, 24, 24, 24, 24]);
    assert_eq!(config.sections.iter().map(|section| section.y).collect::<Vec<_>>(), vec![24, 240, 400, 504, 664]);
    assert_eq!(config.disks.iter().map(|disk| (disk.id.as_str(), disk.enabled, disk.label.as_deref())).collect::<Vec<_>>(), vec![("disk-a", true, None)]);

    let fresh = load_or_create(
        &path,
        &["disk-a".to_string(), "disk-b".to_string()],
    )
    .unwrap();
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
system_fields = ["os", "host", "kernel", "uptime", "packages", "shell", "display", "de", "wm", "wm_theme", "theme", "icons", "font", "cursor", "terminal", "cpu", "gpu", "memory", "swap", "disk", "local_ip", "locale"]
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
    assert!(config.show_background);
    assert_eq!(config.text_color, "#292824");
    assert!(config.sections.iter().all(|section| section.show_header));
    assert_eq!(config.known_sections().iter().map(|section| section.id.as_str()).collect::<Vec<_>>(), vec!["system"]);
    assert!(config.section("custom").is_some());
}

#[test]
fn first_enabled_known_section_skips_unknown_and_disabled_sections() {
    let config = crate::config::Config {
        opacity: 0.92,
        text_opacity: 1.0,
        text_color: "#292824".into(),
        graph_color: None,
        icon_color: None,
        show_background: true,
        sections: vec![
            crate::config::SectionConfig { id: "custom".into(), instance: "custom".into(), enabled: true, show_header: true, monitor: 0, x: 24, y: 24, width: 360, height: None, scale: 1.0, color_mode: None, color_a: None, color_b: None, clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None, date_weekday: true, date_format: None, date_color: None },
            crate::config::SectionConfig { id: "system".into(), instance: "system".into(), enabled: false, show_header: true, monitor: 0, x: 24, y: 24, width: 360, height: None, scale: 1.0, color_mode: None, color_a: None, color_b: None, clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None, date_weekday: true, date_format: None, date_color: None },
            crate::config::SectionConfig { id: "cpu".into(), instance: "cpu".into(), enabled: true, show_header: true, monitor: 0, x: 24, y: 240, width: 360, height: None, scale: 1.0, color_mode: None, color_a: None, color_b: None, clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false, clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None, date_weekday: true, date_format: None, date_color: None },
        ],
        system_fields: vec![],
        show_cpu_cores: true,
        disks: vec![],
    };
    assert_eq!(config.first_enabled_known_section().map(|section| section.id.as_str()), Some("cpu"));
}

#[test]
fn renames_legacy_field_ids_and_adds_the_new_defaults() {
    let source = "opacity = 0.92\nshow_cpu_cores = true\ndisks = []\nsections = []\nsystem_fields = [\"os\", \"desktop\", \"window_manager\"]\n";
    let config = crate::config::parse(source).expect("parse");
    assert_eq!(&config.system_fields[..3], &["os".to_string(), "de".to_string(), "wm".to_string()]);
    assert!(config.system_fields.contains(&"gpu".to_string()));
    assert!(config.system_fields.contains(&"wm_theme".to_string()));
    assert!(!config.system_fields.contains(&"desktop".to_string()));
}

#[test]
fn leaves_a_post_rework_config_alone_so_hidden_fields_stay_hidden() {
    let source = "opacity = 0.92\nshow_cpu_cores = true\ndisks = []\nsections = []\nsystem_fields = [\"os\", \"kernel\"]\n";
    let config = crate::config::parse(source).expect("parse");
    assert_eq!(config.system_fields, vec!["os".to_string(), "kernel".to_string()]);
}
