use crate::{
    config::{self, Config},
    edit_mode,
    window,
};
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Emitter, Manager, State};

const MIN_OPACITY: f64 = 0.1;

// Settings writes the same file `watch.rs` watches. The exact bytes written are
// recorded here and a matching file change is ignored, rather than suppressing
// the watcher for a time window: a self-write and a byte-identical hand edit are
// indistinguishable and both are no-ops, so this cannot swallow a real edit.
#[derive(Default)]
pub struct LastWrite(pub Mutex<String>);

pub fn should_reload(last_written: &str, current: &str) -> bool {
    last_written != current
}

fn is_hex_color(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && value[1..].chars().all(|character| character.is_ascii_hexdigit())
}

// Three near-identical retain loops rather than one generic helper: the section
// pass also filters by known label, the disk pass keys on a field, and the field
// pass keys on the value itself.
pub fn sanitize(mut config: Config) -> Config {
    config = config::normalize_instances(config);
    config.opacity = if config.opacity.is_finite() {
        config.opacity.clamp(MIN_OPACITY, 1.0)
    } else {
        1.0
    };
    config.text_opacity = if config.text_opacity.is_finite() {
        config.text_opacity.clamp(MIN_OPACITY, 1.0)
    } else {
        1.0
    };
    if !is_hex_color(&config.text_color) {
        config.text_color = "#292824".into();
    }
    if config.graph_color.as_deref().is_some_and(|color| !is_hex_color(color)) {
        config.graph_color = None;
    }
    if config.icon_color.as_deref().is_some_and(|color| !is_hex_color(color)) {
        config.icon_color = None;
    }
    config.sections.retain(|section| {
        window::LABELS.contains(&section.id.as_str())
    });
    for section in &mut config.sections {
        // A non-finite or non-positive scale reaches `transform: scale()` and
        // blanks the widget; any positive value is the user's business.
        if !section.scale.is_finite() || section.scale <= 0.0 {
            section.scale = 1.0;
        }
        if section.color_a.as_deref().is_some_and(|color| !is_hex_color(color)) {
            section.color_a = None;
        }
        if section.color_b.as_deref().is_some_and(|color| !is_hex_color(color)) {
            section.color_b = None;
        }
        if section.clock_color.as_deref().is_some_and(|color| !is_hex_color(color)) {
            section.clock_color = None;
        }
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

fn persist(app: &AppHandle, next: Config) {
    let next = sanitize(next);
    for section in next.sections.iter().filter(|section| section.id == "clock") {
        eprintln!("[dbg] persist {}: {}x{:?} layout {:?}", section.instance, section.width, section.height, section.clock_layout);
    }
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
    crate::audio::sync(app, &next);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || window::reconcile(&handle, &next));
}

pub fn apply(app: &AppHandle, next: Config) {
    persist(app, edit_mode::merge_live_geometry(app, next));
}

#[tauri::command]
pub fn update_config(app: AppHandle, next: Config) {
    for section in next.sections.iter().filter(|section| section.id == "clock") {
        eprintln!("[dbg] update_config in {}: {}x{:?} layout {:?}", section.instance, section.width, section.height, section.clock_layout);
    }
    apply(&app, next);
}

#[tauri::command]
pub fn preview_opacity(state: State<'_, Arc<RwLock<Config>>>, value: f64) {
    if let Ok(mut guard) = state.write() {
        guard.opacity = if value.is_finite() { value.clamp(MIN_OPACITY, 1.0) } else { 1.0 };
    }
}

#[tauri::command]
pub fn preview_text_opacity(state: State<'_, Arc<RwLock<Config>>>, value: f64) {
    if let Ok(mut guard) = state.write() {
        guard.text_opacity = if value.is_finite() { value.clamp(MIN_OPACITY, 1.0) } else { 1.0 };
    }
}

// Live-mutates the in-memory config so a corner drag can zoom at frame rate
// without a config file write per mouse move; `edit_mode::exit` persists it.
#[tauri::command]
pub fn update_widget_scale(state: State<'_, Arc<RwLock<Config>>>, id: String, scale: f64) {
    if let Ok(mut guard) = state.write() {
        if let Some(section) = guard.sections.iter_mut().find(|section| section.instance == id) {
            section.scale = if scale.is_finite() && scale > 0.0 { scale } else { 1.0 };
        }
    }
}

#[tauri::command]
pub fn remove_widget(app: AppHandle, instance: String) {
    let Some(state) = app.try_state::<Arc<RwLock<Config>>>() else { return };
    let Some(mut next) = state.read().ok().map(|guard| guard.clone()) else { return };
    next.sections.retain(|section| section.instance != instance);
    apply(&app, next);
    let _ = app.emit("widget-removed", instance);
}
