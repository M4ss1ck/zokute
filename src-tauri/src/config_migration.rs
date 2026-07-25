use super::{Config, DiskPreference, SectionConfig, DEFAULT_SYSTEM_FIELDS, KNOWN_SECTION_IDS};
use serde::Deserialize;

const SECTION_Y_OFFSETS: [i32; 5] = [0, 216, 376, 480, 640];

#[derive(Deserialize)]
struct LegacyConfig {
    monitor: usize,
    x: i32,
    y: i32,
    width: u32,
    opacity: f64,
    widgets: Vec<String>,
}

pub(super) fn migrate(source: &str, detected_disks: &[String]) -> Result<Config, toml::de::Error> {
    let legacy: LegacyConfig = toml::from_str(source)?;
    Ok(Config {
        opacity: legacy.opacity,
        text_opacity: 1.0,
        text_color: super::default_text_color(),
        graph_color: None,
        icon_color: None,
        show_background: true,
        sections: KNOWN_SECTION_IDS
            .iter()
            .zip(SECTION_Y_OFFSETS)
            .map(|(id, y)| SectionConfig {
                id: id.to_string(),
                instance: id.to_string(),
                enabled: legacy.widgets.iter().any(|widget| widget == id),
                show_header: true,
                monitor: legacy.monitor,
                x: legacy.x,
                y: legacy.y + y,
                width: legacy.width,
                scale: 1.0,
            })
            .collect(),
        system_fields: DEFAULT_SYSTEM_FIELDS.iter().map(|field| field.to_string()).collect(),
        show_cpu_cores: true,
        disks: detected_disks.iter().map(|id| DiskPreference { id: id.clone(), enabled: true, label: None }).collect(),
    })
}
