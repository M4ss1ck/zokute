use crate::system_info::{
    count_debian_packages, filter_and_order, format_os, host_from_dmi, first_present,
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
    assert_eq!(count_debian_packages(status), Some(2));
}

#[test]
fn resolves_env_values_in_order() {
    assert_eq!(
        first_present(&[None, Some("zsh"), Some("bash")]),
        Some("zsh".to_string())
    );
    assert_eq!(
        first_present(&[None, None, Some("Cinnamon")]),
        Some("Cinnamon".to_string())
    );
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
