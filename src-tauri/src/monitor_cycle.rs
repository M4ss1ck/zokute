use crate::config::Profile;
use crate::monitor::{resolve_monitor, MonitorCatalog, MonitorIdentity};
use crate::position_model::Position;

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

#[cfg(test)]
#[path = "monitor_cycle_tests.rs"]
mod tests;
