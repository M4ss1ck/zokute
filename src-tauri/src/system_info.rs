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
    if let Ok(status) = fs::read_to_string("/var/lib/dpkg/status") {
        push(&mut fields, "packages", "Packages", Some(count_debian_packages(&status).to_string()));
    }
    let env = env::vars().collect::<Vec<_>>();
    fields.extend(collect_environment_fields_owned(&env, display));
    fields
}

pub fn collect_environment_fields(env: &[(&str, &str)], display: Option<String>) -> Vec<SystemField> {
    collect_environment_fields_with(|keys| resolve_env_value(env, keys), display)
}

fn collect_environment_fields_owned(env: &[(String, String)], display: Option<String>) -> Vec<SystemField> {
    collect_environment_fields_with(|keys| resolve_env_value_owned(env, keys), display)
}

fn collect_environment_fields_with<F>(mut resolve: F, display: Option<String>) -> Vec<SystemField>
where
    F: FnMut(&[&str]) -> Option<String>,
{
    let mut fields = Vec::new();
    let shell = resolve(&["SHELL", "COMSPEC"]);
    let desktop = resolve(&["XDG_CURRENT_DESKTOP", "DESKTOP_SESSION"]);
    push(&mut fields, "shell", "Shell", shell);
    push(&mut fields, "desktop", "Desktop", desktop.clone());
    if desktop.as_deref().is_some_and(is_combined_desktop) {
        push(&mut fields, "window_manager", "Window Manager", desktop);
    }
    push(&mut fields, "theme", "Theme", resolve(&["GTK_THEME", "XDG_THEME_NAME"]));
    push(&mut fields, "terminal", "Terminal", resolve(&["TERM_PROGRAM", "TERMINAL", "TERM"]));
    push(&mut fields, "locale", "Locale", resolve(&["LC_ALL", "LC_MESSAGES", "LANG"]));
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

pub fn count_debian_packages(status: &str) -> u64 {
    status.split("\n\n").filter(|package| package.lines().any(|line| line == "Status: install ok installed")).count() as u64
}

pub fn first_present(values: &[Option<&str>]) -> Option<String> {
    values.iter().copied().flatten().find_map(|value| clean(Some(value)))
}

fn resolve_env_value(env: &[(&str, &str)], keys: &[&str]) -> Option<String> {
    resolve_env_value_with(keys, |key| env.iter().find(|(candidate, _)| *candidate == key).map(|(_, value)| *value))
}

fn resolve_env_value_owned(env: &[(String, String)], keys: &[&str]) -> Option<String> {
    resolve_env_value_with(keys, |key| env.iter().find(|(candidate, _)| candidate.as_str() == key).map(|(_, value)| value.as_str()))
}

fn resolve_env_value_with<'a, F>(keys: &[&str], mut lookup: F) -> Option<String>
where
    F: FnMut(&str) -> Option<&'a str>,
{
    keys.iter().find_map(|key| {
        lookup(key).and_then(|value| {
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_string())
        })
    })
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
