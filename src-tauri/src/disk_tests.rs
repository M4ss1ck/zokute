use crate::disk_linux as linux;
use std::{fs, os::unix::fs::symlink, path::Path};
use tempfile::TempDir;

fn root() -> TempDir { TempDir::new().expect("tmp") }

fn write(path: &Path, value: &str) {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).unwrap(); }
    fs::write(path, value).unwrap();
}

fn link(target: &Path, path: &Path) {
    if let Some(parent) = path.parent() { fs::create_dir_all(parent).unwrap(); }
    symlink(target, path).unwrap();
}

#[test]
fn mounts_unescape_spaces() {
    let mounts = linux::parse_mounts("/dev/sda1 /mnt/My\\040Disk ext4 rw 0 0\n");
    assert_eq!(mounts[0].mount_point, "/mnt/My Disk");
}

#[test]
fn eligible_mounts_only_accept_devices() {
    for fs_type in ["overlay", "tmpfs", "proc", "sysfs", "devtmpfs", "devpts", "cgroup", "cgroup2", "squashfs"] {
        assert!(!linux::eligible(&linux::MountEntry { source: "/dev/sda1".into(), mount_point: "/".into(), fs_type: fs_type.into() }));
    }
    assert!(linux::eligible(&linux::MountEntry { source: "/dev/sda1".into(), mount_point: "/".into(), fs_type: "ext4".into() }));
    assert!(linux::eligible(&linux::MountEntry { source: "/dev/sda1".into(), mount_point: "/boot/efi".into(), fs_type: "vfat".into() }));
    assert!(linux::eligible(&linux::MountEntry { source: "/dev/sda1".into(), mount_point: "/mnt/Data".into(), fs_type: "ext4".into() }));
    assert!(!linux::eligible(&linux::MountEntry { source: "/dev/sda1".into(), mount_point: "/var/lib/docker/overlay2/x/merged".into(), fs_type: "ext4".into() }));
}

#[test]
fn uuid_lookup_falls_back_to_mount_id() {
    let root = root();
    write(&root.path().join("dev/sda1"), "");
    link(Path::new("../../sda1"), &root.path().join("dev/disk/by-uuid/1234"));
    assert_eq!(linux::device_id(root.path(), "/mnt/Data", Path::new("/dev/sda1")), "uuid:1234");
    fs::remove_file(root.path().join("dev/disk/by-uuid/1234")).unwrap();
    assert_eq!(linux::device_id(root.path(), "/mnt/Data", Path::new("/dev/sda1")), "mount:/mnt/Data");
}

#[test]
fn partition_uses_parent_block_device() {
    let root = root();
    write(&root.path().join("sys/devices/controller/nvme0n1/nvme0n1p2/partition"), "1");
    link(Path::new("../../devices/controller/nvme0n1/nvme0n1p2"), &root.path().join("sys/class/block/nvme0n1p2"));
    assert_eq!(linux::device_name(root.path(), Path::new("/dev/nvme0n1p2")), Some("nvme0n1".into()));
}

#[test]
fn matching_hwmon_controller_returns_temperature() {
    let root = root();
    write(&root.path().join("sys/devices/controller/nvme0n1"), "");
    link(Path::new("../../devices/controller/nvme0n1"), &root.path().join("sys/class/block/nvme0n1"));
    link(Path::new("../../../devices/controller/nvme0n1"), &root.path().join("sys/class/hwmon/hwmon0/device"));
    write(&root.path().join("sys/class/hwmon/hwmon0/temp1_input"), "33500");
    write(&root.path().join("sys/class/hwmon/hwmon0/temp1_crit"), "40000");
    write(&root.path().join("sys/class/hwmon/hwmon0/temp1_max"), "50000");
    assert_eq!(linux::temperature_celsius(root.path(), "nvme0n1"), Some(33.5));
}

#[test]
fn unrelated_ambiguous_malformed_and_nonfinite_sensors_are_none() {
    let unrelated = root();
    write(&unrelated.path().join("sys/devices/controller-a/nvme0n1"), "");
    write(&unrelated.path().join("sys/devices/controller-b/other"), "");
    link(Path::new("../../devices/controller-a/nvme0n1"), &unrelated.path().join("sys/class/block/nvme0n1"));
    link(Path::new("../../../devices/controller-b/other"), &unrelated.path().join("sys/class/hwmon/hwmon0/device"));
    write(&unrelated.path().join("sys/class/hwmon/hwmon0/temp1_input"), "33500");
    assert_eq!(linux::temperature_celsius(unrelated.path(), "nvme0n1"), None);

    let ambiguous = root();
    write(&ambiguous.path().join("sys/devices/controller/nvme0n1"), "");
    link(Path::new("../../devices/controller/nvme0n1"), &ambiguous.path().join("sys/class/block/nvme0n1"));
    link(Path::new("../../../devices/controller/nvme0n1"), &ambiguous.path().join("sys/class/hwmon/hwmon0/device"));
    write(&ambiguous.path().join("sys/class/hwmon/hwmon0/temp1_input"), "33500");
    link(Path::new("../../../devices/controller/nvme0n1"), &ambiguous.path().join("sys/class/hwmon/hwmon1/device"));
    write(&ambiguous.path().join("sys/class/hwmon/hwmon1/temp1_input"), "34000");
    assert_eq!(linux::temperature_celsius(ambiguous.path(), "nvme0n1"), None);

    let malformed = root();
    write(&malformed.path().join("sys/devices/controller/nvme0n1"), "");
    link(Path::new("../../devices/controller/nvme0n1"), &malformed.path().join("sys/class/block/nvme0n1"));
    link(Path::new("../../../devices/controller/nvme0n1"), &malformed.path().join("sys/class/hwmon/hwmon0/device"));
    write(&malformed.path().join("sys/class/hwmon/hwmon0/temp1_input"), "oops");
    assert_eq!(linux::temperature_celsius(malformed.path(), "nvme0n1"), None);

    let nonfinite = root();
    write(&nonfinite.path().join("sys/devices/controller/nvme0n1"), "");
    link(Path::new("../../devices/controller/nvme0n1"), &nonfinite.path().join("sys/class/block/nvme0n1"));
    link(Path::new("../../../devices/controller/nvme0n1"), &nonfinite.path().join("sys/class/hwmon/hwmon0/device"));
    write(&nonfinite.path().join("sys/class/hwmon/hwmon0/temp1_input"), "inf");
    assert_eq!(linux::temperature_celsius(nonfinite.path(), "nvme0n1"), None);
}
