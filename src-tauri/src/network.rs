use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize)]
pub struct NetworkReading {
    pub name: String,
    pub connected: bool,
    pub down_bytes_per_second: u64,
    pub up_bytes_per_second: u64,
}

#[derive(Debug, Clone)]
pub struct InterfaceBaseline {
    pub rx_bytes: u64,
    pub tx_bytes: u64,
    pub timestamp: std::time::Instant,
}

pub type NetworkBaselines = HashMap<String, InterfaceBaseline>;

pub fn read_and_diff(baselines: &mut NetworkBaselines, elapsed: f64) -> Vec<NetworkReading> {
    let now = std::time::Instant::now();
    let mut readings = Vec::new();
    for (name, rx, tx, connected) in crate::network_linux::read_counters() {
        let connected = connected || crate::network_linux::is_operstate_up(&name);
        let entry = baselines.entry(name.clone()).or_insert(InterfaceBaseline {
            rx_bytes: rx, tx_bytes: tx, timestamp: now,
        });
        let fresh = entry.timestamp == now;
        let down = if fresh { 0 } else { crate::baselines::delta_per_second(entry.rx_bytes, rx, elapsed) };
        let up = if fresh { 0 } else { crate::baselines::delta_per_second(entry.tx_bytes, tx, elapsed) };
        entry.rx_bytes = rx;
        entry.tx_bytes = tx;
        entry.timestamp = now;
        readings.push(NetworkReading { name, connected, down_bytes_per_second: down, up_bytes_per_second: up });
    }
    readings
}
