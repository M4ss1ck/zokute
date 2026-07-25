use crate::disk::DiskReading;
use std::{fs, path::Path};
use sysinfo::Disk;

pub(crate) struct MountEntry {
    pub source: String,
    pub mount_point: String,
    pub fs_type: String,
}

pub(crate) fn read_mounts(root: &Path) -> Vec<MountEntry> {
    fs::read_to_string(root.join("proc/mounts")).ok().map(|text| parse_mounts(&text)).unwrap_or_default()
}

pub(crate) fn parse_mounts(text: &str) -> Vec<MountEntry> {
    text.lines().filter_map(|line| {
        let mut parts = line.split_whitespace();
        Some(MountEntry { source: unescape(parts.next()?), mount_point: unescape(parts.next()?), fs_type: parts.next()?.to_string() })
    }).collect()
}

pub(crate) fn reading(root: &Path, disk: &Disk, mounts: &[MountEntry]) -> Option<DiskReading> {
    let mount = disk.mount_point().to_string_lossy().into_owned();
    let entry = mounts.iter().find(|entry| entry.mount_point == mount)?;
    if !eligible(entry) { return None; }
    let source = Path::new(&entry.source);
    let name = device_name(root, source).unwrap_or_else(|| source.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default());
    Some(DiskReading { id: device_id(root, &entry.mount_point, source), name: name.clone(), mount, used_bytes: disk.total_space().saturating_sub(disk.available_space()), total_bytes: disk.total_space(), temperature_celsius: temperature_celsius(root, &name) })
}

pub(crate) fn eligible(entry: &MountEntry) -> bool {
    entry.source.starts_with("/dev/")
        && !matches!(entry.fs_type.as_str(), "overlay" | "tmpfs" | "proc" | "sysfs" | "devtmpfs" | "devpts" | "cgroup" | "cgroup2" | "squashfs" | "rootfs")
        && !is_docker_mount(&entry.mount_point)
}

pub(crate) fn device_id(root: &Path, mount: &str, source: &Path) -> String {
    uuid_for(root, source).map(|uuid| format!("uuid:{uuid}")).unwrap_or_else(|| format!("mount:{mount}"))
}

fn uuid_for(root: &Path, source: &Path) -> Option<String> {
    let source = fs::canonicalize(root.join(source.strip_prefix("/").ok()?)).ok()?;
    fs::read_dir(root.join("dev/disk/by-uuid")).ok()?.filter_map(Result::ok).find_map(|entry| {
        fs::canonicalize(entry.path()).ok().and_then(|target| (target == source).then(|| entry.file_name().to_string_lossy().into_owned()))
    })
}

pub(crate) fn device_name(root: &Path, source: &Path) -> Option<String> {
    let name = source.file_name()?.to_string_lossy().into_owned();
    let block = root.join("sys/class/block").join(&name);
    if fs::canonicalize(&block).ok()?.join("partition").exists() {
        fs::canonicalize(block).ok()?.parent().and_then(|path| path.file_name()).map(|path| path.to_string_lossy().into_owned())
    } else {
        Some(name)
    }
}

pub(crate) fn temperature_celsius(root: &Path, block_name: &str) -> Option<f32> {
    let controller = fs::canonicalize(root.join("sys/class/block").join(block_name)).ok()?;
    let mut matches = Vec::new();
    for hwmon in fs::read_dir(root.join("sys/class/hwmon")).ok()? {
        let hwmon = hwmon.ok()?.path();
        let device = fs::canonicalize(hwmon.join("device")).ok()?;
        if !shares_ancestry(&controller, &device) { continue; }
        for input in fs::read_dir(&hwmon).ok()? {
            let input = input.ok()?.path();
            if !is_temp_input(&input) { continue; }
            if let Some(value) = fs::read_to_string(&input).ok().and_then(|value| value.trim().parse::<f32>().ok()).filter(|value| value.is_finite()) {
                matches.push(value / 1000.0);
            }
        }
    }
    (matches.len() == 1).then(|| matches[0])
}

fn shares_ancestry(child: &Path, ancestor: &Path) -> bool {
    child == ancestor || child.strip_prefix(ancestor).is_ok()
}

fn is_docker_mount(mount_point: &str) -> bool {
    let path = Path::new(mount_point);
    path.starts_with("/docker") || path.starts_with("/var/lib/docker")
}

fn is_temp_input(path: &Path) -> bool {
    let name = path.file_name().and_then(|name| name.to_str()).unwrap_or("");
    let Some(core) = name.strip_suffix("_input") else { return false; };
    let Some(digits) = core.strip_prefix("temp") else { return false; };
    !digits.is_empty() && digits.chars().all(|ch| ch.is_ascii_digit())
}

fn unescape(value: &str) -> String {
    let mut output = String::with_capacity(value.len());
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] != b'\\' { output.push(bytes[i] as char); i += 1; continue; }
        if i + 3 < bytes.len() {
            match &bytes[i + 1..i + 4] {
                b"040" => { output.push(' '); i += 4; continue; }
                b"011" => { output.push('\t'); i += 4; continue; }
                b"012" => { output.push('\n'); i += 4; continue; }
                b"134" => { output.push('\\'); i += 4; continue; }
                _ => {}
            }
        }
        output.push('\\');
        i += 1;
    }
    output
}
