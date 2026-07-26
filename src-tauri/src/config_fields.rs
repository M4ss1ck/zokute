use super::{Config, DEFAULT_SYSTEM_FIELDS};

const RENAMED_FIELD_IDS: [(&str, &str); 2] = [("desktop", "de"), ("window_manager", "wm")];

// The old ids are what identify a pre-rework config, so this runs at most once.
// Running the append unconditionally would resurrect every field the user had
// hidden, since hiding a field removes it from the list.
pub(crate) fn upgrade_system_fields(mut config: Config) -> Config {
    let is_legacy = config
        .system_fields
        .iter()
        .any(|field| RENAMED_FIELD_IDS.iter().any(|(old, _)| old == field));
    if !is_legacy {
        return config;
    }
    for field in &mut config.system_fields {
        if let Some((_, replacement)) = RENAMED_FIELD_IDS.iter().find(|(old, _)| old == field) {
            *field = (*replacement).to_string();
        }
    }
    for id in DEFAULT_SYSTEM_FIELDS {
        if !config.system_fields.iter().any(|field| field == id) {
            config.system_fields.push(id.to_string());
        }
    }
    config
}
