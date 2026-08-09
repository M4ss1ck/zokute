#[cfg(test)]
use super::DEFAULT_SYSTEM_FIELDS;
#[cfg(test)]
use crate::config::Profile;

#[cfg(test)]
const RENAMED_FIELD_IDS: [(&str, &str); 2] = [("desktop", "de"), ("window_manager", "wm")];

// The old ids are what identify a pre-rework config, so this runs at most once.
// Running the append unconditionally would resurrect every field the user had
// hidden, since hiding a field removes it from the list.
#[cfg(test)]
pub(crate) fn upgrade_system_fields(mut profile: Profile) -> Profile {
    let is_legacy = profile
        .system_fields
        .iter()
        .any(|field| RENAMED_FIELD_IDS.iter().any(|(old, _)| old == field));
    if !is_legacy {
        return profile;
    }
    for field in &mut profile.system_fields {
        if let Some((_, replacement)) = RENAMED_FIELD_IDS.iter().find(|(old, _)| old == field) {
            *field = (*replacement).to_string();
        }
    }
    for id in DEFAULT_SYSTEM_FIELDS {
        if !profile.system_fields.iter().any(|field| field == id) {
            profile.system_fields.push(id.to_string());
        }
    }
    profile
}
