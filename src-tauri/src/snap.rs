pub const TOLERANCE: i32 = 8;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GuideOrientation {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Guide {
    pub orientation: GuideOrientation,
    pub position: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Snap {
    pub x: i32,
    pub y: i32,
    pub guides: Vec<Guide>,
}

/// Resolves one axis. `others` holds the (start, length) of every other widget
/// on the same axis. Display targets are tried first and a strictly smaller
/// distance is needed to displace the incumbent, so the display wins a tie and
/// the screen centre cannot be stolen by a coincidental widget edge.
fn solve_axis(
    start: i32,
    length: i32,
    display_start: i32,
    display_length: i32,
    others: &[(i32, i32)],
    tolerance: i32,
) -> Option<(i32, i32)> {
    let sources = [start, start + length / 2, start + length];
    let mut best = tolerance + 1;
    let mut winner = None;
    let display_targets = [
        display_start,
        display_start + display_length / 2,
        display_start + display_length,
    ];
    for target in display_targets {
        for source in sources {
            let distance = (target - source).abs();
            if distance <= tolerance && distance < best {
                best = distance;
                winner = Some((target - source, target));
            }
        }
    }
    for (other_start, other_length) in others {
        for target in [*other_start, other_start + other_length] {
            for source in [sources[0], sources[2]] {
                let distance = (target - source).abs();
                if distance <= tolerance && distance < best {
                    best = distance;
                    winner = Some((target - source, target));
                }
            }
        }
    }
    winner
}

/// `others` must already exclude the dragged widget and anything on another
/// display. All coordinates are physical pixels in global desktop space.
pub fn snap_position(widget: Rect, display: Rect, others: &[Rect], tolerance: i32) -> Snap {
    let mut guides = Vec::new();
    let mut x = widget.x;
    let mut y = widget.y;

    let horizontal: Vec<(i32, i32)> = others.iter().map(|rect| (rect.x, rect.width)).collect();
    if let Some((offset, position)) =
        solve_axis(widget.x, widget.width, display.x, display.width, &horizontal, tolerance)
    {
        x += offset;
        guides.push(Guide { orientation: GuideOrientation::Vertical, position });
    }

    let vertical: Vec<(i32, i32)> = others.iter().map(|rect| (rect.y, rect.height)).collect();
    if let Some((offset, position)) =
        solve_axis(widget.y, widget.height, display.y, display.height, &vertical, tolerance)
    {
        y += offset;
        guides.push(Guide { orientation: GuideOrientation::Horizontal, position });
    }

    Snap { x, y, guides }
}

#[cfg(test)]
#[path = "snap_tests.rs"]
mod tests;
