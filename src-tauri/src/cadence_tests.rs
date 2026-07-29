use crate::cadence::{Cadence, Tick};
use std::time::{Duration, Instant};

const INTERVAL: u64 = 1000;

#[test]
fn a_steady_visible_cadence_emits_every_tick() {
    let start = Instant::now();
    let mut cadence = Cadence::new(start);
    assert!(matches!(cadence.step(start + Duration::from_secs(1), true, INTERVAL), Tick::Resume));
    match cadence.step(start + Duration::from_secs(2), true, INTERVAL) {
        Tick::Emit { elapsed } => assert!((elapsed - 1.0).abs() < 0.01),
        other => panic!("expected an emit, got {other:?}"),
    }
}

#[test]
fn hidden_ticks_do_no_work() {
    let start = Instant::now();
    let mut cadence = Cadence::new(start);
    cadence.step(start + Duration::from_secs(1), true, INTERVAL);
    cadence.step(start + Duration::from_secs(2), true, INTERVAL);
    assert!(matches!(cadence.step(start + Duration::from_secs(3), false, INTERVAL), Tick::Idle));
}

#[test]
fn showing_again_after_a_long_hide_resumes_and_then_emits() {
    let start = Instant::now();
    let mut cadence = Cadence::new(start);
    cadence.step(start + Duration::from_secs(1), true, INTERVAL);
    cadence.step(start + Duration::from_secs(2), true, INTERVAL);
    for second in 3..60 {
        cadence.step(start + Duration::from_secs(second), false, INTERVAL);
    }
    assert!(matches!(cadence.step(start + Duration::from_secs(60), true, INTERVAL), Tick::Resume));
    match cadence.step(start + Duration::from_secs(61), true, INTERVAL) {
        Tick::Emit { elapsed } => assert!((elapsed - 1.0).abs() < 0.01),
        other => panic!("collector stalled after a hide instead of emitting, got {other:?}"),
    }
}

#[test]
fn a_suspend_gap_resumes_once_and_never_replays_the_missed_span() {
    let start = Instant::now();
    let mut cadence = Cadence::new(start);
    cadence.step(start + Duration::from_secs(1), true, INTERVAL);
    cadence.step(start + Duration::from_secs(2), true, INTERVAL);
    assert!(matches!(cadence.step(start + Duration::from_secs(600), true, INTERVAL), Tick::Resume));
    match cadence.step(start + Duration::from_secs(601), true, INTERVAL) {
        Tick::Emit { elapsed } => assert!(elapsed < 2.0, "resume replayed a {elapsed}s span"),
        other => panic!("expected an emit after resume, got {other:?}"),
    }
}

#[test]
fn a_gap_shorter_than_the_suspend_threshold_keeps_emitting() {
    let start = Instant::now();
    let mut cadence = Cadence::new(start);
    cadence.step(start + Duration::from_secs(1), true, INTERVAL);
    cadence.step(start + Duration::from_secs(2), true, INTERVAL);
    match cadence.step(start + Duration::from_millis(5500), true, INTERVAL) {
        Tick::Emit { elapsed } => assert!((elapsed - 3.5).abs() < 0.01),
        other => panic!("expected an emit inside the threshold, got {other:?}"),
    }
}

#[test]
fn a_slow_interval_widens_the_suspend_threshold() {
    let start = Instant::now();
    let mut cadence = Cadence::new(start);
    cadence.step(start + Duration::from_secs(10), true, 10000);
    cadence.step(start + Duration::from_secs(20), true, 10000);
    assert!(matches!(cadence.step(start + Duration::from_secs(35), true, 10000), Tick::Emit { .. }));
    assert!(matches!(cadence.step(start + Duration::from_secs(80), true, 10000), Tick::Resume));
}

#[test]
fn rapid_hide_and_show_resumes_exactly_once() {
    let start = Instant::now();
    let mut cadence = Cadence::new(start);
    cadence.step(start + Duration::from_secs(1), true, INTERVAL);
    cadence.step(start + Duration::from_secs(2), true, INTERVAL);
    assert!(matches!(cadence.step(start + Duration::from_millis(2100), false, INTERVAL), Tick::Idle));
    assert!(matches!(cadence.step(start + Duration::from_millis(2200), true, INTERVAL), Tick::Resume));
    assert!(matches!(cadence.step(start + Duration::from_millis(3200), true, INTERVAL), Tick::Emit { .. }));
    assert!(matches!(cadence.step(start + Duration::from_millis(4200), true, INTERVAL), Tick::Emit { .. }));
}
