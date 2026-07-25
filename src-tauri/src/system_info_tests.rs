use crate::system_info::{
    collect_environment_fields, count_debian_packages, filter_and_order, format_os, host_from_dmi,
    SystemField,
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
    let ordered = filter_and_order(&fields, &["kernel".into(), "display".into(), "host".into(), "uptime".into()], 42);
    assert_eq!(ordered.iter().map(|field| field.id.as_str()).collect::<Vec<_>>(), vec!["kernel", "host", "uptime"]);
    assert_eq!(ordered.iter().map(|field| field.value.as_str()).collect::<Vec<_>>(), vec!["6.1", "zokute", "42"]);
}
