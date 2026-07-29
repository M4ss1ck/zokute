use rustfft::{num_complex::Complex, Fft, FftPlanner};
use serde::Serialize;
use std::sync::Arc;

use crate::audio_bands::{band_edges, normalize_db, BANDS};

pub const FFT_SIZE: usize = 2048;

const SAMPLE_RATE: f32 = 48_000.0;
const SILENCE_PEAK: f32 = 0.0005;
const RELEASE: f32 = 0.82;

#[derive(Clone, Serialize)]
pub struct Frame {
    pub bands: Vec<u8>,
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
        Self { fft: FftPlanner::new().plan_fft_forward(FFT_SIZE), edges: band_edges(SAMPLE_RATE, FFT_SIZE), window, smoothed: [0.0; BANDS], scratch: vec![Complex { re: 0.0, im: 0.0 }; FFT_SIZE] }
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
