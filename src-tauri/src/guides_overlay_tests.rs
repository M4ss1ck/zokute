use super::*;
use crate::snap::{Guide, GuideOrientation};

#[test]
fn recognises_overlay_labels() {
    assert!(is_overlay("guides-0"));
    assert!(is_overlay("guides-12"));
    assert!(!is_overlay("clock"));
    assert!(!is_overlay("settings"));
}

#[test]
fn converts_a_vertical_guide_into_overlay_local_css_pixels() {
    let guide = Guide { orientation: GuideOrientation::Vertical, position: 3200 };
    let line = to_local(guide, (1920, 0), 1.0);
    assert_eq!(line.orientation, "Vertical");
    assert_eq!(line.position, 1280.0);
}

#[test]
fn converts_a_horizontal_guide_into_overlay_local_css_pixels() {
    let guide = Guide { orientation: GuideOrientation::Horizontal, position: 900 };
    let line = to_local(guide, (1920, 100), 1.0);
    assert_eq!(line.orientation, "Horizontal");
    assert_eq!(line.position, 800.0);
}

#[test]
fn divides_by_the_scale_factor() {
    let guide = Guide { orientation: GuideOrientation::Vertical, position: 960 };
    let line = to_local(guide, (0, 0), 2.0);
    assert_eq!(line.position, 480.0);
}
