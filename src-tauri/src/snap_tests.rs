use super::*;

const DISPLAY: Rect = Rect { x: 0, y: 0, width: 1920, height: 1080 };

fn snap(widget: Rect, others: &[Rect]) -> Snap {
    snap_position(widget, DISPLAY, others, TOLERANCE)
}

#[test]
fn centres_a_widget_on_the_display() {
    let result = snap(Rect { x: 765, y: 300, width: 400, height: 300 }, &[]);
    assert_eq!(result.x, 760);
    assert_eq!(result.x + 400 / 2, 960);
    assert_eq!(result.y, 300);
    assert_eq!(result.guides, vec![Guide { orientation: GuideOrientation::Vertical, position: 960 }]);
}

#[test]
fn a_left_edge_near_the_centre_meets_the_centre_line() {
    let result = snap(Rect { x: 955, y: 300, width: 400, height: 300 }, &[]);
    assert_eq!(result.x, 960);
    assert_eq!(result.guides, vec![Guide { orientation: GuideOrientation::Vertical, position: 960 }]);
}

#[test]
fn already_centred_reports_the_guide_without_moving() {
    let result = snap(Rect { x: 760, y: 300, width: 400, height: 300 }, &[]);
    assert_eq!(result.x, 760);
    assert_eq!(result.guides, vec![Guide { orientation: GuideOrientation::Vertical, position: 960 }]);
}

#[test]
fn aligns_left_edges_with_another_widget() {
    let other = Rect { x: 200, y: 100, width: 300, height: 60 };
    let result = snap(Rect { x: 203, y: 300, width: 100, height: 80 }, &[other]);
    assert_eq!(result.x, 200);
    assert_eq!(result.y, 300);
    assert_eq!(result.guides, vec![Guide { orientation: GuideOrientation::Vertical, position: 200 }]);
}

#[test]
fn aligns_right_edges_with_another_widget() {
    let other = Rect { x: 100, y: 100, width: 305, height: 60 };
    let result = snap(Rect { x: 300, y: 300, width: 100, height: 80 }, &[other]);
    assert_eq!(result.x, 305);
    assert_eq!(result.guides, vec![Guide { orientation: GuideOrientation::Vertical, position: 405 }]);
}

#[test]
fn another_widgets_centre_is_not_a_target() {
    let other = Rect { x: 200, y: 100, width: 300, height: 60 };
    let result = snap(Rect { x: 352, y: 300, width: 100, height: 80 }, &[other]);
    assert_eq!(result.x, 352);
    assert_eq!(result.y, 300);
    assert!(result.guides.is_empty());
}

#[test]
fn beyond_tolerance_nothing_snaps_and_no_guides_appear() {
    let result = snap(Rect { x: 50, y: 50, width: 100, height: 80 }, &[]);
    assert_eq!(result.x, 50);
    assert_eq!(result.y, 50);
    assert!(result.guides.is_empty());
}

#[test]
fn a_display_target_beats_a_widget_target_at_equal_distance() {
    let other = Rect { x: 750, y: 100, width: 100, height: 60 };
    let result = snap(Rect { x: 755, y: 300, width: 400, height: 80 }, &[other]);
    assert_eq!(result.x, 760);
    assert_eq!(result.guides, vec![Guide { orientation: GuideOrientation::Vertical, position: 960 }]);
}

#[test]
fn the_axes_snap_independently() {
    let other = Rect { x: 1400, y: 300, width: 100, height: 60 };
    let result = snap(Rect { x: 765, y: 302, width: 400, height: 300 }, &[other]);
    assert_eq!(result.x, 760);
    assert_eq!(result.y, 300);
    assert_eq!(
        result.guides,
        vec![
            Guide { orientation: GuideOrientation::Vertical, position: 960 },
            Guide { orientation: GuideOrientation::Horizontal, position: 300 },
        ]
    );
}

#[test]
fn a_monitor_origin_offset_is_respected() {
    let display = Rect { x: 1920, y: 0, width: 2560, height: 1440 };
    let result = snap_position(Rect { x: 3005, y: 300, width: 400, height: 300 }, display, &[], TOLERANCE);
    assert_eq!(result.x, 3000);
    assert_eq!(result.guides, vec![Guide { orientation: GuideOrientation::Vertical, position: 3200 }]);
}
