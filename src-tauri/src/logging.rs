use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;

static LOG_FILE: Mutex<Option<PathBuf>> = Mutex::new(None);

pub fn log_dir() -> PathBuf {
    crate::paths::state_dir().join("logs")
}

pub fn log_path() -> PathBuf {
    log_dir().join("zokute.log")
}

pub fn init() {
    let dir = log_dir();
    let _ = fs::create_dir_all(&dir);
    let path = log_path();
    if let Ok(mut guard) = LOG_FILE.lock() {
        *guard = Some(path.clone());
    }
    let _ = fs::write(&path, "");
}

pub fn log(level: &str, message: &str) {
    if let Ok(guard) = LOG_FILE.lock() {
        if let Some(ref path) = *guard {
            let ts = chrono_now();
            let line = format!("[{ts}] [{level}] {message}\n");
            let _ = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)
                .and_then(|mut f| f.write_all(line.as_bytes()));
        }
    }
}

pub fn warn(message: &str) { log("WARN", message); }
pub fn error(message: &str) { log("ERROR", message); }
pub fn debug(message: &str) { log("DEBUG", message); }

/// Parse a log level string, returning it only if it's a recognized value.
pub fn validated_level(level: &str) -> Option<String> {
    match level {
        "error" | "warn" | "info" | "debug" | "trace" => Some(level.to_string()),
        _ => None,
    }
}

fn chrono_now() -> String {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();
    let (hours, rem) = (secs / 3600 % 24, secs % 3600);
    let (minutes, seconds) = (rem / 60, rem % 60);
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}

/// Read recent log entries as a bounded string.
pub fn recent_logs(max_lines: usize) -> String {
    let path = LOG_FILE.lock()
        .ok()
        .and_then(|g| g.clone())
        .unwrap_or_else(log_path);
    let content = fs::read_to_string(&path).unwrap_or_default();
    let lines: Vec<&str> = content.lines().collect();
    let start = lines.len().saturating_sub(max_lines);
    lines[start..].join("\n")
}
