use crate::plugin_manifest::{discover_manifests, PluginManifest};
use std::collections::BTreeMap;
use std::path::PathBuf;

pub type PluginRegistry = BTreeMap<String, PluginEntry>;

#[derive(Debug, Clone)]
pub struct PluginEntry {
    pub manifest: PluginManifest,
    pub dir: PathBuf,
    pub error: Option<String>,
}

pub fn scan_plugins(plugins_dir: &PathBuf) -> PluginRegistry {
    let mut registry = PluginRegistry::new();
    for (dir, result) in discover_manifests(plugins_dir) {
        let dir_name = dir.file_name().and_then(|n| n.to_str()).unwrap_or("unknown").to_string();
        match result {
            Ok(manifest) => {
                registry.insert(manifest.id.clone(), PluginEntry { manifest, dir, error: None });
            }
            Err(err) => {
                registry.insert(dir_name.clone(), PluginEntry {
                    manifest: create_fallback_manifest(&dir_name),
                    dir,
                    error: Some(format!("{err}")),
                });
            }
        }
    }
    registry
}

fn create_fallback_manifest(id: &str) -> PluginManifest {
    PluginManifest {
        id: id.to_string(), name: id.to_string(), version: "0.0.0".into(),
        protocol: 0, command: vec![], cwd: None,
        interval: 30, timeout_seconds: 10, max_output_bytes: 262144,
        config: Default::default(),
    }
}

pub fn is_plugin_enabled(section_plugin_id: &str, registry: &PluginRegistry) -> bool {
    registry.get(section_plugin_id).map_or(false, |e| e.error.is_none())
}
