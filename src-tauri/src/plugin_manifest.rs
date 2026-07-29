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
