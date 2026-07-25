use crate::config::load_or_create;
use std::fs;
use tempfile::TempDir;

#[test]
fn load_or_create_errors_when_parent_is_a_file() {
    let temp = TempDir::new().unwrap();
    let parent = temp.path().join("parent");
    fs::write(&parent, "not a directory").unwrap();
    let path = parent.join("config.toml");
    let detected: Vec<String> = vec![];
    assert!(load_or_create(&path, &detected).is_err());
}
