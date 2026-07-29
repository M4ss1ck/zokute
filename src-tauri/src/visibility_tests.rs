use crate::config::{Profile, SectionConfig};
use crate::visibility::{demand, VisibilityState};

fn profile(sections: Vec<(&str, bool)>) -> Profile {
    Profile {
        sections: sections.into_iter().map(|(id, enabled)| SectionConfig {
            id: id.into(), instance: id.into(), enabled, ..SectionConfig::default()
        }).collect(),
        ..Profile::default()
    }
}

// COLLECT-002: the default policy pauses work when nothing is on screen.
#[test]
fn a_fresh_state_reports_nothing_visible() {
    assert!(!VisibilityState::default().is_visible());
}

#[test]
fn showing_and_hiding_round_trips() {
    let state = VisibilityState::default();
    state.set_visible(true);
    assert!(state.is_visible());
    state.set_visible(false);
    assert!(!state.is_visible());
}

#[test]
fn clones_share_one_demand_signal() {
    let state = VisibilityState::default();
    let observer = state.clone();
    state.set_visible(true);
    assert!(observer.is_visible(), "a clone must not fork the demand state");
}

// Demand comes from what the profile says should be on screen. Polling a
// window's mapped state right after show() races GTK and reads false, which
// silently gated the collector off for the life of the process.
#[test]
fn enabled_widgets_create_demand() {
    assert!(demand(&profile(vec![("cpu", true)]), false));
}

#[test]
fn a_profile_with_no_enabled_widgets_creates_no_demand() {
    assert!(!demand(&profile(vec![("cpu", false), ("memory", false)]), false));
    assert!(!demand(&profile(vec![]), false));
}

#[test]
fn one_enabled_widget_among_many_is_enough() {
    assert!(demand(&profile(vec![("cpu", false), ("memory", true), ("disk", false)]), false));
}

#[test]
fn hiding_every_widget_removes_demand_even_with_enabled_sections() {
    assert!(!demand(&profile(vec![("cpu", true)]), true), "a tray hide must pause collection");
}

#[test]
fn showing_again_restores_demand() {
    let config = profile(vec![("cpu", true)]);
    assert!(!demand(&config, true));
    assert!(demand(&config, false));
}

#[test]
fn a_section_that_owns_no_window_creates_no_demand() {
    assert!(!demand(&profile(vec![("not-a-widget", true)]), false));
}
