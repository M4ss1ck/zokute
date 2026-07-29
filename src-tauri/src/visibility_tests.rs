use crate::visibility::VisibilityState;

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

#[test]
fn repeated_identical_updates_are_stable() {
    let state = VisibilityState::default();
    state.set_visible(true);
    state.set_visible(true);
    assert!(state.is_visible());
}
