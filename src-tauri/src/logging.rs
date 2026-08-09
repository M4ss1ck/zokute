use std::fs;
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
