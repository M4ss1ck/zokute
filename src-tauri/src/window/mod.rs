pub mod flags;
pub mod lifecycle;
pub mod position;

pub const LABELS: [&str; 8] = ["system", "cpu", "memory", "disk", "network", "spectrum", "ring", "clock"];

pub use lifecycle::{reconcile, toggle_visibility};
