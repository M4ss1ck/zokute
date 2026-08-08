use super::*;
use crate::config::SectionConfig;
use crate::position_model::Anchor;

fn mk_id(c: &str, p: bool) -> MonitorIdentity {
    MonitorIdentity { connector: c.into(), edid_hash: String::new(), manufacturer: None,
        model: None, serial: None, name: c.into(), last_geometry: None, was_primary: p }
}

fn displays(count: usize) -> Vec<(String, MonitorIdentity)> {
    (0..count).map(|i| { let c = format!("monitor-{i}"); (c.clone(), mk_id(&c, i == 0)) }).collect()
}

fn anchored(instance: &str, identity: &str) -> SectionConfig {
    SectionConfig { id: "cpu".into(), instance: instance.into(), enabled: true,
        position: Some(Position::Anchored { monitor_identity: identity.into(), anchor: Anchor::TopLeft,
            offset_x: 24, offset_y: 48 }), ..Default::default() }
}

fn profile_with(sections: Vec<SectionConfig>) -> Profile { Profile { sections, ..Default::default() } }

fn identity_of(p: &Profile, i: &str) -> String {
    p.section(i).expect("section present").effective_position().monitor_identity().to_string()
}

#[test]
fn swaps_widgets_between_two_displays() {
    let profile = profile_with(vec![anchored("cpu", "0"), anchored("memory", "1")]);
    let cycled = cycle(profile, &MonitorCatalog::new(), &displays(2));
    assert_eq!(identity_of(&cycled, "cpu"), "1");
    assert_eq!(identity_of(&cycled, "memory"), "0");
}

#[test]
fn rotates_widgets_through_three_displays() {
    let profile = profile_with(vec![
        anchored("cpu", "0"),
        anchored("memory", "1"),
        anchored("disk", "2"),
    ]);
    let cycled = cycle(profile, &MonitorCatalog::new(), &displays(3));
    assert_eq!(identity_of(&cycled, "cpu"), "1");
    assert_eq!(identity_of(&cycled, "memory"), "2");
    assert_eq!(identity_of(&cycled, "disk"), "0", "the last display wraps to the first");
}

#[test]
fn leaves_a_single_display_profile_untouched() {
    let bare = SectionConfig {
        id: "cpu".into(),
        instance: "cpu".into(),
        enabled: true,
        ..Default::default()
    };
    let profile = profile_with(vec![anchored("memory", "0"), bare]);
    let cycled = cycle(profile, &MonitorCatalog::new(), &displays(1));
    assert_eq!(identity_of(&cycled, "memory"), "0");
    assert!(
        cycled.section("cpu").unwrap().position.is_none(),
        "a single-display cycle must not even materialize positions"
    );
}

#[test]
fn keeps_absolute_coordinates_when_moving_display() {
    let abs = SectionConfig { id: "cpu".into(), instance: "cpu".into(), enabled: true,
        position: Some(Position::Absolute { monitor_identity: "0".into(), x: 800, y: 450 }), ..Default::default() };
    let profile = profile_with(vec![abs]);
    let cycled = cycle(profile, &MonitorCatalog::new(), &displays(2));
    assert_eq!(cycled.section("cpu").unwrap().position, Some(Position::Absolute { monitor_identity: "1".into(), x: 800, y: 450 }));
}

#[test]
fn keeps_anchor_and_offsets_when_moving_display() {
    let profile = profile_with(vec![anchored("cpu", "0")]);
    let cycled = cycle(profile, &MonitorCatalog::new(), &displays(2));
    assert_eq!(
        cycled.section("cpu").unwrap().position,
        Some(Position::Anchored {
            monitor_identity: "1".into(),
            anchor: Anchor::TopLeft,
            offset_x: 24,
            offset_y: 48,
        })
    );
}

#[test]
fn materializes_a_position_for_sections_that_lack_one() {
    let legacy = SectionConfig {
        id: "cpu".into(),
        instance: "cpu".into(),
        enabled: true,
        monitor: 1,
        x: 5,
        y: 6,
        ..Default::default()
    };
    let cycled = cycle(profile_with(vec![legacy]), &MonitorCatalog::new(), &displays(2));
    assert_eq!(
        cycled.section("cpu").unwrap().position,
        Some(Position::Anchored {
            monitor_identity: "0".into(),
            anchor: Anchor::TopLeft,
            offset_x: 5,
            offset_y: 6,
        })
    );
}

#[test]
fn cycles_disabled_sections_too() {
    let mut off = anchored("memory", "0");
    off.enabled = false;
    let cycled = cycle(profile_with(vec![off]), &MonitorCatalog::new(), &displays(2));
    assert_eq!(
        identity_of(&cycled, "memory"),
        "1",
        "a disabled widget must land with its neighbours when re-enabled"
    );
}

#[test]
fn unresolvable_identity_advances_from_the_primary_display() {
    let profile = profile_with(vec![anchored("cpu", "vanished-display")]);
    let cycled = cycle(profile, &MonitorCatalog::new(), &displays(2));
    assert_eq!(identity_of(&cycled, "cpu"), "1");
}
