use super::{default_scale, default_true};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SectionConfig {
    pub id: String,
    #[serde(default)]
    pub instance: String,
    pub enabled: bool,
    #[serde(default = "default_true")] pub show_header: bool,
    pub monitor: usize,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    // Absent means "whatever the content needs", until a vertical drag sets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<u32>,
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default)]
    pub color_mode: Option<String>,
    #[serde(default)]
    pub color_a: Option<String>,
    #[serde(default)]
    pub color_b: Option<String>,
    #[serde(default)]
    pub clock_font: Option<String>,
    #[serde(default)]
    pub clock_color: Option<String>,
    #[serde(default)]
    pub clock_seconds: bool,
    #[serde(default)]
    pub clock_24h: bool,
    #[serde(default = "default_true")]
    pub clock_ampm: bool,
    #[serde(default = "default_true")]
    pub clock_pad: bool,
    #[serde(default)]
    pub clock_layout: Option<String>,
    #[serde(default)]
    pub clock_align: Option<String>,
    #[serde(default = "default_true")]
    pub date_weekday: bool,
    #[serde(default)]
    pub date_format: Option<String>,
    #[serde(default)]
    pub date_color: Option<String>,
}
