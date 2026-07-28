use crate::{atomic_file, config::{self, Config, Profile}, config_error::ConfigError, config_validate, paths};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct RecoveryInfo {
    pub message: String,
    pub config_path: String,
    pub backups_dir: String,
}

pub fn info_from(error: &ConfigError) -> RecoveryInfo {
    RecoveryInfo {
        message: error.to_string(),
        config_path: paths::config_path().to_string_lossy().to_string(),
        backups_dir: paths::backups_dir().to_string_lossy().to_string(),
    }
}

pub fn restore_backup(detected_disks: &[String]) -> Result<(Config, Profile), ConfigError> {
    let config_path = paths::config_path();
    let backup = paths::backups_dir().join("zokute.toml.previous");
    if !backup.exists() {
        return Err(ConfigError::Migration("no backup found".into()));
    }
    let source = std::fs::read_to_string(&backup)?;
    // Try parsing as v3 first
    if let Ok(config) = config::parse(&source) {
        config_validate::check(&config)?;
        let profile = load_or_fresh_profile(&config, detected_disks);
        backup_invalid_if_exists(&config_path);
        atomic_file::write(&config_path, &config::serialize(&config))?;
        return Ok((config, profile));
    }
    // Try v2 migration
    if let Ok((config, profile)) = crate::config::profile_migration::migrate_v2_source(&source, detected_disks) {
        config_validate::check(&config)?;
        backup_invalid_if_exists(&config_path);
        atomic_file::write(&config_path, &config::serialize(&config))?;
        let _ = std::fs::create_dir_all(paths::profiles_dir());
        atomic_file::write(&paths::profile_path(&config.active_profile), &config::serialize_profile(&profile))?;
        return Ok((config, profile));
    }
    Err(ConfigError::Parse("unable to parse backup".into()))
}

fn load_or_fresh_profile(config: &Config, detected_disks: &[String]) -> Profile {
    let profile_path = paths::profile_path(&config.active_profile);
    if profile_path.exists() {
        std::fs::read_to_string(&profile_path)
            .ok()
            .and_then(|source| toml::from_str::<Profile>(&source).ok())
            .unwrap_or_else(|| config::fresh_profile(detected_disks))
    } else {
        config::fresh_profile(detected_disks)
    }
}

pub fn use_defaults(detected_disks: &[String]) -> (Config, Profile) {
    let config_path = paths::config_path();
    backup_invalid_if_exists(&config_path);
    let config = config::fresh_defaults(detected_disks);
    let profile = config::fresh_profile(detected_disks);
    let _ = atomic_file::write(&config_path, &config::serialize(&config));
    let _ = std::fs::create_dir_all(paths::profiles_dir());
    let _ = atomic_file::write(&paths::profile_path(&config.active_profile), &config::serialize_profile(&profile));
    (config, profile)
}

pub fn use_in_memory_defaults(detected_disks: &[String]) -> (Config, Profile) {
    (config::fresh_defaults(detected_disks), config::fresh_profile(detected_disks))
}

pub fn revalidate() -> Result<(), ConfigError> {
    let config_path = paths::config_path();
    let source = std::fs::read_to_string(&config_path)?;
    let config: Config = toml::from_str(&source)
        .map_err(|e| ConfigError::Parse(e.to_string()))?;
    config_validate::check(&config)
}

fn backup_invalid_if_exists(path: &std::path::Path) {
    if !path.exists() { return; }
    let content = std::fs::read_to_string(path).ok();
    if content.as_deref() == Some(&config::serialize(&config::fresh_defaults(&[]))) { return; }
    let backups = paths::backups_dir();
    let _ = std::fs::create_dir_all(&backups);
    let ts = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    let name = path.file_name().unwrap_or_default();
    let dest = backups.join(format!("{}.invalid-{ts}", name.to_string_lossy()));
    let _ = std::fs::copy(path, &dest);
}
