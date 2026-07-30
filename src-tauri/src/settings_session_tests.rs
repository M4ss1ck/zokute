use crate::settings_session::arrange_transition;

#[test]
fn turning_arrange_off_merges_geometry_before_the_flag_drops() {
    // merge_live_geometry early-returns unless EditMode is still set, so the
    // merge must be ordered before the flag clears or a drag is lost.
    assert_eq!(arrange_transition(false), (true, false));
}

#[test]
fn turning_arrange_on_has_nothing_to_merge() {
    assert_eq!(arrange_transition(true), (false, true));
}
