use super::{
    default_scale, default_text_color, default_text_opacity, default_true, Config, DiskPreference,
    SectionConfig, DEFAULT_SYSTEM_FIELDS,
};

const SECTION_Y_OFFSETS: [i32; 5] = [0, 216, 376, 480, 640];
const DEFAULT_SECTION_IDS: [&str; 5] = ["system", "cpu", "memory", "disk", "network"];

pub(super) fn fresh(detected_disks: &[String]) -> Config {
    Config {
        opacity: 0.92,
        text_opacity: default_text_opacity(),
        text_color: default_text_color(),
        graph_color: None,
        icon_color: None,
        show_background: default_true(),
        sections: DEFAULT_SECTION_IDS.iter().zip(SECTION_Y_OFFSETS).map(|(id, y)| SectionConfig {
                id: id.to_string(),
                instance: id.to_string(),
                enabled: true,
                show_header: true,
                monitor: 0,
                x: 24,
                y: 24 + y,
                width: 360,
                height: None,
                scale: default_scale(),
                color_mode: None,
                color_a: None,
                color_b: None,
                gradient_direction: None,
                clock_font: None,
                clock_color: None,
                clock_seconds: false,
                clock_24h: false,
                clock_ampm: true,
                clock_pad: true,
                clock_layout: None,
                clock_align: None,
                date_weekday: true,
                date_format: None,
                date_color: None,
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
