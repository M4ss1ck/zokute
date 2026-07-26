pub mod flags;
pub mod lifecycle;
pub mod position;

pub const LABELS: [&str; 7] = ["system", "cpu", "memory", "disk", "network", "spectrum", "ring"];

pub use lifecycle::{reconcile, toggle_visibility};
