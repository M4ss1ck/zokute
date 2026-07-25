use super::{
    default_scale, default_text_color, default_text_opacity, default_true, Config, DiskPreference,
    SectionConfig, DEFAULT_SYSTEM_FIELDS, KNOWN_SECTION_IDS,
};

const SECTION_Y_OFFSETS: [i32; 5] = [0, 216, 376, 480, 640];

pub(super) fn fresh(detected_disks: &[String]) -> Config {
    Config {
        opacity: 0.92,
        text_opacity: default_text_opacity(),
        text_color: default_text_color(),
        graph_color: None,
        icon_color: None,
        show_background: default_true(),
        sections: KNOWN_SECTION_IDS
            .iter()
            .zip(SECTION_Y_OFFSETS)
            .map(|(id, y)| SectionConfig {
                id: id.to_string(),
                instance: id.to_string(),
                enabled: true,
                show_header: true,
                monitor: 0,
                x: 24,
                y: 24 + y,
                width: 360,
                scale: default_scale(),
            })
            .collect(),
        system_fields: DEFAULT_SYSTEM_FIELDS.iter().map(|field| field.to_string()).collect(),
        show_cpu_cores: true,
        disks: detected_disks
            .iter()
            .map(|id| DiskPreference { id: id.clone(), enabled: true, label: None })
            .collect(),
    }
}
