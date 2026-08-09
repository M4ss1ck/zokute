pub mod flags;
pub mod lifecycle;
pub mod position;
#[cfg(test)] mod lifecycle_tests;

pub const LABELS: [&str; 9] = ["system", "cpu", "memory", "disk", "network", "spectrum", "ring", "clock", "date"];
const CONTROL_LABELS: [&str; 1] = ["settings"];

pub fn is_control_window(label: &str) -> bool {
    CONTROL_LABELS.contains(&label) || crate::guides_overlay::is_overlay(label)
}

pub use lifecycle::{reconcile, show_all, hide_all, toggle_visibility};
