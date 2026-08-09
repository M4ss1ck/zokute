use crate::config::{Config, Profile};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use tauri::{AppHandle, Manager};

/// Guards the one-shot start of the collector, audio, and config watcher.
///
/// Startup and onboarding are two entry points into the same work: a normal
/// launch starts it directly, while a first run cannot until onboarding has
/// written a config. Whichever arrives first wins; the other must not start a
/// second collector.
#[derive(Clone, Default)]
pub struct StartLatch(Arc<AtomicBool>);

impl StartLatch {
    pub fn claim(&self) -> bool {
        !self.0.swap(true, Ordering::SeqCst)
    }

    #[cfg(test)]
    pub fn started(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Starts the background work that needs a valid config: metric collection, the
/// audio capture demand check, and the external-edit watcher.
pub fn start(app: &AppHandle, config_state: Arc<RwLock<Config>>, profile_state: Arc<RwLock<Profile>>) {
    let Some(latch) = app.try_state::<StartLatch>() else { return };
    if !latch.claim() {
        return;
    }
    let profile = profile_state.read().ok().map(|guard| guard.clone());
    if let Some(profile) = profile {
        crate::audio::start(app.clone(), &profile);
    }
    crate::watch::start(app.clone(), config_state.clone(), profile_state.clone());
    tauri::async_runtime::spawn(crate::collect::run(app.clone(), config_state, profile_state));
}
