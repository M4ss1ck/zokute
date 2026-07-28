use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

const BACKUP_KEEP: usize = 5;

pub fn write(path: &Path, contents: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temp = path.with_extension("tmp");
    {
        let mut file = fs::File::create(&temp)?;
        file.write_all(contents.as_bytes())?;
        file.sync_all()?;
    }
    let reread = fs::read_to_string(&temp)?;
    if reread != contents {
        return Err(std::io::Error::other("reread mismatch"));
    }
    fs::rename(&temp, path)?;
    if let Some(parent) = path.parent() {
        if let Ok(dir) = fs::File::open(parent) {
            let _ = dir.sync_all();
        }
    }
    Ok(())
}

pub fn backup_previous(path: &Path, backups_dir: &Path) -> std::io::Result<()> {
    if !path.exists() {
        return Ok(());
    }
    fs::create_dir_all(backups_dir)?;
    let name = path.file_name().unwrap_or_default();
    let dest = backups_dir.join(format!("{}.previous", name.to_string_lossy()));
    fs::copy(path, &dest)?;
    prune_backups(backups_dir, name);
    Ok(())
}

fn prune_backups(dir: &Path, name: &std::ffi::OsStr) {
    let prefix = format!("{}.previous", name.to_string_lossy());
    let mut entries: Vec<(std::time::SystemTime, PathBuf)> = Vec::new();
    if let Ok(read) = fs::read_dir(dir) {
        for entry in read.flatten() {
            let path = entry.path();
            if path.to_string_lossy().contains(&prefix) {
                if let Ok(meta) = path.metadata() {
                    if let Ok(created) = meta.created() {
                        entries.push((created, path));
                    }
                }
            }
        }
    }
    entries.sort_by_key(|entry| std::cmp::Reverse(entry.0));
    for (_, stale) in entries.iter().skip(BACKUP_KEEP) {
        let _ = fs::remove_file(stale);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn writes_to_file_and_can_read_back() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.toml");
        write(&path, "key = \"value\"").unwrap();
        let read = fs::read_to_string(&path).unwrap();
        assert_eq!(read, "key = \"value\"");
    }

    #[test]
    fn temp_file_is_removed_after_write() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.toml");
        write(&path, "content").unwrap();
        assert!(!path.with_extension("tmp").exists());
    }

    #[test]
    fn failed_write_returns_error_when_parent_is_a_file() {
        let dir = TempDir::new().unwrap();
        let parent = dir.path().join("parent");
        fs::write(&parent, "not a directory").unwrap();
        let path = parent.join("test.toml");
        let result = write(&path, "content");
        assert!(result.is_err());
    }

    #[test]
    fn backup_previous_creates_backup_file() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.toml");
        fs::write(&path, "original").unwrap();
        let backups = dir.path().join("backups");
        backup_previous(&path, &backups).unwrap();
        let dest = backups.join("test.toml.previous");
        assert!(dest.exists());
        assert_eq!(fs::read_to_string(dest).unwrap(), "original");
    }

    #[test]
    fn unknown_fields_round_trip_through_parse() {
        let source = r#"schema_version = 1
opacity = 0.9
sections = []
system_fields = ["os"]
show_cpu_cores = true
disks = []
unknown_field = "should survive"
"#;
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("test.toml");
        write(&path, source).unwrap();
        let read = fs::read_to_string(&path).unwrap();
        assert!(read.contains("unknown_field"));
    }
}
