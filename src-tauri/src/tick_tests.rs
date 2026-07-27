use crate::tick::{until_next_second, OFFSET_MS};

#[test]
fn waits_out_the_current_second_then_clears_the_boundary() {
    assert_eq!(until_next_second(1_700_000_000_400).as_millis() as u64, 600 + OFFSET_MS);
    assert_eq!(until_next_second(1_700_000_000_999).as_millis() as u64, 1 + OFFSET_MS);
}

#[test]
fn a_timestamp_already_on_a_boundary_waits_a_whole_second() {
    assert_eq!(until_next_second(1_700_000_000_000).as_millis() as u64, 1000 + OFFSET_MS);
}
