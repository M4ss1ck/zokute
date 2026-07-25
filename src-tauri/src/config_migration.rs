use super::{section, Config, DiskPreference, DEFAULT_SYSTEM_FIELDS, KNOWN_SECTION_IDS, SECTION_Y_OFFSETS};
use serde::Deserialize;

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
        show_background: true,
        sections: KNOWN_SECTION_IDS
            .iter()
            .zip(SECTION_Y_OFFSETS)
            .map(|(id, y)| section(id, legacy.widgets.iter().any(|widget| widget == id), legacy.monitor, legacy.x, legacy.y + y, legacy.width))
            .collect(),
        system_fields: DEFAULT_SYSTEM_FIELDS.iter().map(|field| field.to_string()).collect(),
        show_cpu_cores: true,
        disks: detected_disks.iter().map(|id| DiskPreference { id: id.clone(), enabled: true, label: None }).collect(),
    })
}
