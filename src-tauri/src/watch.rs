use crate::{config, config::Profile, config_write, edit_mode, paths, window, watch_external::{ExternalChange, ExternalConfig}};
use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::sync::{Arc, RwLock};
use std::thread;
use tauri::{AppHandle, Emitter, Manager};

pub fn start(app: AppHandle, config_state: Arc<RwLock<config::Config>>, profile_state: Arc<RwLock<Profile>>) {
    let config_path = paths::config_path();
    let profiles_dir = paths::profiles_dir();
    app.manage(ExternalConfig(std::sync::Mutex::new(None)));
    let app_clone = app.clone();
    thread::spawn(move || {
        let config_parent = config_path.parent().expect("config parent").to_path_buf();
        let (tx, rx) = channel();
        let mut watcher = RecommendedWatcher::new(
            move |result| { let _ = tx.send(result); },
            notify::Config::default(),
        )
        .expect("config watcher");
        watcher.watch(&config_parent, RecursiveMode::NonRecursive).expect("watch config");
        let _ = std::fs::create_dir_all(&profiles_dir);
        watcher.watch(&profiles_dir, RecursiveMode::NonRecursive).expect("watch profiles");
        while let Ok(result) = rx.recv() {
            let Ok(event) = result else { continue };
            if event.paths.iter().any(|p| p == &config_path) {
                handle_config_change(&app_clone, &config_path, &config_state, &profile_state);
                continue;
            }
            for path in &event.paths {
                if path.parent() == Some(profiles_dir.as_path())
                    && path.extension().and_then(|e| e.to_str()) == Some("toml")
                {
                    handle_profile_change(&app_clone, path, &config_state, &profile_state);
                    break;
                }
            }
        }
    });
}

/// An open settings session holds an unsaved draft in memory. Re-reading the
/// file underneath it would silently discard the user's work, so external
/// changes wait until the session ends.
pub fn should_apply_external(session_active: bool, self_write: bool) -> bool {
    !self_write && !session_active
}

fn handle_config_change(app: &AppHandle, config_path: &std::path::Path, config_state: &Arc<RwLock<config::Config>>, profile_state: &Arc<RwLock<Profile>>) {
    let Ok(contents) = std::fs::read_to_string(config_path) else { return };
    if should_apply_external(edit_mode::session_active(app), is_self_write(app, &contents)) {
        if let Ok(next) = config::parse(&contents) {
            { let mut g = config_state.write().expect("config lock"); *g = next.clone(); }
            reload_active_profile(app, &next.active_profile, profile_state);
        }
        return;
    }
    if is_self_write(app, &contents) { return; }
    if let Ok(next) = config::parse(&contents) {
        store_external(app, ExternalChange::Config(contents, next));
        let _ = app.emit("external-config-changed", true);
    }
}

fn handle_profile_change(app: &AppHandle, profile_path: &std::path::Path, config_state: &Arc<RwLock<config::Config>>, profile_state: &Arc<RwLock<Profile>>) {
    let Ok(contents) = std::fs::read_to_string(profile_path) else { return };
    let profile_name = match profile_path.file_stem().and_then(|s| s.to_str()) {
        Some(name) => name.to_string(),
        None => return,
    };
    let active_profile = config_state.read().ok().map(|g| g.active_profile.clone()).unwrap_or_else(|| "default".into());
    if profile_name != active_profile {
        let _ = app.emit("profile-catalog-changed", ());
        return;
    }
    if should_apply_external(edit_mode::session_active(app), is_self_write(app, &contents)) {
        if let Ok(profile) = toml::from_str::<Profile>(&contents) {
            { let mut g = profile_state.write().expect("profile lock"); *g = profile.clone(); }
            crate::audio::sync(app, &profile);
            let a = app.clone(); let s = profile_state.clone();
            let _ = app.run_on_main_thread(move || {
                if let Ok(p) = s.read() { window::reconcile(&a, &p); }
            });
        }
        return;
    }
    if is_self_write(app, &contents) { return; }
    if let Ok(profile) = toml::from_str::<Profile>(&contents) {
        store_external(app, ExternalChange::Profile(contents, profile));
        let _ = app.emit("external-profile-changed", true);
    }
}

fn is_self_write(app: &AppHandle, contents: &str) -> bool {
    app.try_state::<config_write::LastWrite>()
        .and_then(|last| last.0.lock().ok().map(|guard| !config_write::should_reload(&guard, contents)))
        .unwrap_or(false)
}

fn store_external(app: &AppHandle, change: ExternalChange) {
    if let Some(holder) = app.try_state::<ExternalConfig>() {
        if let Ok(mut guard) = holder.0.lock() { *guard = Some(change); }
    }
}

fn reload_active_profile(app: &AppHandle, active_profile: &str, profile_state: &Arc<RwLock<Profile>>) {
    let profile_path = paths::profile_path(active_profile);
    let Ok(source) = std::fs::read_to_string(&profile_path) else { return };
    let Ok(profile) = toml::from_str::<Profile>(&source) else { return };
    { let mut g = profile_state.write().expect("profile lock"); *g = profile.clone(); }
    crate::audio::sync(app, &profile);
    let a = app.clone(); let s = profile_state.clone();
    let _ = app.run_on_main_thread(move || {
        if let Ok(p) = s.read() { window::reconcile(&a, &p); }
    });
}
