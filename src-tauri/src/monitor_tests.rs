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
fn resolves_numeric_monitor_indexes() {
    let current = [
        ("monitor-0".into(), mk_id("monitor-0", "a", true)),
        ("monitor-1".into(), mk_id("monitor-1", "b", false)),
    ];
    assert_eq!(resolve_monitor("1", &MonitorCatalog::new(), &current), Some(1));
}

#[test]
fn resolves_current_connector_without_a_catalog() {
    let current = [
        ("monitor-0".into(), mk_id("monitor-0", "a", true)),
        ("monitor-1".into(), mk_id("monitor-1", "b", false)),
    ];
    assert_eq!(resolve_monitor("monitor-1", &MonitorCatalog::new(), &current), Some(1));
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
    let current: Vec<(String, MonitorIdentity)> = vec![];
    assert_eq!(resolve_monitor("gone", &MonitorCatalog::new(), &current), None);
}
