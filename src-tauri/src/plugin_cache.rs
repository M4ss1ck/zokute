use crate::plugin_protocol::PluginOutput;
use std::collections::HashMap;
use std::sync::Mutex;

pub struct PluginCache {
    entries: Mutex<HashMap<String, CachedEntry>>,
}

struct CachedEntry {
    output: PluginOutput,
}

impl PluginCache {
    pub fn new() -> Self {
        PluginCache { entries: Mutex::new(HashMap::new()) }
    }

    pub fn get(&self, instance: &str) -> Option<PluginOutput> {
        self.entries.lock().ok().and_then(|g| g.get(instance).map(|e| e.output.clone()))
    }

    pub fn set(&self, instance: String, output: PluginOutput) {
        if let Ok(mut guard) = self.entries.lock() {
            guard.insert(instance, CachedEntry { output });
        }
    }

    pub fn remove(&self, instance: &str) {
        if let Ok(mut guard) = self.entries.lock() {
            guard.remove(instance);
        }
    }
}
