use crate::system_info::{
    collect_environment_fields, count_debian_packages, filter_and_order, format_os, format_uptime,
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
fn counts_only_installed_debian_packages() {
    let status = "\
Package: one\nStatus: install ok installed\n\n\
Package: two\nStatus: deinstall ok config-files\n\n\
Package: three\nStatus: install ok installed\n";
    assert_eq!(count_debian_packages(status), 2);
}

#[test]
fn keeps_zero_installed_debian_packages() {
    let status = "\
Package: one\nStatus: deinstall ok config-files\n";
    assert_eq!(count_debian_packages(status), 0);
}

#[test]
fn resolves_environment_fields_in_configured_order() {
    let fields = collect_environment_fields(
        &[
            ("COMSPEC", "cmd.exe"),
            ("SHELL", "bash"),
            ("DESKTOP_SESSION", "xfce"),
            ("XDG_CURRENT_DESKTOP", "Cinnamon"),
            ("TERM", "xterm"),
            ("TERMINAL", "WezTerm"),
            ("TERM_PROGRAM", "WezTerm.app"),
            ("LANG", "en_US.UTF-8"),
            ("LC_MESSAGES", "fr_FR.UTF-8"),
            ("LC_ALL", "de_DE.UTF-8"),
            ("GTK_THEME", "Adwaita"),
        ],
        None,
    );
    assert_eq!(fields.iter().map(|field| field.id.as_str()).collect::<Vec<_>>(), vec!["shell", "desktop", "window_manager", "theme", "terminal", "locale"]);
    assert_eq!(fields.iter().map(|field| field.value.as_str()).collect::<Vec<_>>(), vec!["bash", "Cinnamon", "Cinnamon", "Adwaita", "WezTerm.app", "de_DE.UTF-8"]);
}

#[test]
fn falls_through_blank_environment_values() {
    let fields = collect_environment_fields(
        &[
            ("SHELL", " "),
            ("COMSPEC", "cmd.exe"),
            ("XDG_CURRENT_DESKTOP", "\t"),
            ("DESKTOP_SESSION", "Cinnamon"),
            ("TERM_PROGRAM", ""),
            ("TERMINAL", "WezTerm"),
            ("LC_ALL", " "),
            ("LC_MESSAGES", ""),
            ("LANG", "en_US.UTF-8"),
        ],
        None,
    );
    assert_eq!(fields.iter().map(|field| field.id.as_str()).collect::<Vec<_>>(), vec!["shell", "desktop", "window_manager", "terminal", "locale"]);
    assert_eq!(fields.iter().map(|field| field.value.as_str()).collect::<Vec<_>>(), vec!["cmd.exe", "Cinnamon", "Cinnamon", "WezTerm", "en_US.UTF-8"]);
}

#[test]
fn omits_missing_environment_fields() {
    assert!(collect_environment_fields(&[], None).is_empty());
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
