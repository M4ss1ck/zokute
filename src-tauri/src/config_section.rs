use crate::position_model::Position;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectionConfig {
    pub id: String,
    #[serde(default)]
    pub instance: String,
    pub enabled: bool,
    #[serde(default = "super::config_defaults::default_true")] pub show_header: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    #[serde(default, skip_serializing)]
    pub monitor: usize,
    #[serde(default, skip_serializing)]
    pub x: i32,
    #[serde(default, skip_serializing)]
    pub y: i32,
    pub width: u32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(default = "super::config_defaults::default_scale")]
    pub scale: f64,
    #[serde(default)]
    pub interactive: bool,
    #[serde(default)]
    pub children: Vec<SectionConfig>,
    #[serde(default)]
    pub panel_gap: u32,
    #[serde(default)]
    pub panel_padding: u32,
    #[serde(default = "super::config_defaults::default_true")]
    pub panel_dividers: bool,
    #[serde(default)]
    pub plugin_id: Option<String>,
    #[serde(default = "default_plugin_interval")]
    pub plugin_interval: u64,
    #[serde(default)]
    pub plugin_config: Option<toml::Value>,
    #[serde(default)]
    pub color_mode: Option<String>,
    #[serde(default)]
    pub color_a: Option<String>,
    #[serde(default)]
    pub color_b: Option<String>,
    #[serde(default)]
    pub gradient_direction: Option<String>,
    #[serde(default)]
    pub clock_font: Option<String>,
    #[serde(default)]
    pub clock_color: Option<String>,
    #[serde(default)]
    pub clock_seconds: bool,
    #[serde(default)]
    pub clock_24h: bool,
    #[serde(default = "super::config_defaults::default_true")]
    pub clock_ampm: bool,
    #[serde(default = "super::config_defaults::default_true")]
    pub clock_pad: bool,
    #[serde(default)]
    pub clock_layout: Option<String>,
    #[serde(default)]
    pub clock_align: Option<String>,
    #[serde(default)]
    pub timezone: Option<String>,
    #[serde(default = "super::config_defaults::default_true")]
    pub date_weekday: bool,
    #[serde(default)]
    pub date_format: Option<String>,
    #[serde(default)]
    pub date_color: Option<String>,
    #[serde(default)]
    pub accent_color: Option<String>,
    #[serde(default)]
    pub transparent_surface: Option<bool>,
    #[serde(default)]
    pub opacity_override: Option<f64>,
    #[serde(default)]
    pub border_visible: Option<bool>,
    #[serde(default)]
    pub radius_override: Option<u32>,
    #[serde(default)]
    pub padding_override: Option<u32>,
    #[serde(default)]
    pub font_scale: Option<f64>,
    #[serde(default)]
    pub chart_colors: Option<Vec<String>>,
    #[serde(default)]
    pub viz_bar_count: Option<u32>,
    #[serde(default)]
    pub viz_min_hz: Option<f64>,
    #[serde(default)]
    pub viz_max_hz: Option<f64>,
    #[serde(default)]
    pub viz_gain: Option<f64>,
    #[serde(default)]
    pub viz_smoothing: Option<f64>,
    #[serde(default)]
    pub viz_decay: Option<f64>,
    #[serde(default)]
    pub viz_mirror: Option<bool>,
    #[serde(default)]
    pub viz_gap: Option<u32>,
    #[serde(default)]
    pub viz_rounded_caps: Option<bool>,
    #[serde(default)]
    pub viz_fps: Option<u32>,
    #[serde(flatten)]
    pub extra: BTreeMap<String, toml::Value>,
}

fn default_plugin_interval() -> u64 { 30 }

impl SectionConfig {
    pub fn is_plugin(&self) -> bool {
        self.id == "plugin"
    }

    pub fn is_panel(&self) -> bool {
        self.id == "panel"
    }

    pub fn effective_position(&self) -> Position {
        if let Some(ref pos) = self.position {
            pos.clone()
        } else {
            Position::Anchored {
                monitor_identity: self.monitor.to_string(),
                anchor: crate::position_model::Anchor::TopLeft,
                offset_x: self.x,
                offset_y: self.y,
            }
        }
    }

    pub fn set_position(&mut self, position: Position) {
        self.position = Some(position);
    }
}
