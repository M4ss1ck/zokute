/// Per-second rate between two readings of a monotonic counter.
///
/// A counter that went backwards means the interface was reset or the device
/// reappeared, so the span is not measurable: report nothing and let the next
/// sample re-baseline, rather than turning the wrap into a fictional burst.
pub fn delta_per_second(previous: u64, current: u64, elapsed: f64) -> u64 {
    if current < previous {
        return 0;
    }
    ((current - previous) as f64 / elapsed.max(0.001)) as u64
}
