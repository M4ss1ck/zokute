use crate::plugin_protocol::{parse_response, PROTOCOL_VERSION};

#[test]
fn the_supported_protocol_is_version_one() {
    assert_eq!(PROTOCOL_VERSION, 1);
}

#[test]
fn a_minimal_response_needs_only_a_title() {
    let response = parse_response(br#"{"title":"Notes"}"#).expect("minimal response");
    assert_eq!(response.title, "Notes");
    assert!(response.summary.is_none());
    assert!(response.rows.is_empty());
}

#[test]
fn rows_carry_columns_progress_status_and_actions() {
    let response = parse_response(
        br#"{"title":"Notes","summary":"3 open","rows":[
            {"id":"n1","columns":["Draft","2d"],"progress":0.5,"status":"warn",
             "actions":[{"action":"open-uri","label":"Open","uri":"https://example.com"}]}]}"#,
    ).expect("full response");
    assert_eq!(response.summary.as_deref(), Some("3 open"));
    let row = &response.rows[0];
    assert_eq!(row.id, "n1");
    assert_eq!(row.columns, vec!["Draft", "2d"]);
    assert_eq!(row.progress, Some(0.5));
    assert_eq!(row.status.as_deref(), Some("warn"));
    assert_eq!(row.actions[0].action, "open-uri");
    assert_eq!(row.actions[0].label, "Open");
    assert_eq!(row.actions[0].uri.as_deref(), Some("https://example.com"));
}

#[test]
fn optional_row_fields_may_be_omitted() {
    let response = parse_response(br#"{"title":"T","rows":[{"id":"a","columns":[]}]}"#).expect("sparse row");
    let row = &response.rows[0];
    assert!(row.progress.is_none());
    assert!(row.status.is_none());
    assert!(row.actions.is_empty());
}

#[test]
fn a_response_without_a_title_is_rejected() {
    assert!(parse_response(br#"{"rows":[]}"#).is_err());
}

#[test]
fn malformed_json_is_reported_rather_than_panicking() {
    let error = parse_response(b"not json at all").expect_err("should not parse");
    assert!(error.starts_with("JSON:"));
}

#[test]
fn empty_output_is_reported_as_a_parse_error() {
    assert!(parse_response(b"").is_err());
}
