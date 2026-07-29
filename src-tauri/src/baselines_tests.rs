use crate::baselines::delta_per_second;

// COLLECT-003: counter regression and source reappearance reset rather than
// producing a spike.
#[test]
fn a_rising_counter_becomes_a_per_second_rate() {
    assert_eq!(delta_per_second(1_000, 3_000, 2.0), 2_000 / 2);
}

#[test]
fn a_counter_that_went_backwards_reports_nothing() {
    assert_eq!(delta_per_second(9_000, 100, 1.0), 0, "a reset counter must not read as a huge burst");
}

#[test]
fn an_unchanged_counter_reports_zero() {
    assert_eq!(delta_per_second(500, 500, 1.0), 0);
}

#[test]
fn a_zero_elapsed_span_cannot_divide_by_zero() {
    assert!(delta_per_second(0, 1_000, 0.0) < u64::MAX);
}
