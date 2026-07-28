use crate::config::Config;
use crate::plugin_cache::PluginCache;
use crate::plugin_discovery::PluginRegistry;
use crate::plugin_runner::{run_plugin, RunnerConfig};
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, RwLock};
use std::time::Instant;
use tauri::{AppHandle, Emitter};
use tokio::time::Duration;

pub struct PluginScheduler {
    pub running: Mutex<HashMap<String, Instant>>,
    pub active: AtomicBool,
}

impl PluginScheduler {
    pub fn new() -> Self {
        PluginScheduler {
            running: Mutex::new(HashMap::new()),
            active: AtomicBool::new(true),
        }
    }

    pub fn is_running(&self, instance: &str) -> bool {
        self.running.lock().ok().map_or(false, |g| g.contains_key(instance))
    }
}

pub async fn tick_plugins(
    app: AppHandle,
    config_state: Arc<RwLock<Config>>,
    registry: Arc<RwLock<PluginRegistry>>,
    cache: Arc<PluginCache>,
    scheduler: Arc<PluginScheduler>,
) {
    if !scheduler.active.load(Ordering::Relaxed) { return; }
    let config = config_state.read().ok().map(|g| g.clone());
    let Some(config) = config else { return };
    let registry = registry.read().ok().map(|g| g.clone());
    let Some(registry) = registry else { return };

    for section in &config.sections {
        if !section.enabled || !section.is_plugin() { continue; }
        let plugin_id = section.plugin_id.as_deref().unwrap_or("");
        if plugin_id.is_empty() { continue; }
        let Some(entry) = registry.get(plugin_id) else { continue };
        if entry.error.is_some() { continue; }

        let instance = section.instance.clone();
        if scheduler.is_running(&instance) { continue; }

        let manifest = Arc::new(entry.manifest.clone());
        let manifest_dir = entry.dir.clone();
        let instance_config = section.plugin_config.clone().unwrap_or(toml::Value::Table(Default::default()));
        let interval = manifest.interval.max(2);

        if let Ok(mut running) = scheduler.running.lock() {
            if let Some(last) = running.get(&instance) {
                if last.elapsed() < Duration::from_secs(interval) { continue; }
            }
            running.insert(instance.clone(), Instant::now());
        }

        let app_clone = app.clone();
        let cache_clone = cache.clone();
        let instance_clone = instance.clone();
        let sched_clone = scheduler.clone();

        tokio::spawn(async move {
            let cfg = RunnerConfig {
                manifest, manifest_dir,
                instance_id: instance_clone.clone(),
                config: instance_config,
            };
            let output = run_plugin(&cfg).await;
            cache_clone.set(instance_clone.clone(), output.clone());

            if let Ok(mut running) = sched_clone.running.lock() {
                running.remove(&instance_clone);
            }

            let event_payload = serde_json::json!({
                "instance": instance_clone,
            });
            let _ = app_clone.emit("plugin-output", event_payload);
        });
    }
}
