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

/// Does the active profile want anything on screen?
///
/// Derived from the profile rather than from a window's mapped state: polling
/// `is_visible()` straight after `show()` races GTK and reports false, which
/// gated collection off for the life of the process and left every widget blank.
pub fn demand(profile: &crate::config::Profile, hidden: bool) -> bool {
    !hidden && profile.sections.iter().any(|section| section.enabled && owns_window(section))
}

pub fn owns_window(section: &crate::config::SectionConfig) -> bool {
    crate::window::LABELS.contains(&section.id.as_str()) || section.is_panel() || section.is_plugin()
}
