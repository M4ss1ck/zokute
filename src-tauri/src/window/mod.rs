pub mod flags;
pub mod lifecycle;
pub mod position;

pub const LABELS: [&str; 9] = ["system", "cpu", "memory", "disk", "network", "spectrum", "ring", "clock", "date"];

pub use lifecycle::{reconcile, toggle_visibility};
