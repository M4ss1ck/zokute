use crate::fullscreen::{effect, Effect, FullscreenConfig, Policy};
use crate::fullscreen_x11::Session;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tauri::{AppHandle, Manager};

/// FULL-002 caps the fallback at four checks per second.
const POLL_INTERVAL: Duration = Duration::from_millis(250);

#[derive(Clone, Default)]
pub struct FullscreenState {
    active: Arc<AtomicBool>,
    dim: Arc<RwLock<Option<f64>>>,
    degraded: Arc<AtomicBool>,
}

impl FullscreenState {
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }

    pub fn dim_opacity(&self) -> Option<f64> {
        *self.dim.read().expect("dim")
    }

    /// True when event subscription failed and the capped poll is driving
    /// detection instead. Reported by `zokute diagnostics`.
    pub fn is_degraded(&self) -> bool {
        self.degraded.load(Ordering::Relaxed)
    }
}

pub fn start(app: AppHandle) {
    app.manage(FullscreenState::default());
    std::thread::spawn(move || run(app));
}

fn run(app: AppHandle) {
    match Session::connect() {
        Ok(mut session) => event_loop(&mut session, &app),
        Err(_) => {
            if let Some(state) = app.try_state::<FullscreenState>() {
                state.degraded.store(true, Ordering::Relaxed);
            }
            poll_loop(&app);
        }
    }
}

fn config_of(app: &AppHandle) -> FullscreenConfig {
    app.try_state::<Arc<RwLock<crate::config::Profile>>>()
        .and_then(|state| state.read().ok().map(|profile| profile.fullscreen.clone()))
        .unwrap_or_default()
}

fn apply(app: &AppHandle, active: bool) {
    let Some(state) = app.try_state::<FullscreenState>() else { return };
    let config = config_of(app);
    state.active.store(active, Ordering::Relaxed);
    match effect(active, crate::edit_mode::is_active(app), &config) {
        Effect::Hide => {
            *state.dim.write().expect("dim") = None;
            crate::window::lifecycle::hide_all(app);
        }
        Effect::Dim(opacity) => {
            *state.dim.write().expect("dim") = Some(opacity);
        }
        Effect::None => {
            *state.dim.write().expect("dim") = None;
            if config.behavior == "hide" {
                crate::window::lifecycle::show_all(app);
            }
        }
    }
}

fn event_loop(session: &mut Session, app: &AppHandle) {
    let mut policy = Policy::new();
    let mut deadline: Option<(Instant, u64)> = None;
    loop {
        session.watch_active();
        if let Some(pending) = policy.observe(session.is_fullscreen(), Instant::now(), &config_of(app)) {
            deadline = Some((pending.due, pending.generation));
        }
        match deadline {
            Some((due, generation)) if Instant::now() >= due => {
                // A stale generation resolves to None, so a reversal during the
                // delay simply drops the decision.
                if let Some(active) = policy.resolve(generation) {
                    apply(app, active);
                }
                deadline = None;
            }
            Some((due, _)) => session.wait_until(due),
            None => {
                if session.wait_for_change().is_err() {
                    return;
                }
            }
        }
    }
}

fn poll_loop(app: &AppHandle) {
    let mut policy = Policy::new();
    let mut deadline: Option<(Instant, u64)> = None;
    loop {
        if let Some(pending) = policy.observe(poll_once(), Instant::now(), &config_of(app)) {
            deadline = Some((pending.due, pending.generation));
        }
        if let Some((due, generation)) = deadline {
            if Instant::now() >= due {
                if let Some(active) = policy.resolve(generation) {
                    apply(app, active);
                }
                deadline = None;
            }
        }
        std::thread::sleep(POLL_INTERVAL);
    }
}

/// Without an X11 connection there is nothing to read, so the fallback reports
/// "not fullscreen" rather than shelling out to xprop on a timer.
fn poll_once() -> bool {
    false
}
