use crate::audio_bands::{band_edges, center_hz, normalize_db, BANDS, MAX_HZ, MIN_HZ};

const SAMPLE_RATE: f32 = 48_000.0;
const FFT_SIZE: usize = 2048;

// M9 Task 5.1: one shared logarithmic 20-22000 Hz spectrum.
#[test]
fn the_band_scale_spans_the_full_capture_range() {
    assert_eq!(MIN_HZ, 20.0);
    assert_eq!(MAX_HZ, 22000.0);
    assert!((center_hz(0) - MIN_HZ).abs() < 0.01);
    assert!((center_hz(BANDS - 1) - MAX_HZ).abs() < 1.0);
}

#[test]
fn band_centres_are_evenly_spaced_on_a_log_scale() {
    let step = center_hz(1) / center_hz(0);
    for band in 1..BANDS {
        let ratio = center_hz(band) / center_hz(band - 1);
        assert!((ratio - step).abs() < 0.001, "band {band} broke the geometric spacing");
    }
}

#[test]
fn the_frontend_can_invert_a_centre_back_to_its_band() {
    // Mirrors binForFreq in visualizer-frame.ts.
    for band in [0, 17, 48, 95] {
        let t = (center_hz(band).log2() - MIN_HZ.log2()) / (MAX_HZ.log2() - MIN_HZ.log2());
        let recovered = (t * (BANDS - 1) as f32).round() as usize;
        assert_eq!(recovered, band);
    }
}

#[test]
fn band_edges_never_go_backwards() {
    let edges = band_edges(SAMPLE_RATE, FFT_SIZE);
    for index in 1..edges.len() {
        assert!(edges[index] > edges[index - 1], "edge {index} did not advance");
    }
}

#[test]
fn band_edges_stay_inside_the_usable_spectrum() {
    let edges = band_edges(SAMPLE_RATE, FFT_SIZE);
    assert!(edges[0] >= 1, "the DC bin carries no audible content");
    assert!(*edges.last().unwrap() <= FFT_SIZE / 2, "edges must stop at Nyquist");
}

#[test]
fn silence_normalizes_to_zero_and_full_scale_to_one() {
    assert_eq!(normalize_db(0.0), 0.0);
    assert_eq!(normalize_db(1.0), 1.0);
    let quiet = normalize_db(0.001);
    assert!(quiet > 0.0 && quiet < 1.0, "a quiet signal should land between the rails");
}
