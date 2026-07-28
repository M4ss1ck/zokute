/// Validate and normalize collector interval
pub fn normalize_interval(ms: u64) -> u64 {
    ms.clamp(250, 60000)
}
