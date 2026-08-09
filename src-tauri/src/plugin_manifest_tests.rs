use crate::plugin_manifest::{discover_manifests, PluginManifest};
use std::path::{Path, PathBuf};

    use tempfile::TempDir;

    fn write_manifest(dir: &Path, content: &str) -> PathBuf {
        let path = dir.join("plugin.toml");
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn valid_manifest_parses() {
        let dir = TempDir::new().unwrap();
        let id = dir.path().file_name().unwrap().to_str().unwrap().to_string();
        let content = format!(r#"id = "{id}"
name = "Test Plugin"
version = "1.0.0"
protocol = 1
command = ["python3", "main.py"]
"#);
        let path = write_manifest(dir.path(), &content);
        let manifest = PluginManifest::load(dir.path(), &path).unwrap();
        assert_eq!(manifest.name, "Test Plugin");
        assert_eq!(manifest.version, "1.0.0");
        assert!(manifest.cwd.is_none());
        assert!(manifest.config.is_empty());
        assert_eq!(manifest.interval, 30);
    }

    #[test]
    fn id_mismatch_is_rejected() {
        let dir = TempDir::new().unwrap();
        let plugin_dir = dir.path().join("right-id");
        std::fs::create_dir(&plugin_dir).unwrap();
        let content = r#"id = "wrong-id"
name = "Test"
version = "1.0.0"
protocol = 1
command = ["python3"]
"#;
        let path = write_manifest(&plugin_dir, content);
        assert!(PluginManifest::load(&plugin_dir, &path).is_err());
    }

    #[test]
    fn unsupported_protocol_is_rejected() {
        let dir = TempDir::new().unwrap();
        let id = dir.path().file_name().unwrap().to_str().unwrap().to_string();
        let content = format!(r#"id = "{id}"
name = "Test"
version = "1.0.0"
protocol = 99
command = ["python3"]
"#);
        let path = write_manifest(dir.path(), &content);
        assert!(PluginManifest::load(dir.path(), &path).is_err());
    }

    #[test]
    fn empty_command_is_rejected() {
        let dir = TempDir::new().unwrap();
        let id = dir.path().file_name().unwrap().to_str().unwrap().to_string();
        let content = format!(r#"id = "{id}"
name = "Test"
version = "1.0.0"
protocol = 1
command = []
"#);
        let path = write_manifest(dir.path(), &content);
        assert!(PluginManifest::load(dir.path(), &path).is_err());
    }

    #[test]
    fn interval_below_minimum_is_rejected() {
        let dir = TempDir::new().unwrap();
        let id = dir.path().file_name().unwrap().to_str().unwrap().to_string();
        let content = format!(r#"id = "{id}"
name = "Test"
version = "1.0.0"
protocol = 1
command = ["python3"]
interval = 1
"#);
        let path = write_manifest(dir.path(), &content);
        assert!(PluginManifest::load(dir.path(), &path).is_err());
    }

    #[test]
    fn discover_finds_valid_manifests() {
        let plugins_dir = TempDir::new().unwrap();
        let plugin_dir = plugins_dir.path().join("test-plugin");
        std::fs::create_dir(&plugin_dir).unwrap();
        let content = r#"id = "test-plugin"
name = "Test"
version = "1.0.0"
protocol = 1
command = ["python3"]
"#;
        write_manifest(&plugin_dir, content);
        let results = discover_manifests(plugins_dir.path());
        assert!(!results.is_empty());
        assert!(results[0].1.is_ok());
    }
