use crate::actions::{parse_action, Action};

// INT-003: allowed schemes are validated; unknown schemes fail safely.
#[test]
fn the_allowed_schemes_open() {
    for uri in ["https://example.com", "http://example.com", "file:///home/u/a.txt"] {
        match parse_action("open-uri", Some(uri)) {
            Some(Action::OpenUri(parsed)) => assert_eq!(parsed, uri),
            _ => panic!("{uri} should open"),
        }
    }
}

#[test]
fn an_unknown_scheme_fails_safely() {
    for uri in ["javascript://alert(1)", "ftp://example.com", "maibuk://note/1"] {
        assert!(parse_action("open-uri", Some(uri)).is_none(), "{uri} is not on the allowlist");
    }
}

#[test]
fn a_uri_without_a_scheme_is_refused() {
    assert!(parse_action("open-uri", Some("example.com")).is_none());
    assert!(parse_action("open-uri", Some("")).is_none());
}

#[test]
fn open_uri_without_a_uri_is_refused() {
    assert!(parse_action("open-uri", None).is_none());
}

#[test]
fn refresh_needs_no_argument() {
    assert!(matches!(parse_action("refresh", None), Some(Action::Refresh)));
}

#[test]
fn copy_visible_field_carries_the_field_name() {
    match parse_action("copy-visible-field", Some("cpu.load")) {
        Some(Action::CopyVisibleField(field)) => assert_eq!(field, "cpu.load"),
        _ => panic!("expected a copy action"),
    }
}

// INT-005: plugin-provided arbitrary commands are prohibited.
#[test]
fn an_unrecognized_action_is_rejected() {
    for action in ["run", "exec", "shell", "open-uri-now", ""] {
        assert!(parse_action(action, Some("https://example.com")).is_none(), "{action} must not be honoured");
    }
}
