/// The shared capture publishes one normalized spectrum on this fixed
/// logarithmic scale. Per-instance visualizer parameters select a slice of it,
/// so the range must cover everything AUDIO-003 lets a widget ask for.
pub const BANDS: usize = 96;
pub const MIN_HZ: f32 = 20.0;
pub const MAX_HZ: f32 = 22000.0;

const FLOOR_DB: f32 = -70.0;

/// Centre frequency of `band`, geometrically spaced across the capture range.
/// `visualizer-frame.ts` inverts exactly this to find the band for a frequency.
#[cfg(test)]
pub fn center_hz(band: usize) -> f32 {
    MIN_HZ * (MAX_HZ / MIN_HZ).powf(band as f32 / (BANDS - 1) as f32)
}

/// FFT bin boundaries for each band. Bands narrower than the bin resolution
/// collapse onto successive bins so the edges stay strictly increasing.
pub fn band_edges(sample_rate: f32, fft_size: usize) -> [usize; BANDS + 1] {
    let bin_hz = sample_rate / fft_size as f32;
    let nyquist_bin = fft_size / 2;
    let half_step = 0.5 / (BANDS - 1) as f32;
    let mut edges = [0usize; BANDS + 1];
    let mut previous = 0usize;
    for (index, edge) in edges.iter_mut().enumerate() {
        let t = index as f32 / (BANDS - 1) as f32 - half_step;
        let hz = MIN_HZ * (MAX_HZ / MIN_HZ).powf(t);
        let floor = if index == 0 { 1 } else { previous + 1 };
        *edge = ((hz / bin_hz).round() as usize).max(floor).min(nyquist_bin);
        previous = *edge;
    }
    edges
}

pub fn normalize_db(magnitude: f32) -> f32 {
    if magnitude <= 0.0 {
        return 0.0;
    }
    let decibels = 20.0 * magnitude.log10();
    ((decibels - FLOOR_DB) / -FLOOR_DB).clamp(0.0, 1.0)
}
