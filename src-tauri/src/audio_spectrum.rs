use rustfft::{num_complex::Complex, Fft, FftPlanner};
use serde::Serialize;
use std::sync::Arc;

pub const FFT_SIZE: usize = 2048;
pub const BANDS: usize = 96;

const SAMPLE_RATE: f32 = 48_000.0;
const MIN_HZ: f32 = 30.0;
const MAX_HZ: f32 = 16_000.0;
const FLOOR_DB: f32 = -70.0;
const SILENCE_PEAK: f32 = 0.0005;
const RELEASE: f32 = 0.82;

#[derive(Clone, Serialize)]
pub struct Frame {
    pub bands: Vec<u8>,
}

pub fn band_edges() -> [usize; BANDS + 1] {
    let bin_hz = SAMPLE_RATE / FFT_SIZE as f32;
    let ratio = MAX_HZ / MIN_HZ;
    let mut edges = [0usize; BANDS + 1];
    let mut previous = 0usize;
    for (index, edge) in edges.iter_mut().enumerate() {
        let hz = MIN_HZ * ratio.powf(index as f32 / BANDS as f32);
        let floor = if index == 0 { 1 } else { previous + 1 };
        *edge = ((hz / bin_hz).round() as usize).max(floor);
        previous = *edge;
    }
    edges
}

pub fn normalize_db(magnitude: f32) -> f32 {
    if magnitude <= 0.0 { return 0.0; }
    let decibels = 20.0 * magnitude.log10();
    ((decibels - FLOOR_DB) / -FLOOR_DB).clamp(0.0, 1.0)
}

pub fn is_silent(samples: &[f32]) -> bool {
    !samples.iter().any(|sample| sample.abs() > SILENCE_PEAK)
}

pub struct Analyzer {
    fft: Arc<dyn Fft<f32>>,
    edges: [usize; BANDS + 1],
    window: [f32; FFT_SIZE],
    smoothed: [f32; BANDS],
    scratch: Vec<Complex<f32>>,
}

impl Analyzer {
    pub fn new() -> Self {
        let mut window = [0.0f32; FFT_SIZE];
        for (index, value) in window.iter_mut().enumerate() {
            *value = 0.5 - 0.5 * (std::f32::consts::TAU * index as f32 / FFT_SIZE as f32).cos();
        }
        Self { fft: FftPlanner::new().plan_fft_forward(FFT_SIZE), edges: band_edges(), window, smoothed: [0.0; BANDS], scratch: vec![Complex { re: 0.0, im: 0.0 }; FFT_SIZE] }
    }

    pub fn analyze(&mut self, samples: &[f32; FFT_SIZE]) -> Frame {
        for (index, slot) in self.scratch.iter_mut().enumerate() {
            *slot = Complex { re: samples[index] * self.window[index], im: 0.0 };
        }
        self.fft.process(&mut self.scratch);
        let full_scale = FFT_SIZE as f32 / 4.0;
        let bands = (0..BANDS).map(|band| {
            let magnitude = self.scratch[self.edges[band]..self.edges[band + 1]].iter().map(|bin| bin.norm()).fold(0.0f32, f32::max);
            let level = normalize_db(magnitude / full_scale);
            let previous = self.smoothed[band];
            self.smoothed[band] = if level > previous { level } else { previous * RELEASE + level * (1.0 - RELEASE) };
            (self.smoothed[band] * 255.0) as u8
        }).collect();
        Frame { bands }
    }
}
