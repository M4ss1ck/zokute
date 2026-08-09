use std::path::Path;

pub fn list_interfaces() -> Vec<String> {
    let mut interfaces = Vec::new();
    let Ok(entries) = std::fs::read_dir("/sys/class/net") else { return interfaces };
    for entry in entries.flatten() {
        if let Some(name) = entry.file_name().to_str() {
            interfaces.push(name.to_string());
        }
    }
    interfaces.sort();
    interfaces
}

pub fn is_operstate_up(name: &str) -> bool {
    let path = Path::new("/sys/class/net").join(name).join("operstate");
    std::fs::read_to_string(&path).ok().is_some_and(|s| s.trim() == "up")
}

pub fn read_counters() -> Vec<(String, u64, u64, bool)> {
    let mut results = Vec::new();
    for name in list_interfaces() {
        let base = Path::new("/sys/class/net").join(&name);
        let rx = std::fs::read_to_string(base.join("statistics/rx_bytes"))
            .ok().and_then(|s| s.trim().parse::<u64>().ok()).unwrap_or(0);
        let tx = std::fs::read_to_string(base.join("statistics/tx_bytes"))
            .ok().and_then(|s| s.trim().parse::<u64>().ok()).unwrap_or(0);
        let carrier = std::fs::read_to_string(base.join("carrier"))
            .ok().and_then(|s| s.trim().parse::<u8>().ok()).map(|c| c == 1).unwrap_or(false);
        results.push((name, rx, tx, carrier));
    }
    results
}
