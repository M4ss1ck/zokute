use std::collections::HashMap;
use std::fs;
use std::io;

fn read_proc_stat_total() -> io::Result<u64> {
    let content = fs::read_to_string("/proc/stat")?;
    let line = content
        .lines()
        .next()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "empty /proc/stat"))?;
    let fields: Vec<&str> = line.split_whitespace().collect();
    if fields.len() < 9 {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "short cpu line"));
    }
    let sum: u64 = fields[1..9]
        .iter()
        .filter_map(|s| s.parse::<u64>().ok())
        .sum();
    Ok(sum)
}

pub(crate) fn parse_pid_ticks(content: &str) -> Option<u64> {
    let close = content.rfind(')')?;
    let fields: Vec<&str> = content[close + 2..].split_whitespace().collect();
    let utime = fields.get(11)?.parse::<u64>().ok()?;
    let stime = fields.get(12)?.parse::<u64>().ok()?;
    Some(utime + stime)
}

fn read_pid_ticks(pid: i32) -> io::Result<u64> {
    let content = fs::read_to_string(format!("/proc/{}/stat", pid))?;
    // Exited-child totals would spike the delta while live descendants are
    // already sampled separately.
    parse_pid_ticks(&content)
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidData, "invalid process stat"))
}

fn read_pid_pss(pid: i32) -> io::Result<u64> {
    let content = fs::read_to_string(format!("/proc/{}/smaps_rollup", pid))?;
    for line in content.lines() {
        if let Some(value) = line.strip_prefix("Pss:") {
            return Ok(value
                .split_whitespace()
                .next()
                .and_then(|s| s.parse::<u64>().ok())
                .unwrap_or(0));
        }
    }
    Ok(0)
}

pub(crate) fn discover_descendants(root_pid: i32) -> Vec<i32> {
    let mut pids = Vec::new();
    let mut queue = vec![root_pid];
    let mut head = 0;
    while head < queue.len() {
        let pid = queue[head];
        head += 1;
        pids.push(pid);
        if let Ok(content) = fs::read_to_string(format!("/proc/{}/task/{}/children", pid, pid)) {
            for child_str in content.split_whitespace() {
                if let Ok(child_pid) = child_str.parse::<i32>() {
                    queue.push(child_pid);
                }
            }
        }
    }
    pids
}

fn read_all_pid_ticks(pids: &[i32]) -> HashMap<i32, u64> {
    pids.iter()
        .filter_map(|&pid| read_pid_ticks(pid).ok().map(|t| (pid, t)))
        .collect()
}

fn read_total_pss(pids: &[i32]) -> u64 {
    pids.iter().filter_map(|&pid| read_pid_pss(pid).ok()).sum()
}

struct CpuBaseline {
    pid_ticks: HashMap<i32, u64>,
    total_ticks: u64,
}

pub(crate) struct ResourceReader {
    baseline: Option<CpuBaseline>,
    root_pid: i32,
}

impl ResourceReader {
    pub(crate) fn new() -> Self {
        ResourceReader {
            baseline: None,
            root_pid: std::process::id() as i32,
        }
    }

    pub(crate) fn sample(&mut self) -> Option<(f64, f64)> {
        let pids = discover_descendants(self.root_pid);
        let total_ticks = read_proc_stat_total().ok()?;
        let pid_ticks = read_all_pid_ticks(&pids);
        let pss_kb = read_total_pss(&pids);
        let ram_mib = pss_kb as f64 / 1024.0;

        let result = match &self.baseline {
            Some(prev) => {
                let total_delta = total_ticks.saturating_sub(prev.total_ticks) as f64;
                if total_delta == 0.0 {
                    Some((0.0, ram_mib))
                } else {
                    let pid_delta: u64 = pid_ticks
                        .iter()
                        .filter_map(|(&pid, &ticks)| {
                            prev.pid_ticks
                                .get(&pid)
                                .map(|&prev_ticks| ticks.saturating_sub(prev_ticks))
                        })
                        .sum();
                    let cpu_pct = ((pid_delta as f64 / total_delta) * 100.0).min(100.0);
                    Some((cpu_pct, ram_mib))
                }
            }
            None => None,
        };

        self.baseline = Some(CpuBaseline {
            pid_ticks,
            total_ticks,
        });
        result
    }
}
