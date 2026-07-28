use crate::{
    atomic_file, config,
    config::Config,
    config_error::ConfigError,
    config_validate,
    edit_geometry, paths, window,
};
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Emitter, Manager, State};
#[path = "config_write_helpers.rs"] mod helpers;
use helpers::{clamp_opacity, is_hex_color, is_valid_theme};

pub use helpers::should_reload;

#[derive(Default)]
pub struct LastWrite(pub Mutex<String>);

pub fn sanitize(mut config: Config) -> Config {
    config.schema_version = 1;
    config = config::normalize_instances(config);
    config.opacity = clamp_opacity(config.opacity);
    config.text_opacity = clamp_opacity(config.text_opacity);
    if !is_valid_theme(&config.theme) {
        config.theme = "light".into();
    }
    if !is_hex_color(&config.text_color) {
        config.text_color = "#292824".into();
    }
    if config.graph_color.as_deref().is_some_and(|color| !is_hex_color(color)) {
        config.graph_color = None;
    }
    if config.icon_color.as_deref().is_some_and(|color| !is_hex_color(color)) {
        config.icon_color = None;
    }
    if config.accent_color.as_deref().is_some_and(|color| !is_hex_color(color)) {
        config.accent_color = None;
    }
    if !matches!(config.density.as_str(), "compact" | "comfortable") { config.density = "compact".into(); }
    config.font_scale = config.font_scale.clamp(0.5, 2.0);
    if !matches!(config.byte_format.as_str(), "binary" | "decimal") { config.byte_format = "binary".into(); }
    if !matches!(config.temperature_unit.as_str(), "celsius" | "fahrenheit") { config.temperature_unit = "celsius".into(); }
    config.sections.retain(|section| {
        window::LABELS.contains(&section.id.as_str()) || section.id == "plugin" || section.id == "panel"
    });
    for section in &mut config.sections {
        if !section.scale.is_finite() || section.scale <= 0.0 {
            section.scale = 1.0;
        }
        if section.color_a.as_deref().is_some_and(|color| !is_hex_color(color)) { section.color_a = None; }
        if section.color_b.as_deref().is_some_and(|color| !is_hex_color(color)) { section.color_b = None; }
        if section.clock_color.as_deref().is_some_and(|color| !is_hex_color(color)) { section.clock_color = None; }
        if section.date_color.as_deref().is_some_and(|color| !is_hex_color(color)) { section.date_color = None; }
        if section.accent_color.as_deref().is_some_and(|color| !is_hex_color(color)) { section.accent_color = None; }
        if let Some(o) = section.opacity_override { section.opacity_override = Some(o.clamp(0.1, 1.0)); }
        if let Some(r) = section.radius_override { section.radius_override = Some(r.clamp(2, 24)); }
        if let Some(p) = section.padding_override { section.padding_override = Some(p.clamp(0, 32)); }
        if let Some(f) = section.font_scale { section.font_scale = Some(f.clamp(0.5, 2.0)); }
        if section.chart_colors.as_ref().is_some_and(|colors| colors.iter().any(|c| !is_hex_color(c))) { section.chart_colors = None; }
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

fn persist(app: &AppHandle, next: Config) -> Result<(), ConfigError> {
    config_validate::check(&next)?;
    let next = sanitize(next);
    let contents = config::serialize(&next);
    if let Some(last) = app.try_state::<LastWrite>() {
        if let Ok(mut guard) = last.0.lock() {
            *guard = contents.clone();
        }
    }
    atomic_file::write(&paths::config_path(), &contents)?;
    if let Some(state) = app.try_state::<Arc<RwLock<Config>>>() {
        if let Ok(mut guard) = state.write() {
            *guard = next.clone();
        }
    }
    crate::audio::sync(app, &next);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || window::reconcile(&handle, &next));
    Ok(())
}

pub fn apply(app: &AppHandle, next: Config) {
    let next = edit_geometry::merge_live_geometry(app, next);
    if let Err(error) = persist(app, next) {
        eprintln!("apply: {error}");
    }
}

#[tauri::command]
pub fn update_config(app: AppHandle, next: Config) {
    apply(&app, next);
}

#[tauri::command]
pub fn preview_opacity(state: State<'_, Arc<RwLock<Config>>>, value: f64) {
    if let Ok(mut guard) = state.write() {
        guard.opacity = clamp_opacity(value);
    }
}

#[tauri::command]
pub fn preview_text_opacity(state: State<'_, Arc<RwLock<Config>>>, value: f64) {
    if let Ok(mut guard) = state.write() {
        guard.text_opacity = clamp_opacity(value);
    }
}

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
