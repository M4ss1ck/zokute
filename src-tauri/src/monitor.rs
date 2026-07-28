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
mod tests {
    use super::*;

    fn mk_id(connector: &str, hash: &str, primary: bool) -> MonitorIdentity {
        MonitorIdentity {
            connector: connector.into(),
            edid_hash: hash.into(),
            manufacturer: None,
            model: None,
            serial: None,
            name: connector.into(),
            last_geometry: None,
            was_primary: primary,
        }
    }

    fn with_geom(mut id: MonitorIdentity, x: i32, y: i32, w: u32, h: u32) -> MonitorIdentity {
        id.last_geometry = Some(MonitorGeometry { x, y, width: w, height: h, scale_factor: 1.0 });
        id
    }

    #[test]
    fn resolves_by_exact_match() {
        let catalog = [("eDP-1".into(), mk_id("eDP-1", "abc", false))].into();
        let current = [("eDP-1".into(), mk_id("eDP-1", "abc", false))];
        assert_eq!(resolve_monitor("eDP-1", &catalog, &current), Some(0));
    }

    #[test]
    fn falls_back_to_connector_when_hash_changes() {
        let catalog = [("eDP-1".into(), mk_id("eDP-1", "abc", false))].into();
        let current = [("eDP-1".into(), mk_id("eDP-1", "xyz", false))];
        assert_eq!(resolve_monitor("eDP-1", &catalog, &current), Some(0));
    }

    #[test]
    fn falls_back_to_edid_hash_when_connector_renames() {
        let catalog = [("LVDS-1".into(), mk_id("LVDS-1", "abc", false))].into();
        let current = [("eDP-1".into(), mk_id("eDP-1", "abc", false))];
        assert_eq!(resolve_monitor("LVDS-1", &catalog, &current), Some(0));
    }

    #[test]
    fn falls_back_to_last_geometry() {
        let id = with_geom(mk_id("gone", "abc", false), 0, 0, 1920, 1080);
        let catalog = [("gone".into(), id)].into();
        let current = [("eDP-1".into(), with_geom(mk_id("eDP-1", "xyz", false), 0, 0, 1920, 1080))];
        assert_eq!(resolve_monitor("gone", &catalog, &current), Some(0));
    }

    #[test]
    fn falls_back_to_primary_monitor() {
        let catalog = [("gone".into(), mk_id("gone", "abc", false))].into();
        let current = [
            ("HDMI-1".into(), mk_id("HDMI-1", "x", false)),
            ("eDP-1".into(), mk_id("eDP-1", "y", true)),
        ];
        assert_eq!(resolve_monitor("gone", &catalog, &current), Some(1));
    }

    #[test]
    fn falls_back_to_first_monitor() {
        let catalog = [("gone".into(), mk_id("gone", "abc", false))].into();
        let current = [("eDP-1".into(), mk_id("eDP-1", "x", false))];
        assert_eq!(resolve_monitor("gone", &catalog, &current), Some(0));
    }

    #[test]
    fn returns_none_when_no_monitors() {
        let catalog = MonitorCatalog::new();
        let current: Vec<(String, MonitorIdentity)> = vec![];
        assert_eq!(resolve_monitor("gone", &catalog, &current), None);
    }
}
