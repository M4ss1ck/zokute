use crate::config::SectionConfig;
use std::collections::BTreeMap;

impl Default for SectionConfig {
    fn default() -> Self {
        SectionConfig {
            id: String::new(), instance: String::new(), enabled: false, show_header: true,
            position: None, monitor: 0, x: 0, y: 0, width: 360, height: None, scale: 1.0,
            interactive: false, children: Vec::new(), panel_gap: 0, panel_padding: 0,
            plugin_id: None, plugin_interval: 30, plugin_config: None,
            color_mode: None, color_a: None, color_b: None, gradient_direction: None,
            clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false,
            clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None,
            timezone: None, date_weekday: true, date_format: None, date_color: None,
            accent_color: None, transparent_surface: None, opacity_override: None,
            border_visible: None, radius_override: None, padding_override: None,
            font_scale: None, chart_colors: None,
            viz_bar_count: None, viz_min_hz: None, viz_max_hz: None, viz_gain: None,
            viz_smoothing: None, viz_decay: None, viz_mirror: None, viz_gap: None,
            viz_rounded_caps: None, viz_fps: None,
            extra: BTreeMap::new(),
        }
    }
}
