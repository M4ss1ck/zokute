use serde::Serialize;
use std::{env, fs};
use sysinfo::System;

#[derive(Clone, Debug, Serialize)]
pub struct SystemField {
    pub id: String,
    pub label: String,
    pub value: String,
}

pub fn collect_static(display: Option<String>) -> Vec<SystemField> {
    let mut fields = Vec::new();
    push(&mut fields, "os", "OS", format_os(System::name().as_deref(), System::os_version().as_deref()));
    let sys_vendor = read_dmi("/sys/class/dmi/id/sys_vendor");
    let board_vendor = read_dmi("/sys/class/dmi/id/board_vendor");
    let product_name = read_dmi("/sys/class/dmi/id/product_name");
    let vendor = first_present(&[sys_vendor.as_deref(), board_vendor.as_deref()]);
    push(&mut fields, "host", "Host", host_from_dmi(product_name.as_deref(), vendor.as_deref()));
    push(&mut fields, "kernel", "Kernel", System::kernel_version());
    push(&mut fields, "packages", "Packages", count_debian_packages(&fs::read_to_string("/var/lib/dpkg/status").unwrap_or_default()).map(|count| count.to_string()));
    push(&mut fields, "shell", "Shell", env_value(&["SHELL", "COMSPEC"]));
    let desktop = env_value(&["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION"]);
    push(&mut fields, "desktop", "Desktop", desktop.clone());
    if desktop.as_deref().is_some_and(is_combined_desktop) {
        push(&mut fields, "window_manager", "Window Manager", desktop);
    }
    push(&mut fields, "theme", "Theme", env_value(&["GTK_THEME", "XDG_THEME_NAME"]));
    push(&mut fields, "terminal", "Terminal", env_value(&["TERM_PROGRAM", "TERMINAL", "TERM"]));
    push(&mut fields, "locale", "Locale", env_value(&["LC_ALL", "LC_MESSAGES", "LANG"]));
    push(&mut fields, "display", "Display", display);
    fields
}

pub fn filter_and_order(fields: &[SystemField], order: &[String], uptime: u64) -> Vec<SystemField> {
    order
        .iter()
        .filter_map(|id| {
            if id == "uptime" {
                return Some(SystemField { id: id.clone(), label: "Uptime".into(), value: uptime.to_string() });
            }
            fields.iter().find(|field| field.id == *id).cloned()
        })
        .collect()
}

pub fn format_os(name: Option<&str>, version: Option<&str>) -> Option<String> {
    match (clean(name), clean(version)) {
        (Some(name), Some(version)) => Some(format!("{name} {version}")),
        (Some(name), None) => Some(name),
        (None, Some(version)) => Some(version),
        (None, None) => None,
    }
}

pub fn host_from_dmi(product_name: Option<&str>, vendor: Option<&str>) -> Option<String> {
    clean(product_name)
        .filter(|value| !is_generic_dmi(value))
        .or_else(|| clean(vendor).filter(|value| !is_generic_dmi(value)))
}

pub fn count_debian_packages(status: &str) -> Option<u64> {
    let count = status.split("\n\n").filter(|package| package.lines().any(|line| line == "Status: install ok installed")).count();
    (count > 0).then_some(count as u64)
}

pub fn first_present(values: &[Option<&str>]) -> Option<String> {
    values.iter().copied().flatten().find_map(|value| clean(Some(value)))
}

fn env_value(keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| env::var(key).ok().and_then(|value| clean(Some(&value))))
}

fn read_dmi(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().and_then(|value| clean(Some(&value)))
}

fn push(fields: &mut Vec<SystemField>, id: &str, label: &str, value: Option<String>) {
    if let Some(value) = value {
        fields.push(SystemField { id: id.into(), label: label.into(), value });
    }
}

fn clean(value: Option<&str>) -> Option<String> {
    value.map(str::trim).filter(|value| !value.is_empty()).map(ToOwned::to_owned)
}

fn is_generic_dmi(value: &str) -> bool {
    matches!(value, "System Product Name" | "System Version" | "To be filled by O.E.M." | "Default string")
}

fn is_combined_desktop(value: &str) -> bool {
    matches!(value, "Cinnamon")
}
