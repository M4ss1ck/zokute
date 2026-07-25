use crate::{
    config::{self, Config},
    edit_mode,
    window,
};
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Manager, State};

const MIN_OPACITY: f64 = 0.1;
const MIN_WIDTH: u32 = 120;

// Settings writes the same file `watch.rs` watches. The exact bytes written are
// recorded here and a matching file change is ignored, rather than suppressing
// the watcher for a time window: a self-write and a byte-identical hand edit are
// indistinguishable and both are no-ops, so this cannot swallow a real edit.
#[derive(Default)]
pub struct LastWrite(pub Mutex<String>);

pub fn should_reload(last_written: &str, current: &str) -> bool {
    last_written != current
}

// Three near-identical retain loops rather than one generic helper: the section
// pass also filters by known label, the disk pass keys on a field, and the field
// pass keys on the value itself.
pub fn sanitize(mut config: Config) -> Config {
    config.opacity = if config.opacity.is_finite() {
        config.opacity.clamp(MIN_OPACITY, 1.0)
    } else {
        1.0
    };
    let mut seen_sections: Vec<String> = Vec::new();
    config.sections.retain(|section| {
        if !window::LABELS.contains(&section.id.as_str()) {
            return false;
        }
        if seen_sections.iter().any(|seen| seen == &section.id) {
            return false;
        }
        seen_sections.push(section.id.clone());
        true
    });
    for section in &mut config.sections {
        section.width = section.width.max(MIN_WIDTH);
    }
    let mut seen_fields: Vec<String> = Vec::new();
    config.system_fields.retain(|field| {
        if seen_fields.iter().any(|seen| seen == field) {
            return false;
        }
        seen_fields.push(field.clone());
        true
    });
    let mut seen_disks: Vec<String> = Vec::new();
    config.disks.retain(|disk| {
        if seen_disks.iter().any(|seen| seen == &disk.id) {
            return false;
        }
        seen_disks.push(disk.id.clone());
        true
    });
    config
}

pub fn apply(app: &AppHandle, next: Config) {
    let next = sanitize(edit_mode::merge_live_geometry(app, next));
    let contents = config::serialize(&next);
    // Recorded before the write so the watcher cannot read the new file and
    // compare it against a stale record.
    if let Some(last) = app.try_state::<LastWrite>() {
        if let Ok(mut guard) = last.0.lock() {
            *guard = contents.clone();
        }
    }
    let _ = config::write_str(&config::path(), &contents);
    if let Some(state) = app.try_state::<Arc<RwLock<Config>>>() {
        if let Ok(mut guard) = state.write() {
            *guard = next.clone();
        }
    }
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || window::reconcile(&handle, &next));
}

#[tauri::command]
pub fn update_config(app: AppHandle, next: Config) {
    apply(&app, next);
}

#[tauri::command]
pub fn preview_opacity(state: State<'_, Arc<RwLock<Config>>>, value: f64) {
    if let Ok(mut guard) = state.write() {
        guard.opacity = if value.is_finite() { value.clamp(MIN_OPACITY, 1.0) } else { 1.0 };
    }
}
