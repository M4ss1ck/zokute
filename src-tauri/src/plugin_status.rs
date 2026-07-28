use crate::plugin_discovery::PluginRegistry;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct PluginDiagnostic {
    pub id: String,
    pub name: String,
    pub version: String,
    pub protocol: u32,
    pub valid: bool,
    pub error: Option<String>,
}

pub fn collect_diagnostics(registry: &PluginRegistry) -> Vec<PluginDiagnostic> {
    registry.iter().map(|(id, entry)| PluginDiagnostic {
        id: id.clone(),
        name: entry.manifest.name.clone(),
        version: entry.manifest.version.clone(),
        protocol: entry.manifest.protocol,
        valid: entry.error.is_none(),
        error: entry.error.clone(),
    }).collect()
}
