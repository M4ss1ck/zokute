use crate::cli::{parse_args, CliCommand};

fn args(rest: &[&str]) -> Vec<String> {
    std::iter::once("zokute".to_string()).chain(rest.iter().map(|a| a.to_string())).collect()
}

#[test]
fn the_lifecycle_verbs_parse() {
    assert!(matches!(parse_args(&args(&["show"])), Some(CliCommand::Show)));
    assert!(matches!(parse_args(&args(&["hide"])), Some(CliCommand::Hide)));
    assert!(matches!(parse_args(&args(&["toggle"])), Some(CliCommand::Toggle)));
    assert!(matches!(parse_args(&args(&["edit"])), Some(CliCommand::Edit)));
    assert!(matches!(parse_args(&args(&["reload"])), Some(CliCommand::Reload)));
    assert!(matches!(parse_args(&args(&["status"])), Some(CliCommand::Status)));
}

#[test]
fn no_command_at_all_parses_to_nothing() {
    assert!(parse_args(&args(&[])).is_none());
}

#[test]
fn an_unknown_command_parses_to_nothing() {
    assert!(parse_args(&args(&["frobnicate"])).is_none());
}

// M10 Task 3.1: config validate [path]
#[test]
fn config_validate_takes_an_optional_path() {
    match parse_args(&args(&["config", "validate"])) {
        Some(CliCommand::ConfigValidate(path)) => assert!(path.is_none()),
        _ => panic!("expected config validate"),
    }
    match parse_args(&args(&["config", "validate", "/tmp/z.toml"])) {
        Some(CliCommand::ConfigValidate(path)) => assert_eq!(path.as_deref(), Some("/tmp/z.toml")),
        _ => panic!("expected config validate with a path"),
    }
}

// M10 Task 3.2: redaction is explicit.
#[test]
fn config_show_redacts_only_when_asked() {
    match parse_args(&args(&["config", "show"])) {
        Some(CliCommand::ConfigShow { redact_plugin_config }) => assert!(!redact_plugin_config),
        _ => panic!("expected config show"),
    }
    match parse_args(&args(&["config", "show", "--redact-plugin-config"])) {
        Some(CliCommand::ConfigShow { redact_plugin_config }) => assert!(redact_plugin_config),
        _ => panic!("expected redacted config show"),
    }
}

#[test]
fn an_unknown_config_subcommand_parses_to_nothing() {
    assert!(parse_args(&args(&["config"])).is_none());
    assert!(parse_args(&args(&["config", "edit"])).is_none());
}

// M10 Task 3.3: diagnostics [--output path]
#[test]
fn diagnostics_takes_an_optional_output_path() {
    match parse_args(&args(&["diagnostics"])) {
        Some(CliCommand::Diagnostics { output }) => assert!(output.is_none()),
        _ => panic!("expected diagnostics"),
    }
    match parse_args(&args(&["diagnostics", "--output", "/tmp/d.json"])) {
        Some(CliCommand::Diagnostics { output }) => assert_eq!(output.as_deref(), Some("/tmp/d.json")),
        _ => panic!("expected diagnostics with an output path"),
    }
}
