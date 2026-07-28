use serde::Deserialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub protocol: u32,
    pub command: Vec<String>,
    pub cwd: Option<String>,
    #[serde(default = "default_interval")]
    pub interval: u64,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "default_max_output")]
    pub max_output_bytes: u64,
    #[serde(default)]
    pub config: BTreeMap<String, toml::Value>,
}

fn default_interval() -> u64 { 30 }
fn default_timeout() -> u64 { 10 }
fn default_max_output() -> u64 { 262144 }

#[derive(Debug)]
pub enum ManifestError {
    Io(std::io::Error),
    Parse(String),
    Invalid(String),
}

impl std::fmt::Display for ManifestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifestError::Io(e) => write!(f, "IO: {e}"),
            ManifestError::Parse(e) => write!(f, "parse: {e}"),
            ManifestError::Invalid(e) => write!(f, "invalid: {e}"),
        }
    }
}

impl PluginManifest {
    pub fn load(dir: &Path, manifest_path: &Path) -> Result<PluginManifest, ManifestError> {
        let source = std::fs::read_to_string(manifest_path).map_err(ManifestError::Io)?;
        let manifest: PluginManifest = toml::from_str(&source)
            .map_err(|e| ManifestError::Parse(e.to_string()))?;
        manifest.validate(dir)?;
        Ok(manifest)
    }

    fn validate(&self, dir: &Path) -> Result<(), ManifestError> {
        if self.id.is_empty() {
            return Err(ManifestError::Invalid("id must be nonempty".into()));
        }
        let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if self.id != dir_name {
            return Err(ManifestError::Invalid(format!(
                "plugin id '{}' does not match directory name '{dir_name}'", self.id
            )));
        }
        if self.name.is_empty() {
            return Err(ManifestError::Invalid("name must be nonempty".into()));
        }
        if self.protocol != 1 {
            return Err(ManifestError::Invalid(format!(
                "unsupported protocol {}", self.protocol
            )));
        }
        if self.command.is_empty() {
            return Err(ManifestError::Invalid("command array must be nonempty".into()));
        }
        if self.interval < 2 {
            return Err(ManifestError::Invalid("interval must be >= 2 seconds".into()));
        }
        if self.timeout_seconds == 0 {
            return Err(ManifestError::Invalid("timeout must be positive".into()));
        }
        if self.max_output_bytes == 0 || self.max_output_bytes > 1_048_576 {
            return Err(ManifestError::Invalid("max_output_bytes must be 1..1048576".into()));
        }
        Ok(())
    }
}

pub fn discover_manifests(plugins_dir: &Path) -> Vec<(PathBuf, Result<PluginManifest, ManifestError>)> {
    let mut results = Vec::new();
    let Ok(entries) = std::fs::read_dir(plugins_dir) else { return results };
    for entry in entries.flatten() {
        let dir_path = entry.path();
        if !dir_path.is_dir() { continue; }
        let manifest_path = dir_path.join("plugin.toml");
        if !manifest_path.exists() { continue; }
        let result = PluginManifest::load(&dir_path, &manifest_path);
        results.push((dir_path, result));
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn write_manifest(dir: &Path, content: &str) -> PathBuf {
        let path = dir.join("plugin.toml");
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn valid_manifest_parses() {
        let dir = TempDir::new().unwrap();
        let id = dir.path().file_name().unwrap().to_str().unwrap().to_string();
        let content = format!(r#"id = "{id}"
name = "Test Plugin"
version = "1.0.0"
protocol = 1
command = ["python3", "main.py"]
"#);
        let path = write_manifest(dir.path(), &content);
        let manifest = PluginManifest::load(dir.path(), &path).unwrap();
        assert_eq!(manifest.name, "Test Plugin");
        assert_eq!(manifest.interval, 30);
    }

    #[test]
    fn id_mismatch_is_rejected() {
        let dir = TempDir::new().unwrap();
        let plugin_dir = dir.path().join("right-id");
        std::fs::create_dir(&plugin_dir).unwrap();
        let content = r#"id = "wrong-id"
name = "Test"
version = "1.0.0"
protocol = 1
command = ["python3"]
"#;
        let path = write_manifest(&plugin_dir, content);
        assert!(PluginManifest::load(&plugin_dir, &path).is_err());
    }

    #[test]
    fn unsupported_protocol_is_rejected() {
        let dir = TempDir::new().unwrap();
        let id = dir.path().file_name().unwrap().to_str().unwrap().to_string();
        let content = format!(r#"id = "{id}"
name = "Test"
version = "1.0.0"
protocol = 99
command = ["python3"]
"#);
        let path = write_manifest(dir.path(), &content);
        assert!(PluginManifest::load(dir.path(), &path).is_err());
    }

    #[test]
    fn empty_command_is_rejected() {
        let dir = TempDir::new().unwrap();
        let id = dir.path().file_name().unwrap().to_str().unwrap().to_string();
        let content = format!(r#"id = "{id}"
name = "Test"
version = "1.0.0"
protocol = 1
command = []
"#);
        let path = write_manifest(dir.path(), &content);
        assert!(PluginManifest::load(dir.path(), &path).is_err());
    }

    #[test]
    fn interval_below_minimum_is_rejected() {
        let dir = TempDir::new().unwrap();
        let id = dir.path().file_name().unwrap().to_str().unwrap().to_string();
        let content = format!(r#"id = "{id}"
name = "Test"
version = "1.0.0"
protocol = 1
command = ["python3"]
interval = 1
"#);
        let path = write_manifest(dir.path(), &content);
        assert!(PluginManifest::load(dir.path(), &path).is_err());
    }

    #[test]
    fn discover_finds_valid_manifests() {
        let plugins_dir = TempDir::new().unwrap();
        let plugin_dir = plugins_dir.path().join("test-plugin");
        std::fs::create_dir(&plugin_dir).unwrap();
        let content = r#"id = "test-plugin"
name = "Test"
version = "1.0.0"
protocol = 1
command = ["python3"]
"#;
        write_manifest(&plugin_dir, content);
        let results = discover_manifests(plugins_dir.path());
        assert!(!results.is_empty());
        assert!(results[0].1.is_ok());
    }
}
