use std::env;
use std::path::PathBuf;

pub fn config_dir() -> PathBuf {
    if let Ok(home) = env::var("XDG_CONFIG_HOME") {
        PathBuf::from(home).join("zokute")
    } else {
        PathBuf::from(env::var("HOME").unwrap_or_default()).join(".config/zokute")
    }
}

pub fn config_path() -> PathBuf {
    config_dir().join("zokute.toml")
}

/// The config file path as text, for the onboarding "Your config is at" line
/// and any other display use.
pub fn config_location() -> String {
    config_path().to_string_lossy().to_string()
}

#[tauri::command]
pub fn config_location_cmd() -> String {
    config_location()
}

pub fn legacy_config_path() -> PathBuf {
    config_dir().join("config.toml")
}

pub fn state_dir() -> PathBuf {
    if let Ok(home) = env::var("XDG_STATE_HOME") {
        PathBuf::from(home).join("zokute")
    } else {
        PathBuf::from(env::var("HOME").unwrap_or_default()).join(".local/state/zokute")
    }
}

pub fn backups_dir() -> PathBuf {
    state_dir().join("backups")
}

pub fn settings_window_path() -> PathBuf {
    state_dir().join("settings-window.toml")
}

pub fn profiles_dir() -> PathBuf {
    config_dir().join("profiles")
}

pub fn profile_path(name: &str) -> PathBuf {
    profiles_dir().join(format!("{name}.toml"))
}

pub fn plugins_dir() -> PathBuf {
    config_dir().join("plugins")
}
