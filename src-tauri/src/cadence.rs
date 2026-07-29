use std::time::Instant;

/// What the collector loop should do with the tick it just woke up on.
#[derive(Debug)]
pub enum Tick {
    /// Nothing is on screen; skip the sample without disturbing delta baselines.
    Idle,
    /// First sample after a hide or a suspend gap. Delta baselines are stale and
    /// must be cleared; this tick carries no trustworthy elapsed span.
    Resume,
    /// Emit a sample. `elapsed` is the monotonic span since the last emit and is
    /// the divisor for every per-second rate.
    Emit { elapsed: f64 },
}

pub struct Cadence {
    last: Instant,
    resume: bool,
}

impl Cadence {
    pub fn new(now: Instant) -> Self {
        Cadence { last: now, resume: true }
    }

    pub fn step(&mut self, now: Instant, visible: bool, interval_ms: u64) -> Tick {
        // `last` advances on every path, including the ones that emit nothing.
        // Leaving it behind makes the next elapsed span look like a suspend and
        // strands the loop in a resume cycle that never emits again.
        let elapsed = now.saturating_duration_since(self.last).as_secs_f64();
        self.last = now;
        if !visible {
            self.resume = true;
            return Tick::Idle;
        }
        let threshold = (interval_ms as f64 * 2.0).max(5000.0) / 1000.0;
        if elapsed > threshold {
            self.resume = true;
        }
        if self.resume {
            self.resume = false;
            return Tick::Resume;
        }
        Tick::Emit { elapsed: elapsed.max(0.001) }
    }
}
