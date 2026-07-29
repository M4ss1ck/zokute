use crate::background::StartLatch;

#[test]
fn the_first_claim_wins() {
    let latch = StartLatch::default();
    assert!(latch.claim(), "the first caller must start the background work");
}

#[test]
fn a_second_claim_does_not_start_a_duplicate_loop() {
    let latch = StartLatch::default();
    latch.claim();
    assert!(!latch.claim(), "a second start would run two collectors");
    assert!(!latch.claim());
}

// Startup and onboarding are two entry points into the same work: whichever
// arrives first starts it, and the other must not double it.
#[test]
fn clones_share_one_latch() {
    let latch = StartLatch::default();
    let onboarding = latch.clone();
    assert!(latch.claim());
    assert!(!onboarding.claim(), "a clone must not be able to start a second collector");
}

#[test]
fn a_latch_that_was_never_claimed_reports_not_started() {
    assert!(!StartLatch::default().started());
}

#[test]
fn claiming_marks_it_started() {
    let latch = StartLatch::default();
    latch.claim();
    assert!(latch.started());
}
