use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Manager};

pub struct FullscreenState(pub Arc<AtomicBool>);

impl FullscreenState {
    pub fn new() -> Self {
        FullscreenState(Arc::new(AtomicBool::new(false)))
    }

    pub fn is_fullscreen(&self) -> bool {
        self.0.load(Ordering::Relaxed)
    }

    pub fn set_fullscreen(&self, fullscreen: bool) {
        self.0.store(fullscreen, Ordering::Relaxed);
    }
}

pub fn start(app: AppHandle) {
    let state = FullscreenState::new();
    app.manage(state);

    #[cfg(target_os = "linux")]
    {
        std::thread::spawn(move || {
            let mut consecutive_exit = 0u32;
            loop {
                let fullscreen = check_xprop();
                if let Some(s) = app.try_state::<FullscreenState>() {
                    // Exit hysteresis: require 3 consecutive exit readings
                    // before clearing, to avoid flicker on transient state changes
                    if fullscreen {
                        consecutive_exit = 0;
                        s.set_fullscreen(true);
                    } else {
                        consecutive_exit += 1;
                        if consecutive_exit >= 3 {
                            s.set_fullscreen(false);
                        }
                    }
                }
                std::thread::sleep(Duration::from_millis(250));
            }
        });
    }

    #[cfg(not(target_os = "linux"))]
    {
        let _ = app;
    }
}

#[cfg(target_os = "linux")]
fn check_xprop() -> bool {
    let window_id = Command::new("xprop")
        .args(["-root", "_NET_ACTIVE_WINDOW"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| {
            // Output: "_NET_ACTIVE_WINDOW(WINDOW): window id # 0x3a00004, ..."
            s.split_whitespace()
                .find(|p| p.starts_with("0x"))
                .map(|s| s.trim_end_matches(',').to_string())
        });

    match window_id {
        Some(id) => Command::new("xprop")
            .args(["-id", &id, "_NET_WM_STATE"])
            .output()
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.contains("_NET_WM_STATE_FULLSCREEN"))
            .unwrap_or(false),
        None => false,
    }
}
