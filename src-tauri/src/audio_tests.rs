use crate::audio_bands::{band_edges, BANDS};
use crate::audio_spectrum::{is_silent, Analyzer, FFT_SIZE};
use crate::audio::{is_visualizer, record_buffer_attr};

const SAMPLE_RATE: f32 = 48_000.0;

fn sine(hz: f32) -> [f32; FFT_SIZE] {
    let mut samples = [0.0f32; FFT_SIZE];
    for (index, sample) in samples.iter_mut().enumerate() {
        *sample = (std::f32::consts::TAU * hz * index as f32 / SAMPLE_RATE).sin();
    }
    samples
}

fn band_containing(hz: f32) -> usize {
    let bin = (hz / (SAMPLE_RATE / FFT_SIZE as f32)).round() as usize;
    let edges = band_edges(SAMPLE_RATE, FFT_SIZE);
    (0..BANDS).find(|band| edges[*band] <= bin && bin < edges[band + 1]).expect("band for hz")
}

#[test]
fn a_tone_lights_its_own_band_and_leaves_distant_bands_dark() {
    let mut analyzer = Analyzer::new();
    let frame = analyzer.analyze(&sine(1_000.0));
    let lit = band_containing(1_000.0);
    let dark = band_containing(250.0);
    assert!(frame.bands[lit] > 200, "1 kHz band was {}", frame.bands[lit]);
    assert!(frame.bands[dark] < 20, "250 Hz band was {}", frame.bands[dark]);
}

#[test]
fn silence_is_detected_only_on_actual_silence() {
    assert!(is_silent(&[0.0f32; 64]));
    assert!(!is_silent(&sine(1_000.0)));
}

#[test]
fn only_visualizer_ids_are_visualizers() {
    assert!(is_visualizer("spectrum"));
    assert!(is_visualizer("ring"));
    assert!(!is_visualizer("cpu"));
    assert!(!is_visualizer(""));
}

#[test]
fn record_buffer_attr_uses_the_one_hop_latency() {
    let attr = record_buffer_attr();
    assert_eq!(attr.maxlength, u32::MAX);
    assert_eq!(attr.fragsize, 8192);
}
