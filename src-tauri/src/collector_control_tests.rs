use crate::collector_control::normalize_interval;

// COLLECT-001: 250-60000 ms, default 1000.
#[test]
fn a_supported_interval_passes_through() {
    assert_eq!(normalize_interval(1000), 1000);
    assert_eq!(normalize_interval(250), 250);
    assert_eq!(normalize_interval(60000), 60000);
}

#[test]
fn an_interval_below_the_floor_is_raised() {
    assert_eq!(normalize_interval(0), 250);
    assert_eq!(normalize_interval(100), 250);
}

#[test]
fn an_interval_above_the_ceiling_is_capped() {
    assert_eq!(normalize_interval(120000), 60000);
}
