pub mod flags;
pub mod lifecycle;
pub mod position;

pub const LABELS: [&str; 5] = ["system", "cpu", "memory", "disk", "network"];

pub use lifecycle::{reconcile, toggle_visibility};
