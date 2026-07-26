use serde::{de::Error as _, Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};
#[path = "config_migration.rs"]
mod config_migration;
#[path = "config_defaults.rs"]
mod config_defaults;
const KNOWN_SECTION_IDS: [&str; 7] = ["system", "cpu", "memory", "disk", "network", "spectrum", "ring"];
pub(crate) const DEFAULT_SYSTEM_FIELDS: [&str; 12] = [
    "os", "host", "kernel", "uptime", "packages", "shell", "display", "desktop", "window_manager",
    "theme", "terminal", "locale",
];
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub opacity: f64,
    #[serde(default = "default_text_opacity")]
    pub text_opacity: f64,
    #[serde(default = "default_text_color")]
    pub text_color: String,
    #[serde(default)] pub graph_color: Option<String>,
    #[serde(default)] pub icon_color: Option<String>,
    #[serde(default = "default_true")]
    pub show_background: bool,
    pub sections: Vec<SectionConfig>,
    pub system_fields: Vec<String>,
    pub show_cpu_cores: bool,
    pub disks: Vec<DiskPreference>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectionConfig {
    pub id: String,
    #[serde(default)]
    pub instance: String,
    pub enabled: bool,
    #[serde(default = "default_true")] pub show_header: bool,
    pub monitor: usize,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    // Absent means "whatever the content needs", until a vertical drag sets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default)]
    pub color_mode: Option<String>,
    #[serde(default)]
    pub color_a: Option<String>,
    #[serde(default)]
    pub color_b: Option<String>,
}
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DiskPreference {
    pub id: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}
impl Config {
    pub fn known_sections(&self) -> Vec<&SectionConfig> {
        self.sections
            .iter()
            .filter(|section| KNOWN_SECTION_IDS.contains(&section.id.as_str()))
            .collect()
    }

    pub fn first_enabled_known_section(&self) -> Option<&SectionConfig> {
        self.known_sections().into_iter().find(|section| section.enabled)
    }

    pub fn section(&self, instance: &str) -> Option<&SectionConfig> {
        self.sections.iter().find(|section| section.instance == instance)
    }

    pub fn disk_preference(&self, id: &str) -> Option<&DiskPreference> {
        self.disks.iter().find(|disk| disk.id == id)
    }
}

pub fn path() -> PathBuf {
    if let Ok(home) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(home).join("zokute/config.toml");
    }
    PathBuf::from(env::var("HOME").expect("HOME")).join(".config/zokute/config.toml")
}

pub fn load_or_create(path: &Path, detected_disks: &[String]) -> std::io::Result<Config> {
    if !path.exists() {
        let config = config_defaults::fresh(detected_disks);
        write(path, &config)?;
        return Ok(config);
    }
    let source = fs::read_to_string(path)?;
    if let Ok(config) = parse(&source) {
        return Ok(config);
    }
    if let Ok(config) = config_migration::migrate(&source, detected_disks) {
        write(path, &config)?;
        return Ok(config);
    }
    panic!("invalid config");
}

pub fn load(path: &Path) -> Result<Config, toml::de::Error> {
    parse(&fs::read_to_string(path).map_err(toml::de::Error::custom)?)
}

pub fn parse(source: &str) -> Result<Config, toml::de::Error> {
    toml::from_str(source).map(normalize_instances)
}

pub fn serialize(config: &Config) -> String {
    toml::to_string_pretty(config).expect("config")
}

pub fn write(path: &Path, config: &Config) -> std::io::Result<()> {
    write_str(path, &serialize(config))
}

pub fn write_str(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temp = path.with_extension("toml.tmp");
    fs::write(&temp, contents)?;
    fs::rename(&temp, path)
}

pub fn normalize_instances(mut config: Config) -> Config {
    let mut labels: Vec<String> = Vec::new();
    for section in &mut config.sections {
        let base = if section.instance.is_empty() { &section.id } else { &section.instance };
        let mut label = base.clone();
        let mut suffix = 2;
        while labels.contains(&label) {
            label = format!("{base}-{suffix}");
            suffix += 1;
        }
        section.instance = label.clone();
        labels.push(label);
    }
    config
}

fn default_scale() -> f64 { 1.0 }
fn default_text_opacity() -> f64 { 1.0 }
fn default_text_color() -> String { "#292824".into() }
fn default_true() -> bool { true }
