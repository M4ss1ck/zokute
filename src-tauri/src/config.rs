use serde::{de::Error as _, Deserialize, Serialize};
use std::{env, fs, path::PathBuf};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Config {
    pub monitor: usize,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub opacity: f64,
    pub widgets: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            monitor: 0,
            x: 24,
            y: 24,
            width: 360,
            height: 760,
            opacity: 0.92,
            widgets: vec!["system", "cpu", "memory", "disk", "network", "temperatures"]
                .into_iter()
                .map(str::to_string)
                .collect(),
        }
    }
}

pub fn path() -> PathBuf {
    if let Ok(home) = env::var("XDG_CONFIG_HOME") {
        return PathBuf::from(home).join("zokute/config.toml");
    }
    PathBuf::from(env::var("HOME").expect("HOME")).join(".config/zokute/config.toml")
}

pub fn load_or_create() -> Config {
    let path = path();
    if !path.exists() {
        let config = Config::default();
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let toml = toml::to_string_pretty(&config).expect("default config");
        let _ = fs::write(&path, toml);
        return config;
    }
    if let Ok(config) = load(&path) {
        return config;
    }
    Config::default()
}

pub fn load(path: &PathBuf) -> Result<Config, toml::de::Error> {
    let source = fs::read_to_string(path).map_err(toml::de::Error::custom)?;
    toml::from_str(&source)
}
