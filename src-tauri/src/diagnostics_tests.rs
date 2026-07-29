use crate::config::Profile;
use crate::diagnostics::{collect, DetectionMode};
use crate::fullscreen::FullscreenConfig;

fn profile(behavior: &str) -> Profile {
    Profile {
        fullscreen: FullscreenConfig { behavior: behavior.into(), ..FullscreenConfig::default() },
        ..Profile::default()
    }
}

#[test]
fn diagnostics_report_the_active_fullscreen_behavior() {
    let report = collect(None, Some(&profile("dim")), DetectionMode::Events);
    assert_eq!(report.fullscreen.behavior, "dim");
    assert_eq!(report.fullscreen.dim_opacity, 0.25);
}

#[test]
fn event_driven_detection_is_reported_as_such() {
    let report = collect(None, Some(&profile("hide")), DetectionMode::Events);
    assert_eq!(report.fullscreen.detection, "events");
    assert!(!report.fullscreen.degraded);
}

#[test]
fn the_capped_poll_fallback_is_reported_as_degraded() {
    let report = collect(None, Some(&profile("hide")), DetectionMode::DegradedPoll);
    assert_eq!(report.fullscreen.detection, "poll");
    assert!(report.fullscreen.degraded, "a degraded mode must be visible in diagnostics");
}

#[test]
fn diagnostics_carry_no_note_text_or_plugin_output() {
    let report = collect(None, Some(&profile("show")), DetectionMode::Events);
    let json = serde_json::to_string(&report).expect("serialize");
    assert!(!json.contains("plugin_config"));
    assert!(!json.contains("content"));
}
