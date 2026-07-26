use crate::disk;
use serde::Serialize;
use std::fs;
use sysinfo::{Disks, System};

#[derive(Clone, Debug, Serialize)]
pub struct SystemField {
    pub id: String,
    pub label: String,
    pub value: String,
}

// Used when fastfetch is not installed: only what sysinfo and DMI can
// answer honestly. Everything richer -- shell version, DE version, GTK theme,
// fonts, GPU -- needs fastfetch's probes and is simply absent here.
pub fn fallback() -> Vec<SystemField> {
    let mut fields = Vec::new();
    push(&mut fields, "os", "OS", format_os(System::name().as_deref(), System::os_version().as_deref()));
    let sys_vendor = read_dmi("/sys/class/dmi/id/sys_vendor");
    let board_vendor = read_dmi("/sys/class/dmi/id/board_vendor");
    let product_name = read_dmi("/sys/class/dmi/id/product_name");
    push(&mut fields, "host", "Host", host_from_dmi_candidates(product_name.as_deref(), &[sys_vendor.as_deref(), board_vendor.as_deref()]));
    push(&mut fields, "kernel", "Kernel", System::kernel_version());
    let mut system = System::new();
    system.refresh_memory();
    push(&mut fields, "memory", "Memory", Some(format_usage(system.used_memory(), system.total_memory())));
    push(&mut fields, "swap", "Swap", Some(format_usage(system.used_swap(), system.total_swap())));
    for reading in disk::discover(&Disks::new_with_refreshed_list()) {
        fields.push(SystemField {
            id: "disk".into(),
            label: format!("Disk ({})", reading.mount),
            value: format_usage(reading.used_bytes, reading.total_bytes),
        });
    }
    push(&mut fields, "locale", "Locale", std::env::var("LANG").ok().and_then(|value| clean(Some(&value))));
    fields
}

// fastfetch's stock preset (and any user's own config) can emit ids the app
// has never heard of, e.g. battery or power_adapter. Without this they parse
// fine but never reach the settings list -- so unknown ids are appended after
// the known catalog, in first-appearance order, instead of being dropped.
pub fn catalog_order<T: AsRef<str>>(fields: &[SystemField], known: &[T]) -> Vec<String> {
    let mut order: Vec<String> = known.iter().map(|id| id.as_ref().to_string()).collect();
    for field in fields {
        if !order.contains(&field.id) {
            order.push(field.id.clone());
        }
    }
    order
}

pub fn filter_and_order<T: AsRef<str>>(fields: &[SystemField], order: &[T], uptime: u64) -> Vec<SystemField> {
    order
        .iter()
        .flat_map(|id| {
            let id = id.as_ref();
            if id == "uptime" {
                return vec![SystemField { id: id.into(), label: "Uptime".into(), value: format_uptime(uptime) }];
            }
            fields.iter().filter(|field| field.id == id).cloned().collect()
        })
        .collect()
}

pub fn format_uptime(seconds: u64) -> String {
    let parts = [(seconds / 86400, "day"), (seconds % 86400 / 3600, "hour"), (seconds % 3600 / 60, "min")];
    let text = parts
        .iter()
        .filter(|(count, _)| *count > 0)
        .map(|(count, unit)| if *count == 1 { format!("{count} {unit}") } else { format!("{count} {unit}s") })
        .collect::<Vec<_>>()
        .join(", ");
    if text.is_empty() { "0 mins".to_string() } else { text }
}

pub fn format_usage(used: u64, total: u64) -> String {
    // Rounded floating-point percentage: integer division would floor 42.65% to 42%,
    // but fastfetch rounds -- 13.01 GiB / 30.50 GiB reports as 43%, not 42%.
    let percent = if total == 0 { 0 } else { (used as f64 / total as f64 * 100.0).round() as u64 };
    format!("{} / {} ({percent}%)", format_bytes(used), format_bytes(total))
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KiB", "MiB", "GiB", "TiB"];
    let mut value = bytes as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 { format!("{bytes} B") } else { format!("{value:.2} {}", UNITS[unit]) }
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
    host_from_dmi_candidates(product_name, &[vendor])
}

pub fn host_from_dmi_candidates(product_name: Option<&str>, vendors: &[Option<&str>]) -> Option<String> {
    clean(product_name)
        .filter(|value| !is_generic_dmi(value))
        .or_else(|| vendors.iter().copied().flatten().find_map(|value| clean(Some(value)).filter(|value| !is_generic_dmi(value))))
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
