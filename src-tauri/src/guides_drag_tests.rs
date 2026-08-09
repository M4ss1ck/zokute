use super::*;

const LEFT: Rect = Rect { x: 0, y: 0, width: 1920, height: 1080 };

fn named(label: &str, x: i32, y: i32) -> (String, Rect) {
    (label.to_string(), Rect { x, y, width: 100, height: 80 })
}

#[test]
fn excludes_the_dragged_widget_from_its_own_targets() {
    let all = vec![named("clock", 100, 100), named("cpu", 400, 400)];
    let rects = other_rects(&all, "clock", LEFT);
    assert_eq!(rects, vec![Rect { x: 400, y: 400, width: 100, height: 80 }]);
}

#[test]
fn excludes_widgets_on_another_display() {
    let all = vec![named("cpu", 400, 400), named("disk", 2400, 400)];
    let rects = other_rects(&all, "clock", LEFT);
    assert_eq!(rects, vec![Rect { x: 400, y: 400, width: 100, height: 80 }]);
}

#[test]
fn keeps_widgets_on_a_secondary_display_when_that_is_the_one_being_used() {
    let right = Rect { x: 1920, y: 0, width: 2560, height: 1440 };
    let all = vec![named("cpu", 400, 400), named("disk", 2400, 400)];
    let rects = other_rects(&all, "clock", right);
    assert_eq!(rects, vec![Rect { x: 2400, y: 400, width: 100, height: 80 }]);
}

#[test]
fn returns_nothing_when_the_dragged_widget_is_alone_on_its_display() {
    let all = vec![named("clock", 100, 100)];
    assert!(other_rects(&all, "clock", LEFT).is_empty());
}
