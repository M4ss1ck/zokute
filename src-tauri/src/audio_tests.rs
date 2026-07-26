use crate::audio_spectrum::{band_edges, is_silent, normalize_db, Analyzer, BANDS, FFT_SIZE};
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
    let edges = band_edges();
    (0..BANDS).find(|band| edges[*band] <= bin && bin < edges[band + 1]).expect("band for hz")
}

#[test]
fn band_edges_strictly_increase_and_stay_inside_the_spectrum() {
    let edges = band_edges();
    assert!(edges[0] >= 1, "the DC bin carries no signal and must be excluded");
    for window in edges.windows(2) {
        assert!(window[1] > window[0], "every band needs at least one bin: {window:?}");
    }
    assert!(edges[BANDS] < FFT_SIZE / 2, "top edge must stay below Nyquist");
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
fn normalize_db_maps_full_scale_to_one_and_the_floor_to_zero() {
    assert_eq!(normalize_db(0.0), 0.0);
    assert!((normalize_db(1.0) - 1.0).abs() < 0.001);
    assert_eq!(normalize_db(0.0001), 0.0);
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
