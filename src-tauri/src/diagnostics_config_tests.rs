use crate::config::{Config, Profile};
use std::collections::BTreeMap;


    fn sample_config() -> Config {
        Config {
            schema_version: 3, active_profile: "default".into(),
            opacity: 0.9, text_opacity: 1.0, text_color: "#000".into(),
            graph_color: None, icon_color: None, show_background: true,
            theme: "light".into(), accent_color: None, density: "compact".into(),
            font_scale: 1.0, sans_font: None, mono_font: None,
            byte_format: "binary".into(), temperature_unit: "celsius".into(),
            locale: None, extra: BTreeMap::new(),
        }
    }

    fn sample_profile() -> Profile {
        Profile {
            profile_schema_version: 1, sections: vec![], system_fields: vec![],
            show_cpu_cores: true, disks: vec![], collect_interval_ms: 1000,
            monitor_catalog: BTreeMap::new(), fullscreen: Default::default(), extra: BTreeMap::new(),
        }
    }

    #[test]
    fn show_config_contains_expected_sections() {
        let output = crate::diagnostics_config::show_config(false, &sample_config(), &sample_profile());
        assert!(output.contains("Global config"));
        assert!(output.contains("Profile"));
        assert!(output.contains("Sections"));
    }

    #[test]
    fn show_config_redacts_plugin_config() {
        let config = sample_config();
        let mut profile = sample_profile();
        profile.sections.push(crate::config::SectionConfig {
            id: "plugin".into(), instance: "plugin-1".into(), enabled: true,
            plugin_config: Some(toml::Value::Table(toml::map::Map::new())),
            position: None, show_header: true, monitor: 0, x: 0, y: 0,
            width: 360, height: None, scale: 1.0,
            color_mode: None, color_a: None, color_b: None, gradient_direction: None,
            clock_font: None, clock_color: None, clock_seconds: false,
            clock_24h: false, clock_ampm: true, clock_pad: true,
            clock_layout: None, clock_align: None,
            date_weekday: true, date_format: None, date_color: None,
            interactive: false, timezone: None, plugin_id: None, plugin_interval: 30,
            children: vec![], panel_gap: 0, panel_padding: 0, panel_dividers: true,
            accent_color: None, transparent_surface: None, opacity_override: None,
            border_visible: None, radius_override: None, padding_override: None,
            font_scale: None, chart_colors: None,
            viz_bar_count: None, viz_min_hz: None, viz_max_hz: None,
            viz_gain: None, viz_smoothing: None, viz_decay: None,
            viz_mirror: None, viz_gap: None, viz_rounded_caps: None, viz_fps: None,
            extra: BTreeMap::new(),
        });
        let output = crate::diagnostics_config::show_config(true, &config, &profile);
        assert!(output.contains("<redacted>"));
    }
