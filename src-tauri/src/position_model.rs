use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum Anchor {
    #[serde(rename = "top_left")]
    TopLeft,
    #[serde(rename = "top_center")]
    TopCenter,
    #[serde(rename = "top_right")]
    TopRight,
    #[serde(rename = "center_left")]
    CenterLeft,
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "center_right")]
    CenterRight,
    #[serde(rename = "bottom_left")]
    BottomLeft,
    #[serde(rename = "bottom_center")]
    BottomCenter,
    #[serde(rename = "bottom_right")]
    BottomRight,
}

impl Anchor {
    pub fn anchor_point(&self, width: u32, height: u32) -> (i32, i32) {
        let w = width as i32;
        let h = height as i32;
        let half_w = w / 2;
        let half_h = h / 2;
        match self {
            Anchor::TopLeft => (0, 0),
            Anchor::TopCenter => (half_w, 0),
            Anchor::TopRight => (w, 0),
            Anchor::CenterLeft => (0, half_h),
            Anchor::Center => (half_w, half_h),
            Anchor::CenterRight => (w, half_h),
            Anchor::BottomLeft => (0, h),
            Anchor::BottomCenter => (half_w, h),
            Anchor::BottomRight => (w, h),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum Position {
    Absolute {
        monitor_identity: String,
        x: i32,
        y: i32,
    },
    Anchored {
        monitor_identity: String,
        anchor: Anchor,
        #[serde(default)]
        offset_x: i32,
        #[serde(default)]
        offset_y: i32,
    },
}

impl Position {
    pub fn monitor_identity(&self) -> &str {
        match self {
            Position::Absolute { monitor_identity, .. } => monitor_identity,
            Position::Anchored { monitor_identity, .. } => monitor_identity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn anchor_points_are_correct() {
        assert_eq!(Anchor::TopLeft.anchor_point(800, 600), (0, 0));
        assert_eq!(Anchor::TopCenter.anchor_point(800, 600), (400, 0));
        assert_eq!(Anchor::TopRight.anchor_point(800, 600), (800, 0));
        assert_eq!(Anchor::Center.anchor_point(800, 600), (400, 300));
        assert_eq!(Anchor::BottomRight.anchor_point(800, 600), (800, 600));
    }

    #[test]
    fn absolute_position_round_trips_through_toml() {
        let pos = Position::Absolute {
            monitor_identity: "eDP-1".into(),
            x: 24,
            y: 240,
        };
        let toml_str = toml::to_string_pretty(&pos).unwrap();
        let parsed: Position = toml::from_str(&toml_str).unwrap();
        assert_eq!(pos, parsed);
    }

    #[test]
    fn anchored_position_round_trips_through_toml() {
        let pos = Position::Anchored {
            monitor_identity: "eDP-1".into(),
            anchor: Anchor::TopLeft,
            offset_x: 24,
            offset_y: 240,
        };
        let toml_str = toml::to_string_pretty(&pos).unwrap();
        let parsed: Position = toml::from_str(&toml_str).unwrap();
        assert_eq!(pos, parsed);
    }

    #[test]
    fn position_returns_its_monitor_identity() {
        let abs = Position::Absolute { monitor_identity: "a".into(), x: 0, y: 0 };
        let anc = Position::Anchored { monitor_identity: "b".into(), anchor: Anchor::TopLeft, offset_x: 0, offset_y: 0 };
        assert_eq!(abs.monitor_identity(), "a");
        assert_eq!(anc.monitor_identity(), "b");
    }
}
