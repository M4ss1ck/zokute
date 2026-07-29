use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

pub type MonitorCatalog = BTreeMap<String, MonitorIdentity>;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MonitorIdentity {
    pub connector: String,
    pub edid_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manufacturer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial: Option<String>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_geometry: Option<MonitorGeometry>,
    #[serde(default)]
    pub was_primary: bool,
}

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
pub struct MonitorGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub scale_factor: f64,
}

pub fn resolve_monitor(
    identity: &str,
    catalog: &MonitorCatalog,
    current_monitors: &[(String, MonitorIdentity)],
) -> Option<usize> {
    if let Ok(index) = identity.parse::<usize>() {
        if index < current_monitors.len() { return Some(index); }
    }
    if let Some(index) = current_monitors.iter().position(|(connector, _)| connector == identity) {
        return Some(index);
    }
    if let Some(known) = catalog.get(identity) {
        if let Some(idx) = current_monitors
            .iter()
            .position(|(connector, id)| connector == &known.connector && id.edid_hash == known.edid_hash)
        {
            return Some(idx);
        }
        if let Some(idx) = current_monitors
            .iter()
            .position(|(connector, _)| connector == &known.connector)
        {
            return Some(idx);
        }
        if let Some(idx) = current_monitors
            .iter()
            .position(|(_, id)| id.edid_hash == known.edid_hash)
        {
            return Some(idx);
        }
        if let Some(geom) = &known.last_geometry {
            if let Some(idx) = current_monitors.iter().position(|(_, id)| {
                id.last_geometry.map_or(false, |g| {
                    g.x == geom.x && g.y == geom.y && g.width == geom.width
                })
            }) {
                return Some(idx);
            }
        }
    }
    if let Some(idx) = current_monitors.iter().position(|(_, id)| id.was_primary) {
        return Some(idx);
    }
    if let Some(idx) = current_monitors.iter().position(|(_, _)| true) {
        return Some(idx);
    }
    None
}

#[cfg(test)]
#[path = "monitor_tests.rs"]
mod tests;
