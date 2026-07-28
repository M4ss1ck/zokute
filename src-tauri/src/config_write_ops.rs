use super::{Config, Profile, config_defaults, config_migration, config_validate, profile_migration, profile_mod, resolve_profile_path};
use crate::{atomic_file, config_error::ConfigError, monitor::MonitorCatalog, paths};
use serde::de::Error as _;
use std::collections::BTreeMap;
use std::fs;

pub fn load_profile(config_path: &std::path::Path, config: &Config, detected_disks: &[String]) -> Result<Profile, ConfigError> {
    let profile_path = resolve_profile_path(config_path, &config.active_profile);
    if profile_path.exists() {
        let source = fs::read_to_string(&profile_path)?;
        let profile: Profile = toml::from_str(&source)
            .map_err(|e| ConfigError::Parse(e.to_string()))?;
        config_validate::check_profile(&profile)?;
        Ok(profile)
    } else {
        let profile = config_defaults::fresh_profile(detected_disks);
        let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
        atomic_file::write(&profile_path, &serialize_profile(&profile))?;
        Ok(profile)
    }
}

pub fn create_or_migrate_legacy(new_path: &std::path::Path, detected_disks: &[String]) -> Result<(Config, Profile), ConfigError> {
    let legacy = paths::legacy_config_path();
    if legacy.exists() {
        let source = fs::read_to_string(&legacy)?;
        if let Ok((config, profile)) = profile_migration::migrate_v2_source(&source, detected_disks) {
            atomic_file::backup_previous(&legacy, &paths::backups_dir())?;
            atomic_file::write(new_path, &serialize(&config))?;
            let profile_path = resolve_profile_path(new_path, &config.active_profile);
            let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
            atomic_file::write(&profile_path, &serialize_profile(&profile))?;
            let _ = fs::remove_file(&legacy);
            return Ok((config, profile));
        }
        if let Ok((config, data)) = config_migration::migrate_with_data(&source, detected_disks) {
            let profile = Profile {
                profile_schema_version: profile_mod::PROFILE_SCHEMA_VERSION,
                sections: data.sections, system_fields: data.system_fields,
                show_cpu_cores: data.show_cpu_cores, disks: data.disks,
                monitor_catalog: MonitorCatalog::new(), extra: BTreeMap::new(),
            };
            atomic_file::backup_previous(&legacy, &paths::backups_dir())?;
            atomic_file::write(new_path, &serialize(&config))?;
            let profile_path = resolve_profile_path(new_path, &config.active_profile);
            let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
            atomic_file::write(&profile_path, &serialize_profile(&profile))?;
            let _ = fs::remove_file(&legacy);
            return Ok((config, profile));
        }
        if let Ok(config) = parse(&source) {
            let profile = config_defaults::fresh_profile(detected_disks);
            atomic_file::backup_previous(&legacy, &paths::backups_dir())?;
            atomic_file::write(new_path, &serialize(&config))?;
            let profile_path = resolve_profile_path(new_path, &config.active_profile);
            let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
            atomic_file::write(&profile_path, &serialize_profile(&profile))?;
            let _ = fs::remove_file(&legacy);
            return Ok((config, profile));
        }
    }
    let config = config_defaults::fresh(detected_disks);
    let profile = config_defaults::fresh_profile(detected_disks);
    atomic_file::write(new_path, &serialize(&config))?;
    let profile_path = resolve_profile_path(new_path, &config.active_profile);
    let _ = std::fs::create_dir_all(profile_path.parent().unwrap());
    atomic_file::write(&profile_path, &serialize_profile(&profile))?;
    Ok((config, profile))
}

pub fn parse(source: &str) -> Result<Config, toml::de::Error> {
    let value: toml::Value = toml::from_str(source)?;
    let version = value.get("schema_version").and_then(|v| v.as_integer());
    if version.is_some_and(|v| v > config_validate::CURRENT_SCHEMA_VERSION as i64) {
        return Err(toml::de::Error::custom(format!("schema version {} is newer than supported {}", version.unwrap(), config_validate::CURRENT_SCHEMA_VERSION)));
    }
    if version == Some(0) { return Err(toml::de::Error::custom("schema version 0 is invalid")); }
    if value.get("monitor").is_some() || value.get("widgets").is_some() {
        return Err(toml::de::Error::custom("legacy format requires migration"));
    }
    let config: Config = toml::from_str(source)?;
    Ok(config)
}

pub fn serialize(config: &Config) -> String {
    toml::to_string_pretty(config).expect("config")
}

pub fn serialize_profile(profile: &Profile) -> String {
    toml::to_string_pretty(profile).expect("profile")
}

pub fn fresh_defaults(detected_disks: &[String]) -> Config {
    config_defaults::fresh(detected_disks)
}

pub fn fresh_profile(detected_disks: &[String]) -> Profile {
    config_defaults::fresh_profile(detected_disks)
}
