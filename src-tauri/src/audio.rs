use crate::{
    audio_spectrum::{is_silent, Analyzer, FFT_SIZE},
    config::Profile,
    window::LABELS,
};
use libpulse_binding::{def::BufferAttr, error::PAErr, sample::{Format, Spec}, stream::Direction};
use libpulse_simple_binding::Simple;
use std::{
    mem::size_of,
    sync::{atomic::{AtomicBool, Ordering}, Arc},
    thread,
    time::{Duration, Instant},
};
use tauri::{AppHandle, Emitter, EventTarget, Manager};

const HOP: usize = 1024;
const CHANNELS: usize = 2;
const DEVICE: &str = "@DEFAULT_MONITOR@";
const SAMPLE_RATE: u32 = 48_000;
const SILENCE_RESET: Duration = Duration::from_secs(10);

pub struct Enabled(pub Arc<AtomicBool>);

pub fn is_visualizer(id: &str) -> bool {
    LABELS[5..].contains(&id)
}

pub fn sync(app: &AppHandle, profile: &Profile) {
    let Some(state) = app.try_state::<Enabled>() else { return };
    state.0.store(
        profile.sections.iter().any(|section| section.enabled && is_visualizer(&section.id)),
        Ordering::Relaxed,
    );
}

fn connect() -> Result<Simple, PAErr> {
    let spec = Spec { format: Format::F32le, channels: CHANNELS as u8, rate: SAMPLE_RATE };
    let attr = record_buffer_attr();
    Simple::new(None, "Zokute", Direction::Record, Some(DEVICE), "visualizer", &spec, None, Some(&attr))
}

pub fn record_buffer_attr() -> BufferAttr {
    BufferAttr {
        maxlength: u32::MAX,
        tlength: u32::MAX,
        prebuf: u32::MAX,
        minreq: u32::MAX,
        fragsize: (HOP * CHANNELS * size_of::<f32>()) as u32,
    }
}

fn downmix(raw: &[u8]) -> [f32; HOP] {
    let mut mono = [0.0f32; HOP];
    for (index, slot) in mono.iter_mut().enumerate() {
        let base = index * CHANNELS * 4;
        let left = f32::from_le_bytes(raw[base..base + 4].try_into().expect("left"));
        let right = f32::from_le_bytes(raw[base + 4..base + 8].try_into().expect("right"));
        *slot = (left + right) * 0.5;
    }
    mono
}

fn emit_frame(app: &AppHandle, frame: &crate::audio_spectrum::Frame) {
    let _ = app.emit_filter("audio", frame, |target| match target {
        EventTarget::WebviewWindow { label } => label.split_once('-').map_or(is_visualizer(label), |(base, _)| is_visualizer(base)),
        _ => false,
    });
}

pub fn start(app: AppHandle, initial: &Profile) {
    let enabled = Arc::new(AtomicBool::new(false));
    app.manage(Enabled(enabled.clone()));
    sync(&app, initial);
    thread::spawn(move || {
        let mut analyzer = Analyzer::new();
        let mut window = [0.0f32; FFT_SIZE];
        let mut raw = vec![0u8; HOP * CHANNELS * 4];
        let mut stream: Option<Simple> = None;
        let mut silent_since = None::<Instant>;
        loop {
            if !enabled.load(Ordering::Relaxed) {
                stream = None;
                silent_since = None;
                thread::sleep(Duration::from_millis(500));
                continue;
            }
            if stream.is_none() {
                match connect() {
                    Ok(opened) => stream = Some(opened),
                    Err(_) => {
                        thread::sleep(Duration::from_millis(500));
                        continue;
                    }
                }
            }
            let Some(active) = stream.as_ref() else { continue };
            if active.read(&mut raw).is_err() {
                stream = None;
                thread::sleep(Duration::from_millis(100));
                continue;
            }
            let mono = downmix(&raw);
            if is_silent(&mono) {
                let start = silent_since.get_or_insert_with(Instant::now);
                if start.elapsed() >= SILENCE_RESET {
                    stream = None;
                    silent_since = None;
                    window.fill(0.0);
                }
                continue;
            }
            silent_since = None;
            window.copy_within(HOP.., 0);
            window[FFT_SIZE - HOP..].copy_from_slice(&mono);
            let frame = analyzer.analyze(&window);
            emit_frame(&app, &frame);
        }
    });
}
