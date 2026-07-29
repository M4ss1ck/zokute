use crate::fullscreen::{effect, Effect, FullscreenConfig, Policy};
use std::time::{Duration, Instant};

fn config(behavior: &str) -> FullscreenConfig {
    FullscreenConfig { behavior: behavior.into(), ..FullscreenConfig::default() }
}

#[test]
fn the_default_policy_leaves_widgets_alone() {
    let config = FullscreenConfig::default();
    assert_eq!(config.behavior, "show");
    assert_eq!(config.dim_opacity, 0.25);
    assert_eq!(config.enter_delay_ms, 150);
    assert_eq!(config.exit_delay_ms, 250);
    assert!(matches!(effect(true, false, &config), Effect::None));
}

#[test]
fn hide_behavior_hides_only_while_fullscreen_is_active() {
    let config = config("hide");
    assert!(matches!(effect(true, false, &config), Effect::Hide));
    assert!(matches!(effect(false, false, &config), Effect::None));
}

#[test]
fn dim_behavior_reduces_to_the_configured_opacity() {
    let config = config("dim");
    match effect(true, false, &config) {
        Effect::Dim(opacity) => assert_eq!(opacity, 0.25),
        other => panic!("expected a dim, got {other:?}"),
    }
}

#[test]
fn edit_mode_overrides_fullscreen_hiding() {
    assert!(matches!(effect(true, true, &config("hide")), Effect::None));
    assert!(matches!(effect(true, true, &config("dim")), Effect::None));
}

#[test]
fn entering_fullscreen_waits_for_the_enter_delay() {
    let start = Instant::now();
    let mut policy = Policy::new();
    let pending = policy.observe(true, start, &config("hide")).expect("a decision should be scheduled");
    assert_eq!(pending.due, start + Duration::from_millis(150));
    assert!(!policy.is_active(), "the state must not flip before the delay elapses");
}

#[test]
fn leaving_fullscreen_waits_for_the_longer_exit_delay() {
    let start = Instant::now();
    let mut policy = Policy::new();
    let enter = policy.observe(true, start, &config("hide")).unwrap();
    policy.resolve(enter.generation);
    let exit = policy.observe(false, start + Duration::from_secs(1), &config("hide")).expect("an exit should be scheduled");
    assert_eq!(exit.due, start + Duration::from_secs(1) + Duration::from_millis(250));
}

#[test]
fn a_resolved_decision_becomes_the_active_state() {
    let start = Instant::now();
    let mut policy = Policy::new();
    let pending = policy.observe(true, start, &config("hide")).unwrap();
    assert_eq!(policy.resolve(pending.generation), Some(true));
    assert!(policy.is_active());
}

#[test]
fn a_reversal_before_the_delay_cancels_the_pending_decision() {
    let start = Instant::now();
    let mut policy = Policy::new();
    let pending = policy.observe(true, start, &config("hide")).unwrap();
    assert!(policy.observe(false, start + Duration::from_millis(50), &config("hide")).is_none());
    assert_eq!(policy.resolve(pending.generation), None, "the stale decision must not apply");
    assert!(!policy.is_active());
}

#[test]
fn a_stale_generation_never_applies() {
    let start = Instant::now();
    let mut policy = Policy::new();
    let first = policy.observe(true, start, &config("hide")).unwrap();
    policy.resolve(first.generation);
    let exit = policy.observe(false, start + Duration::from_secs(1), &config("hide")).unwrap();
    let re_enter = policy.observe(true, start + Duration::from_millis(1100), &config("hide"));
    assert!(re_enter.is_none(), "returning to the active state just cancels the exit");
    assert_eq!(policy.resolve(exit.generation), None);
    assert!(policy.is_active(), "the flicker must not have hidden anything");
}

#[test]
fn repeated_identical_observations_schedule_nothing_new() {
    let start = Instant::now();
    let mut policy = Policy::new();
    let pending = policy.observe(true, start, &config("hide")).unwrap();
    assert!(policy.observe(true, start + Duration::from_millis(10), &config("hide")).is_none());
    assert_eq!(policy.resolve(pending.generation), Some(true));
    assert!(policy.observe(true, start + Duration::from_millis(20), &config("hide")).is_none());
}
