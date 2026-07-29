use crate::config::Config;
use crate::config::Profile;
use crate::config_error::ConfigError;

pub const CURRENT_SCHEMA_VERSION: u32 = 3;
const MIN_OPACITY: f64 = 0.1;

pub fn check(config: &Config) -> Result<(), ConfigError> {
    match config.theme.as_str() {
        "light" | "dark" | "system" => {}
        _ => return Err(ConfigError::Validation(format!("invalid theme '{}'", config.theme))),
    }
    match config.byte_format.as_str() {
        "binary" | "decimal" => {}
        _ => return Err(ConfigError::Validation(format!("invalid byte_format '{}'", config.byte_format))),
    }
    match config.temperature_unit.as_str() {
        "celsius" | "fahrenheit" => {}
        _ => return Err(ConfigError::Validation(format!("invalid temperature_unit '{}'", config.temperature_unit))),
    }
    if !config.opacity.is_finite() || config.opacity < MIN_OPACITY || config.opacity > 1.0 {
        return Err(ConfigError::Validation(format!(
            "global opacity {} out of range",
            config.opacity
        )));
    }
    Ok(())
}

pub fn check_profile(profile: &Profile) -> Result<(), ConfigError> {
    if profile.sections.is_empty() {
        return Err(ConfigError::Validation(
            "at least one section is required".into(),
        ));
    }
    for section in &profile.sections {
        if section.id.is_empty() {
            return Err(ConfigError::Validation(
                "section id must be nonempty".into(),
            ));
        }
        if !section.scale.is_finite() || section.scale <= 0.0 {
            return Err(ConfigError::Validation(format!(
                "section {} has invalid scale {}",
                section.instance, section.scale
            )));
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, DiskPreference, SectionConfig};
    use crate::config_validate::{check, check_profile};
use crate::config::Profile;
    use crate::position_model::{Anchor, Position};
    use std::collections::BTreeMap;

    fn valid_config() -> Config {
        Config {
            schema_version: 3,
            active_profile: "default".into(),
            opacity: 0.9,
            text_opacity: 1.0,
            text_color: "#292824".into(),
            graph_color: None,
            icon_color: None,
            show_background: true,
            theme: "light".into(),
            accent_color: None,
            density: "compact".into(),
            font_scale: 1.0,
            sans_font: None,
            mono_font: None,
            byte_format: "binary".into(),
            temperature_unit: "celsius".into(),
            locale: None,
            extra: BTreeMap::new(),
        }
    }

    fn valid_profile() -> Profile {
        Profile {
            profile_schema_version: 1,
            sections: vec![SectionConfig {
                id: "cpu".into(), instance: "cpu".into(), enabled: true, show_header: true,
                position: Some(Position::Anchored { monitor_identity: "0".into(), anchor: Anchor::TopLeft, offset_x: 0, offset_y: 0 }),
                monitor: 0, x: 0, y: 0, width: 360, height: None, scale: 1.0,
                color_mode: None, color_a: None, color_b: None, gradient_direction: None,
                clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false,
                clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None,
                date_weekday: true, date_format: None, date_color: None,
                interactive: false, timezone: None, plugin_id: None, plugin_interval: 30, plugin_config: None, children: vec![], panel_gap: 0, panel_padding: 0, accent_color: None, transparent_surface: None, opacity_override: None, border_visible: None, radius_override: None, padding_override: None, font_scale: None, chart_colors: None, viz_bar_count: None, viz_min_hz: None, viz_max_hz: None, viz_gain: None, viz_smoothing: None, viz_decay: None, viz_mirror: None, viz_gap: None, viz_rounded_caps: None, viz_fps: None, extra: BTreeMap::new()
            }],
            system_fields: vec!["os".into()],
            show_cpu_cores: true,
            disks: vec![DiskPreference { id: "a".into(), enabled: true, label: None, extra: BTreeMap::new() }],
            collect_interval_ms: 1000, monitor_catalog: BTreeMap::new(), fullscreen: Default::default(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn empty_sections_is_rejected() {
        let mut p = valid_profile();
        p.sections = vec![];
        assert!(check_profile(&p).is_err());
    }

    #[test]
    fn empty_section_id_is_rejected() {
        let mut p = valid_profile();
        p.sections[0].id = "".into();
        assert!(check_profile(&p).is_err());
    }

    #[test]
    fn valid_config_passes_check() {
        assert!(check(&valid_config()).is_ok());
    }

    #[test]
    fn valid_profile_passes_check() {
        assert!(check_profile(&valid_profile()).is_ok());
    }
}
