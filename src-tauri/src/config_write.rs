use crate::{
    atomic_file, config,
    config::Config,
    config_error::ConfigError,
    config_validate,
    edit_geometry, paths, window,
};
use crate::config::Profile;
use std::sync::{Arc, Mutex, RwLock};
use tauri::{AppHandle, Emitter, Manager, State};
#[path = "config_write_helpers.rs"] mod helpers;
use helpers::{clamp_opacity, is_hex_color, is_valid_theme};
#[path = "config_sanitize_profile.rs"] mod sanitize_profile_mod;
#[path = "config_profile_commands.rs"] pub(crate) mod profile_commands_mod;

pub use helpers::should_reload;
pub use sanitize_profile_mod::sanitize_profile;

#[derive(Default)]
pub struct LastWrite(pub Mutex<String>);

pub fn sanitize(mut config: Config) -> Config {
    config.schema_version = config_validate::CURRENT_SCHEMA_VERSION;
    config.opacity = clamp_opacity(config.opacity);
    config.text_opacity = clamp_opacity(config.text_opacity);
    if !is_valid_theme(&config.theme) { config.theme = "light".into(); }
    if !is_hex_color(&config.text_color) { config.text_color = "#292824".into(); }
    if config.graph_color.as_deref().is_some_and(|color| !is_hex_color(color)) { config.graph_color = None; }
    if config.icon_color.as_deref().is_some_and(|color| !is_hex_color(color)) { config.icon_color = None; }
    if config.accent_color.as_deref().is_some_and(|color| !is_hex_color(color)) { config.accent_color = None; }
    if !matches!(config.density.as_str(), "compact" | "comfortable") { config.density = "compact".into(); }
    config.font_scale = config.font_scale.clamp(0.5, 2.0);
    if !matches!(config.byte_format.as_str(), "binary" | "decimal") { config.byte_format = "binary".into(); }
    if !matches!(config.temperature_unit.as_str(), "celsius" | "fahrenheit") { config.temperature_unit = "celsius".into(); }
    config
}

/// Everything a config change does except touch the disk: validate, sanitize,
/// publish to the shared state, resync audio, reconcile windows. Drafts stop
/// here; only `persist` continues on to the filesystem.
pub fn apply_in_memory(app: &AppHandle, next_config: Config, next_profile: Profile) -> Result<(), ConfigError> {
    config_validate::check(&next_config)?;
    config_validate::check_profile(&next_profile)?;
    let next_config = sanitize(next_config);
    // Reconcile below calls position::apply, which writes the stored position
    // back onto every window. Without folding live geometry in first, any draft
    // change made after a drag would snap the dragged widget back and the drag
    // would be lost for good. No-op unless Arrange is on.
    let next_profile = sanitize_profile(edit_geometry::merge_live_geometry(app, next_profile));
    if let Some(state) = app.try_state::<Arc<RwLock<Config>>>() {
        if let Ok(mut guard) = state.write() { *guard = next_config; }
    }
    if let Some(state) = app.try_state::<Arc<RwLock<Profile>>>() {
        if let Ok(mut guard) = state.write() { *guard = next_profile.clone(); }
    }
    crate::audio::sync(app, &next_profile);
    let handle = app.clone();
    let _ = app.run_on_main_thread(move || window::reconcile(&handle, &next_profile));
    Ok(())
}

fn persist(app: &AppHandle, next_config: Config, next_profile: Profile) -> Result<(), ConfigError> {
    config_validate::check(&next_config)?;
    config_validate::check_profile(&next_profile)?;
    let sanitized_config = sanitize(next_config.clone());
    let sanitized_profile = sanitize_profile(next_profile.clone());
    let config_contents = config::serialize(&sanitized_config);
    let profile_contents = config::serialize_profile(&sanitized_profile);
    if let Some(last) = app.try_state::<LastWrite>() {
        if let Ok(mut guard) = last.0.lock() { *guard = config_contents.clone(); }
    }
    atomic_file::write(&paths::config_path(), &config_contents)?;
    let _ = std::fs::create_dir_all(paths::profiles_dir());
    atomic_file::write(&paths::profile_path(&sanitized_config.active_profile), &profile_contents)?;
    apply_in_memory(app, next_config, next_profile)
}

pub fn apply(app: &AppHandle, next_config: Config, next_profile: Profile) {
    let next_profile = edit_geometry::merge_live_geometry(app, next_profile);
    if let Err(error) = persist(app, next_config, next_profile) {
        eprintln!("apply: {error}");
    }
}

#[tauri::command]
pub fn draft_config(app: AppHandle, next: Config) {
    let profile = app.try_state::<Arc<RwLock<Profile>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone()))
        .unwrap_or_else(|| config::fresh_profile(&[]));
    if let Err(error) = apply_in_memory(&app, next, profile) {
        eprintln!("draft_config: {error}");
    }
}

#[tauri::command]
pub fn preview_opacity(state: State<'_, Arc<RwLock<Config>>>, value: f64) {
    if let Ok(mut guard) = state.write() { guard.opacity = clamp_opacity(value); }
}

#[tauri::command]
pub fn preview_text_opacity(state: State<'_, Arc<RwLock<Config>>>, value: f64) {
    if let Ok(mut guard) = state.write() { guard.text_opacity = clamp_opacity(value); }
}

#[tauri::command]
pub fn update_widget_scale(state: State<'_, Arc<RwLock<Profile>>>, id: String, scale: f64) {
    if let Ok(mut guard) = state.write() {
        if let Some(section) = guard.sections.iter_mut().find(|section| section.instance == id) {
            section.scale = if scale.is_finite() && scale > 0.0 { scale } else { 1.0 };
        }
    }
}

#[tauri::command]
pub fn remove_widget(app: AppHandle, instance: String) {
    let Some(state) = app.try_state::<Arc<RwLock<Profile>>>() else { return };
    let Some(mut next_profile) = state.read().ok().map(|guard| guard.clone()) else { return };
    next_profile.sections.retain(|section| section.instance != instance);
    let config_state = app.try_state::<Arc<RwLock<Config>>>()
        .and_then(|s| s.read().ok().map(|g| g.clone()));
    let Some(config) = config_state else { return };
    if let Err(error) = apply_in_memory(&app, config, next_profile) {
        eprintln!("remove_widget: {error}");
        return;
    }
    let _ = app.emit("widget-removed", instance);
}
