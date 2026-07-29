use crate::onboarding::WorkArea;
use crate::onboarding_presets::snapshot_for;

fn area() -> WorkArea {
    WorkArea { identity: "monitor-0".into(), width: 1920, height: 1080 }
}

// M10 Task 2.3: Minimal creates clock/date; System Monitor adds the four
// metrics widgets; Blank creates none.
#[test]
fn the_minimal_preset_creates_only_a_clock_and_date() {
    let (_config, profile) = snapshot_for("minimal", "dark", &area(), &[]);
    let ids: Vec<_> = profile.sections.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(ids, vec!["clock", "date"]);
}

#[test]
fn the_system_monitor_preset_creates_the_specified_widgets() {
    let (_config, profile) = snapshot_for("system_monitor", "dark", &area(), &[]);
    let ids: Vec<_> = profile.sections.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(ids, vec!["clock", "date", "cpu", "memory", "disk", "network"]);
    assert!(profile.show_cpu_cores);
    assert!(!profile.system_fields.is_empty());
}

#[test]
fn the_blank_preset_creates_no_widgets() {
    let (_config, profile) = snapshot_for("blank", "light", &area(), &[]);
    assert!(profile.sections.is_empty());
}

#[test]
fn an_unrecognized_preset_falls_back_to_blank_rather_than_failing() {
    let (_config, profile) = snapshot_for("does-not-exist", "system", &area(), &[]);
    assert!(profile.sections.is_empty());
}

#[test]
fn the_chosen_theme_reaches_the_config() {
    for theme in ["light", "dark", "system"] {
        assert_eq!(snapshot_for("minimal", theme, &area(), &[]).0.theme, theme);
    }
}

// Roadmap rule 4: instance IDs must survive every composition operation.
#[test]
fn every_created_section_carries_a_unique_instance_id() {
    let (_config, profile) = snapshot_for("system_monitor", "dark", &area(), &[]);
    let mut instances: Vec<_> = profile.sections.iter().map(|s| s.instance.clone()).collect();
    instances.sort();
    let count = instances.len();
    instances.dedup();
    assert_eq!(instances.len(), count, "instance ids must be unique");
    assert!(profile.sections.iter().all(|s| !s.instance.is_empty()));
}

// M10 Task 2.4: anchors come from detected work areas, not hard-coded globals.
#[test]
fn widgets_are_anchored_and_laid_out_without_overlapping() {
    let (_config, profile) = snapshot_for("system_monitor", "dark", &area(), &[]);
    assert!(profile.sections.iter().all(|s| s.position.is_some()), "sections must be anchored");
    let mut ys: Vec<_> = profile.sections.iter().map(|s| s.y).collect();
    ys.sort();
    ys.dedup();
    assert_eq!(ys.len(), profile.sections.len(), "stacked widgets must not share a y");
}

#[test]
fn a_fresh_onboarding_profile_uses_the_default_cadence() {
    let (config, profile) = snapshot_for("minimal", "dark", &area(), &[]);
    assert_eq!(profile.collect_interval_ms, 1000);
    assert_eq!(config.schema_version, 3);
    assert_eq!(config.active_profile, "default");
}

#[test]
fn the_system_monitor_preset_enables_detected_disks() {
    let detected = vec!["uuid:root".to_string(), "mount:/data".to_string()];
    let (_, profile) = snapshot_for("system_monitor", "light", &area(), &detected);
    assert_eq!(
        profile.disks.iter().map(|disk| (&disk.id, disk.enabled)).collect::<Vec<_>>(),
        vec![(&detected[0], true), (&detected[1], true)],
    );
}
