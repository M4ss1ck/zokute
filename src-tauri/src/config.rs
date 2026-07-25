use serde::{de::Error as _, Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

#[path = "config_migration.rs"]
mod config_migration;

const KNOWN_SECTION_IDS: [&str; 5] = ["system", "cpu", "memory", "disk", "network"];
const SECTION_Y_OFFSETS: [i32; 5] = [0, 216, 376, 480, 640];
const DEFAULT_SYSTEM_FIELDS: [&str; 12] = [
    "os", "host", "kernel", "uptime", "packages", "shell", "display", "desktop", "window_manager",
    "theme", "terminal", "locale",
];

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub opacity: f64,
    #[serde(default = "default_text_opacity")]
    pub text_opacity: f64,
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
    pub enabled: bool,
    pub monitor: usize,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    #[serde(default = "default_scale")]
    pub scale: f64,
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
        KNOWN_SECTION_IDS.iter().filter_map(|id| self.section(id)).collect()
    }

    pub fn first_enabled_known_section(&self) -> Option<&SectionConfig> {
        KNOWN_SECTION_IDS
            .iter()
            .filter_map(|id| self.section(id))
            .find(|section| section.enabled)
    }

    pub fn section(&self, id: &str) -> Option<&SectionConfig> {
        // Later TOML entries win so explicit edits near the bottom override older values.
        self.sections.iter().rev().find(|section| section.id == id)
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
        let config = fresh(detected_disks);
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
    toml::from_str(source)
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

fn fresh(detected_disks: &[String]) -> Config {
    Config {
        opacity: 0.92,
        text_opacity: default_text_opacity(),
        show_background: default_true(),
        sections: sections(true),
        system_fields: DEFAULT_SYSTEM_FIELDS.iter().map(|field| field.to_string()).collect(),
        show_cpu_cores: true,
        disks: detected_disks.iter().map(|id| DiskPreference { id: id.clone(), enabled: true, label: None }).collect(),
    }
}

fn sections(enabled: bool) -> Vec<SectionConfig> {
    KNOWN_SECTION_IDS
        .iter()
        .zip(SECTION_Y_OFFSETS)
        .map(|(id, y)| section(id, enabled, 0, 24, 24 + y, 360))
        .collect()
}

fn section(id: &str, enabled: bool, monitor: usize, x: i32, y: i32, width: u32) -> SectionConfig {
    SectionConfig { id: id.to_string(), enabled, monitor, x, y, width, scale: default_scale() }
}

fn default_scale() -> f64 { 1.0 }

fn default_text_opacity() -> f64 { 1.0 }

fn default_true() -> bool { true }
