use crate::config::Config;
use crate::config_error::ConfigError;

pub const CURRENT_SCHEMA_VERSION: u32 = 2;
const MIN_OPACITY: f64 = 0.1;

pub fn check(config: &Config) -> Result<(), ConfigError> {
    if config.sections.is_empty() {
        return Err(ConfigError::Validation(
            "at least one section is required".into(),
        ));
    }
    for section in &config.sections {
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
    if !config.opacity.is_finite() || config.opacity < MIN_OPACITY || config.opacity > 1.0 {
        return Err(ConfigError::Validation(format!(
            "global opacity {} out of range",
            config.opacity
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::config::{Config, DiskPreference, SectionConfig};
    use crate::config_validate::check;
    use crate::position_model::{Anchor, Position};
    use std::collections::BTreeMap;

    fn valid() -> Config {
        Config {
            schema_version: 2,
            opacity: 0.9,
            text_opacity: 1.0,
            text_color: "#292824".into(),
            graph_color: None,
            icon_color: None,
            show_background: true,
            sections: vec![        SectionConfig {
                id: "cpu".into(), instance: "cpu".into(), enabled: true, show_header: true,
                position: Some(Position::Anchored { monitor_identity: "0".into(), anchor: Anchor::TopLeft, offset_x: 0, offset_y: 0 }),
                monitor: 0, x: 0, y: 0, width: 360, height: None, scale: 1.0,
                color_mode: None, color_a: None, color_b: None, gradient_direction: None,
                clock_font: None, clock_color: None, clock_seconds: false, clock_24h: false,
                clock_ampm: true, clock_pad: true, clock_layout: None, clock_align: None,
                date_weekday: true, date_format: None, date_color: None,
                interactive: false, timezone: None, plugin_id: None, plugin_interval: 30, plugin_config: None, extra: BTreeMap::new()
            }],
            system_fields: vec!["os".into()],
            show_cpu_cores: true,
            disks: vec![DiskPreference { id: "a".into(), enabled: true, label: None, extra: BTreeMap::new() }],
            monitor_catalog: BTreeMap::new(),
            extra: BTreeMap::new(),
        }
    }

    #[test]
    fn empty_sections_is_rejected() {
        assert!(check(&Config { sections: vec![], ..valid() }).is_err());
    }

    #[test]
    fn empty_section_id_is_rejected() {
        let mut cfg = valid();
        cfg.sections[0].id = "".into();
        assert!(check(&cfg).is_err());
    }

    #[test]
    fn valid_config_passes_check() {
        assert!(check(&valid()).is_ok());
    }
}
