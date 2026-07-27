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
fn sections_without_color_fields_still_parse() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    write(&path, "opacity = 0.92\nsystem_fields = []\nshow_cpu_cores = true\ndisks = []\n\n[[sections]]\nid = \"cpu\"\nenabled = true\nmonitor = 0\nx = 24\ny = 0\nwidth = 360\n");
    let config = load(&path).unwrap();
    let section = config.section("cpu").unwrap();
    assert!(section.color_mode.is_none());
    assert!(section.color_a.is_none());
    assert!(section.color_b.is_none());
    assert!(section.gradient_direction.is_none());
}

#[test]
fn visualizer_sections_are_known_and_carry_colors() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    write(&path, "opacity = 0.92\nsystem_fields = []\nshow_cpu_cores = true\ndisks = []\n\n[[sections]]\nid = \"spectrum\"\nenabled = true\nmonitor = 0\nx = 0\ny = 900\nwidth = 1920\ncolor_mode = \"gradient\"\ncolor_a = \"#c07100\"\ncolor_b = \"#2563eb\"\ngradient_direction = \"vertical\"\n");
    let config = load(&path).unwrap();
    assert_eq!(config.known_sections().iter().map(|section| section.id.as_str()).collect::<Vec<_>>(), vec!["spectrum"]);
    let section = config.section("spectrum").unwrap();
    assert_eq!(section.color_mode.as_deref(), Some("gradient"));
    assert_eq!(section.color_a.as_deref(), Some("#c07100"));
    assert_eq!(section.color_b.as_deref(), Some("#2563eb"));
    assert_eq!(section.gradient_direction.as_deref(), Some("vertical"));
}

#[test]
fn a_fresh_config_contains_no_visualizers() {
    let temp = TempDir::new().unwrap();
    let path = temp.path().join("config.toml");
    let config = load_or_create(&path, &[]).unwrap();
    assert_eq!(config.sections.iter().map(|section| section.id.as_str()).collect::<Vec<_>>(), vec!["system", "cpu", "memory", "disk", "network"]);
}
