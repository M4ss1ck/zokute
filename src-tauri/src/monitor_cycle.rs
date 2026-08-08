use crate::config::{Config, Profile};
use crate::monitor::{resolve_monitor, MonitorCatalog, MonitorGeometry, MonitorIdentity};
use crate::position_model::Position;
use crate::{config_write, edit_geometry, edit_mode, edit_touched};
use std::sync::{Arc, RwLock};
use tauri::{AppHandle, Manager};

fn with_monitor(position: Position, identity: String) -> Position {
    match position {
        Position::Absolute { x, y, .. } => Position::Absolute { monitor_identity: identity, x, y },
        Position::Anchored { anchor, offset_x, offset_y, .. } => Position::Anchored {
            monitor_identity: identity,
            anchor,
            offset_x,
            offset_y,
        },
    }
}

/// Advance every widget one display along the compositor's enumeration order,
/// wrapping past the last back to the first. Coordinates are deliberately left
/// alone: anchored widgets re-derive their anchor from the target's size, and
/// absolute ones keep the offsets the user chose, even if that puts them off a
/// smaller display.
pub fn cycle(
    mut profile: Profile,
    catalog: &MonitorCatalog,
    monitors: &[(String, MonitorIdentity)],
) -> Profile {
    if monitors.len() < 2 {
        return profile;
    }
    for section in &mut profile.sections {
        let position = section.effective_position();
        let index = resolve_monitor(position.monitor_identity(), catalog, monitors).unwrap_or(0);
        let next = (index + 1) % monitors.len();
        section.set_position(with_monitor(position, next.to_string()));
    }
    profile
}

pub fn run(app: &AppHandle) {
    // The settings dialog keeps its unsaved draft in this same shared state;
    // cycling mid-session would persist that draft to disk and let the
    // dialog's stale copy overwrite the cycled monitor identities on save.
    if edit_mode::session_active(app) {
        return;
    }
    let Ok(monitors) = app.available_monitors() else { return };
    // The tray item's enabled state is fixed when the menu is built, so a
    // display unplugged since startup leaves an enabled item that must no-op.
    if monitors.len() < 2 {
        return;
    }
    let current: Vec<(String, MonitorIdentity)> = monitors
        .iter()
        .enumerate()
        .map(|(i, monitor)| {
            let position = monitor.position();
            let size = monitor.size();
            let identity = MonitorIdentity {
                connector: format!("monitor-{i}"),
                edid_hash: String::new(),
                manufacturer: None,
                model: None,
                serial: None,
                name: format!("Monitor {i}"),
                last_geometry: Some(MonitorGeometry {
                    x: position.x,
                    y: position.y,
                    width: size.width,
                    height: size.height,
                    scale_factor: monitor.scale_factor(),
                }),
                was_primary: i == 0,
            };
            (format!("monitor-{i}"), identity)
        })
        .collect();

    let Some(config) = app
        .try_state::<Arc<RwLock<Config>>>()
        .and_then(|state| state.read().ok().map(|guard| guard.clone()))
    else {
        return;
    };
    let Some(profile) = app
        .try_state::<Arc<RwLock<Profile>>>()
        .and_then(|state| state.read().ok().map(|guard| guard.clone()))
    else {
        return;
    };

    let profile = edit_geometry::merge_live_geometry(app, profile);
    // Disarms the live-geometry merge that `config_write::apply` performs
    // internally; without this, that merge re-captures each dragged widget's
    // current on-screen monitor and silently undoes the cycle for it. Noted
    // here because this reasoning previously lived only in a gitignored doc.
    edit_touched::clear(app);
    let catalog = profile.monitor_catalog.clone();
    if let Err(error) = config_write::apply(app, config, cycle(profile, &catalog, &current)) {
        eprintln!("cycle_displays: {error}");
    }
}

#[cfg(test)]
#[path = "monitor_cycle_tests.rs"]
mod tests;
