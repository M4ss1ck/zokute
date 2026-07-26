use crate::system_info::{
    catalog_order, filter_and_order, format_bytes, format_os, format_uptime, format_usage,
    host_from_dmi, host_from_dmi_candidates, SystemField,
};

#[test]
fn formats_os_name_and_version() {
    assert_eq!(
        format_os(Some("Debian GNU/Linux"), Some("12 (bookworm)")),
        Some("Debian GNU/Linux 12 (bookworm)".to_string())
    );
    assert_eq!(format_os(Some("Fedora"), None), Some("Fedora".to_string()));
}

#[test]
fn rejects_generic_dmi_host_names_before_vendor_fallback() {
    assert_eq!(
        host_from_dmi(Some("System Product Name"), Some("Lenovo")),
        Some("Lenovo".to_string())
    );
}

#[test]
fn skips_generic_sys_vendor_and_uses_board_vendor() {
    assert_eq!(
        host_from_dmi_candidates(Some("System Product Name"), &[Some("To be filled by O.E.M."), Some("Lenovo")]),
        Some("Lenovo".to_string())
    );
}

#[test]
fn omits_missing_values_and_keeps_configured_order() {
    let fields = vec![
        SystemField { id: "host".into(), label: "Host".into(), value: "zokute".into() },
        SystemField { id: "kernel".into(), label: "Kernel".into(), value: "6.1".into() },
    ];
    let order = vec!["kernel".to_string(), "display".to_string(), "host".to_string(), "uptime".to_string()];
    let ordered = filter_and_order(&fields, &order, 42);
    assert_eq!(ordered.iter().map(|field| field.id.as_str()).collect::<Vec<_>>(), vec!["kernel", "host", "uptime"]);
    assert_eq!(ordered.iter().map(|field| field.value.as_str()).collect::<Vec<_>>(), vec!["6.1", "zokute", "0 mins"]);
}

#[test]
fn supports_a_static_catalog_independent_of_the_selection() {
    let fields = vec![SystemField { id: "host".into(), label: "Host".into(), value: "zokute".into() }];
    let ordered = filter_and_order(&fields, &["host", "uptime"], 42);
    assert_eq!(ordered.iter().map(|field| field.id.as_str()).collect::<Vec<_>>(), vec!["host", "uptime"]);
}

#[test]
fn emits_every_row_sharing_a_requested_id() {
    let fields = vec![
        SystemField { id: "display".into(), label: "Display (A)".into(), value: "1920x1080".into() },
        SystemField { id: "display".into(), label: "Display (B)".into(), value: "2560x1440".into() },
        SystemField { id: "locale".into(), label: "Locale".into(), value: "en_US.UTF-8".into() },
    ];
    let ordered = filter_and_order(&fields, &["locale", "display"], 0);
    assert_eq!(
        ordered.iter().map(|field| field.label.as_str()).collect::<Vec<_>>(),
        vec!["Locale", "Display (A)", "Display (B)"]
    );
}

#[test]
fn appends_unknown_fastfetch_ids_after_the_known_catalog_in_first_appearance_order() {
    let fields = vec![
        SystemField { id: "battery".into(), label: "Battery".into(), value: "80%".into() },
        SystemField { id: "host".into(), label: "Host".into(), value: "zokute".into() },
        SystemField { id: "battery".into(), label: "Battery 2".into(), value: "90%".into() },
    ];
    let order = catalog_order(&fields, &["host", "kernel"]);
    assert_eq!(order, vec!["host", "kernel", "battery"]);
}

#[test]
fn formats_uptime_the_way_fastfetch_phrases_it() {
    assert_eq!(format_uptime(0), "0 mins");
    assert_eq!(format_uptime(59), "0 mins");
    assert_eq!(format_uptime(60), "1 min");
    assert_eq!(format_uptime(8460), "2 hours, 21 mins");
    assert_eq!(format_uptime(3600), "1 hour");
    assert_eq!(format_uptime(97500), "1 day, 3 hours, 5 mins");
    assert_eq!(format_uptime(172800), "2 days");
}

#[test]
fn injects_the_formatted_uptime_regardless_of_the_catalog() {
    let ordered = filter_and_order(&[], &["uptime"], 8460);
    assert_eq!(ordered.len(), 1);
    assert_eq!(ordered[0].label, "Uptime");
    assert_eq!(ordered[0].value, "2 hours, 21 mins");
}

#[test]
fn formats_byte_counts_the_way_fastfetch_does() {
    assert_eq!(format_bytes(0), "0 B");
    assert_eq!(format_bytes(512), "512 B");
    assert_eq!(format_bytes(2 * 1024 * 1024 * 1024), "2.00 GiB");
    assert_eq!(format_bytes(13_968_836_198), "13.01 GiB");
}

#[test]
fn formats_usage_as_used_over_total_with_a_percentage() {
    assert_eq!(format_usage(0, 2 * 1024 * 1024 * 1024), "0 B / 2.00 GiB (0%)");
    assert_eq!(
        format_usage(13_968_836_198, 32_749_355_008),
        "13.01 GiB / 30.50 GiB (43%)"
    );
}

#[test]
fn reports_zero_percent_rather_than_dividing_by_a_missing_total() {
    assert_eq!(format_usage(0, 0), "0 B / 0 B (0%)");
}
