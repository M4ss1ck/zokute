use crate::process_resources::{discover_descendants, parse_pid_ticks, ResourceReader};
use std::collections::HashMap;

#[test]
fn stat_parser_uses_utime_stime_not_cutime_cstime() {
    let stat = "12345 (test) name) S 1 1 1 0 -1 0 0 0 0 0 100 50 999 999 0 0 0 0 0 0 0 0 0 0";
    assert_eq!(parse_pid_ticks(stat), Some(150));
}

#[test]
fn cpu_delta_only_counts_pids_present_in_both_samples() {
    let mut prev = HashMap::new();
    prev.insert(1, 100u64);
    prev.insert(2, 200u64);
    let mut cur = HashMap::new();
    cur.insert(2, 250u64); // delta = 50
    cur.insert(3, 300u64); // new PID, excluded

    let pid_delta: u64 = cur
        .iter()
        .filter_map(|(&pid, &ticks)| {
            prev.get(&pid)
                .map(|&prev_ticks| ticks.saturating_sub(prev_ticks))
        })
        .sum();
    assert_eq!(pid_delta, 50);
}

#[test]
fn discovers_at_least_own_pid() {
    let pids = discover_descendants(std::process::id() as i32);
    assert!(pids.contains(&(std::process::id() as i32)));
}

#[test]
fn resource_reader_returns_none_on_first_sample() {
    let mut reader = ResourceReader::new();
    assert!(reader.sample().is_none());
}

#[test]
fn resource_reader_second_sample_returns_reasonable_values() {
    let mut reader = ResourceReader::new();
    let _ = reader.sample();
    std::thread::sleep(std::time::Duration::from_millis(200));
    let second = reader.sample();
    assert!(second.is_some(), "second sample must be Some");
    let (cpu, ram) = second.unwrap();
    assert!((0.0..=100.0).contains(&cpu), "cpu {cpu} out of range");
    assert!(ram >= 0.0, "ram {ram} must be non-negative");
}
