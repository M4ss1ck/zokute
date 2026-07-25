use crate::disk_linux;
use serde::Serialize;
use std::path::Path;
use sysinfo::Disks;

#[derive(Clone, Debug, Serialize)]
pub struct DiskReading {
    pub id: String,
    pub name: String,
    pub mount: String,
    pub used_bytes: u64,
    pub total_bytes: u64,
    pub temperature_celsius: Option<f32>,
}

pub fn discover(disks: &Disks) -> Vec<DiskReading> {
    discover_with_root(Path::new("/"), disks)
}

pub(crate) fn discover_with_root(root: &Path, disks: &Disks) -> Vec<DiskReading> {
    let mounts = disk_linux::read_mounts(root);
    let mut readings = disks
        .list()
        .iter()
        .filter_map(|disk| disk_linux::reading(root, disk, &mounts))
        .collect::<Vec<_>>();
    readings.sort_by(|a, b| a.mount.cmp(&b.mount));
    readings
}
