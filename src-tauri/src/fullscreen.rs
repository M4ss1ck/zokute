use std::time::{Duration, Instant};

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct FullscreenConfig {
    pub behavior: String,
    pub dim_opacity: f64,
    pub enter_delay_ms: u64,
    pub exit_delay_ms: u64,
}

impl Default for FullscreenConfig {
    fn default() -> Self {
        FullscreenConfig { behavior: "show".into(), dim_opacity: 0.25, enter_delay_ms: 150, exit_delay_ms: 250 }
    }
}

/// What the widgets should do given the settled fullscreen state.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Effect {
    None,
    Hide,
    Dim(f64),
}

pub fn effect(active: bool, edit_mode: bool, config: &FullscreenConfig) -> Effect {
    // You cannot arrange widgets you cannot see, so edit mode outranks the
    // policy for both hiding and dimming.
    if !active || edit_mode {
        return Effect::None;
    }
    match config.behavior.as_str() {
        "hide" => Effect::Hide,
        "dim" => Effect::Dim(config.dim_opacity),
        _ => Effect::None,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Pending {
    pub target: bool,
    pub due: Instant,
    pub generation: u64,
}

/// Debounces raw EWMH fullscreen observations. Every scheduled decision carries
/// a generation; a reversal bumps it so the delayed decision lands stale and is
/// dropped rather than flickering the widgets.
pub struct Policy {
    active: bool,
    pending: Option<Pending>,
    generation: u64,
}

impl Policy {
    pub fn new() -> Self {
        Policy { active: false, pending: None, generation: 0 }
    }

    #[cfg(test)]
    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn observe(&mut self, raw: bool, now: Instant, config: &FullscreenConfig) -> Option<Pending> {
        if raw == self.active {
            // Back to the settled state: drop any decision still in flight.
            if self.pending.is_some() {
                self.generation += 1;
                self.pending = None;
            }
            return None;
        }
        if self.pending.map(|pending| pending.target) == Some(raw) {
            return None;
        }
        let delay = if raw { config.enter_delay_ms } else { config.exit_delay_ms };
        self.generation += 1;
        let pending = Pending { target: raw, due: now + Duration::from_millis(delay), generation: self.generation };
        self.pending = Some(pending);
        Some(pending)
    }

    pub fn resolve(&mut self, generation: u64) -> Option<bool> {
        let pending = self.pending?;
        if pending.generation != generation {
            return None;
        }
        self.pending = None;
        self.active = pending.target;
        Some(pending.target)
    }
}
