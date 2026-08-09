use crate::config::Profile;
use crate::config_error::ConfigError;
use crate::paths;

/// Load a profile by name
pub fn load(name: &str) -> Result<Profile, ConfigError> {
    let path = paths::profile_path(name);
    let source = std::fs::read_to_string(&path)?;
    let profile: Profile = toml::from_str(&source)
        .map_err(|e| ConfigError::Parse(e.to_string()))?;
    Ok(profile)
}

/// Duplicate a profile
#[tauri::command]
pub fn duplicate_profile(name: String, new_name: String) -> Result<(), String> {
    let profile = load(&name).map_err(|e| e.to_string())?;
    let dest = paths::profile_path(&new_name);
    if dest.exists() {
        return Err(format!("profile '{new_name}' already exists"));
    }
    let contents = crate::config::serialize_profile(&profile);
    crate::atomic_file::write(&dest, &contents).map_err(|e| e.to_string())?;
    Ok(())
}

/// Export a profile to a user-chosen path
#[tauri::command]
pub fn export_profile(name: String, dest: String) -> Result<(), String> {
    let profile = load(&name).map_err(|e| e.to_string())?;
    let plugin_ids: Vec<String> = profile.sections.iter()
        .filter(|s| s.is_plugin())
        .filter_map(|s| s.plugin_id.clone())
        .collect();
    let export = serde_json::json!({
        "profile": profile,
        "plugin_manifests": plugin_ids,
        "exported_at": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0),
    });
    let json = serde_json::to_string_pretty(&export)
        .map_err(|e| format!("export serialize: {e}"))?;
    std::fs::write(&dest, &json).map_err(|e| format!("export write: {e}"))?;
    Ok(())
}

/// Import a profile from a file
#[tauri::command]
pub fn import_profile(source: String, new_name: String) -> Result<String, String> {
    let source_path = std::path::PathBuf::from(&source);
    let raw = std::fs::read_to_string(&source_path).map_err(|e| format!("read: {e}"))?;

    let profile = if let Ok(export) = serde_json::from_str::<serde_json::Value>(&raw) {
        if let Some(p) = export.get("profile") {
            serde_json::from_value(p.clone()).map_err(|e| format!("import profile from JSON: {e}"))?
        } else {
            return Err("export JSON missing 'profile' key".into());
        }
    } else {
        toml::from_str::<Profile>(&raw).map_err(|e| format!("import parse: {e}"))?
    };

    let dest = paths::profile_path(&new_name);
    if dest.exists() {
        return Err(format!("profile '{new_name}' already exists"));
    }

    let plugins_dir = paths::plugins_dir();
    let missing_plugins: Vec<String> = profile.sections.iter()
        .filter(|s| s.is_plugin())
        .filter_map(|s| s.plugin_id.as_ref())
        .filter(|id| !plugins_dir.join(id).is_dir())
        .cloned()
        .collect();

    let contents = crate::config::serialize_profile(&profile);
    crate::atomic_file::write(&dest, &contents).map_err(|e| e.to_string())?;

    let msg = if missing_plugins.is_empty() {
        format!("imported '{}'", new_name)
    } else {
        format!("imported '{}' (missing plugins: {})", new_name, missing_plugins.join(", "))
    };
    Ok(msg)
}

/// Delete a profile
#[tauri::command]
pub fn delete_profile(name: String) -> Result<(), String> {
    if name == "default" {
        return Err("cannot delete default profile".into());
    }
    let path = paths::profile_path(&name);
    if !path.exists() {
        return Err(format!("profile '{name}' not found"));
    }
    std::fs::remove_file(&path).map_err(|e| format!("delete: {e}"))?;
    Ok(())
}
