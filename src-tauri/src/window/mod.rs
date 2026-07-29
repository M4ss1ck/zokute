pub mod flags;
pub mod lifecycle;
pub mod position;
#[cfg(test)] mod lifecycle_tests;

pub const LABELS: [&str; 9] = ["system", "cpu", "memory", "disk", "network", "spectrum", "ring", "clock", "date"];
const CONTROL_LABELS: [&str; 2] = ["settings", "layout-editor"];

pub fn is_control_window(label: &str) -> bool {
    CONTROL_LABELS.contains(&label)
}

pub use lifecycle::{reconcile, show_all, hide_all, toggle_visibility};
