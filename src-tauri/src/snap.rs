pub const DEFAULT_TOLERANCE: i32 = 8;
pub const DEFAULT_GRID: i32 = 6;

#[derive(Debug, Clone, Copy)]
pub struct SnapPoint {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy)]
pub struct Guide {
    pub orientation: GuideOrientation,
    pub position: i32,
}

#[derive(Debug, Clone, Copy)]
pub enum GuideOrientation {
    Vertical,
    Horizontal,
}

fn work_area_edges(wa_x: i32, wa_y: i32, wa_w: u32, wa_h: u32) -> Vec<SnapPoint> {
    let w = wa_w as i32;
    let h = wa_h as i32;
    vec![
        SnapPoint { x: wa_x, y: wa_y },
        SnapPoint { x: wa_x + w / 2, y: wa_y + h / 2 },
        SnapPoint { x: wa_x + w, y: wa_y + h },
        SnapPoint { x: wa_x + w / 2, y: wa_y },
        SnapPoint { x: wa_x, y: wa_y + h / 2 },
        SnapPoint { x: wa_x + w, y: wa_y + h / 2 },
        SnapPoint { x: wa_x, y: wa_y + h },
        SnapPoint { x: wa_x + w / 2, y: wa_y + h },
    ]
}

fn widget_edges(x: i32, y: i32, w: u32, h: u32) -> Vec<SnapPoint> {
    let w = w as i32;
    let h = h as i32;
    vec![
        SnapPoint { x, y },
        SnapPoint { x: x + w / 2, y: y + h / 2 },
        SnapPoint { x: x + w, y: y + h },
        SnapPoint { x: x + w / 2, y },
        SnapPoint { x, y: y + h / 2 },
        SnapPoint { x: x + w, y: y + h / 2 },
        SnapPoint { x, y: y + h },
        SnapPoint { x: x + w / 2, y: y + h },
    ]
}

pub fn snap_position(
    x: i32, y: i32, _w: u32, _h: u32,
    wa_x: i32, wa_y: i32, wa_w: u32, wa_h: u32,
    other_widgets: &[(i32, i32, u32, u32)],
    tolerance: i32,
    bypass: bool,
) -> (i32, i32, Vec<Guide>) {
    if bypass {
        return (x, y, vec![]);
    }

    let mut guides = Vec::new();
    let mut snapped_x = x;
    let mut snapped_y = y;
    let mut best_dx = tolerance;
    let mut best_dy = tolerance;
    let mut hit_x = false;
    let mut hit_y = false;

    let targets = work_area_edges(wa_x, wa_y, wa_w, wa_h);
    for target in &targets {
        let dx = (x - target.x).abs();
        let dy = (y - target.y).abs();
        if dx <= best_dx && (dx > 0 || dy > 0) {
            best_dx = dx;
            snapped_x = target.x;
            hit_x = true;
        }
        if dy <= best_dy && (dy > 0 || dx > 0) {
            best_dy = dy;
            snapped_y = target.y;
            hit_y = true;
        }
    }

    for other in other_widgets {
        for target in widget_edges(other.0, other.1, other.2, other.3) {
            let dx = (x - target.x).abs();
            let dy = (y - target.y).abs();
            if dx <= best_dx && dx > 0 {
                best_dx = dx;
                snapped_x = target.x;
                hit_x = true;
            }
            if dy <= best_dy && dy > 0 {
                best_dy = dy;
                snapped_y = target.y;
                hit_y = true;
            }
        }
    }

    if hit_x {
        guides.push(Guide { orientation: GuideOrientation::Vertical, position: snapped_x });
    }
    if hit_y {
        guides.push(Guide { orientation: GuideOrientation::Horizontal, position: snapped_y });
    }

    (snapped_x, snapped_y, guides)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_snap_within_tolerance() {
        let (sx, sy, guides) = snap_position(50, 50, 100, 80, 0, 0, 1920, 1080, &[], 8, false);
        assert_eq!(sx, 50);
        assert_eq!(sy, 50);
        assert!(guides.is_empty());
    }

    #[test]
    fn snaps_to_work_area_top_left() {
        let (sx, sy, _) = snap_position(3, 4, 100, 80, 0, 0, 1920, 1080, &[], 8, false);
        assert_eq!(sx, 0);
        assert_eq!(sy, 0);
    }

    #[test]
    fn bypass_returns_original() {
        let (sx, sy, _) = snap_position(5, 5, 100, 80, 0, 0, 1920, 1080, &[], 8, true);
        assert_eq!(sx, 5);
        assert_eq!(sy, 5);
    }

    #[test]
    fn snaps_to_other_widget_edge() {
        let (sx, sy, _) = snap_position(195, 50, 100, 80, 0, 0, 1920, 1080, &[(200, 50, 100, 80)], 8, false);
        assert_eq!(sx, 200);
        assert_eq!(sy, 50);
    }
}
