use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

#[derive(Clone, Default)]
pub struct VisibilityState(pub Arc<AtomicBool>);

impl VisibilityState {
    pub fn set_visible(&self, visible: bool) {
        self.0.store(visible, Ordering::Relaxed);
    }

    pub fn is_visible(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }
}

/// Demand-driven: returns true if any widget should be running collection
pub fn has_demand(state: &VisibilityState) -> bool {
    state.is_visible()
}
