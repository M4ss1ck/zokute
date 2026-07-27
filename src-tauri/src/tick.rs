use std::time::{Duration, SystemTime, UNIX_EPOCH};

// The emitter's interval runs on the monotonic clock, so its phase against the
// wall clock is whatever it happened to be at startup: a clock widget reading
// these timestamps could show a second up to a second late, and jitter around
// the boundary could repeat or skip one. Firing just inside each second keeps
// every tick's second the true one.
pub const OFFSET_MS: u64 = 50;

pub fn until_next_second(now_ms: u64) -> Duration {
    Duration::from_millis(1000 - now_ms % 1000 + OFFSET_MS)
}

pub fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_millis() as u64)
        .unwrap_or(0)
}
