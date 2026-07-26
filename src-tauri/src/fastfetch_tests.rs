use crate::fastfetch::parse;

const SAMPLE: &str = "\
massick@massick-pc
------------------
OS: Linux Mint 22.3 x86_64
Kernel: Linux 6.17.0-40-generic
Uptime: 2 hours, 21 mins
Packages: 2823 (dpkg), 26 (flatpak)
Shell: bash 5.2.21
Display (LS27DG30X): 1920x1080 in 27\", 180 Hz [External] *
Display (VG279Q3A): 1920x1080 in 27\", 180 Hz [External]
DE: Cinnamon 6.6.9
WM: Muffin (X11)
WM Theme: Mint-Y-Dark-Aqua (Mint-Y)
Theme: Mint-Y-Dark-Aqua [GTK2/3/4]
Icons: dracula-icons [GTK2/3/4]
Font: Ubuntu (10pt) [GTK2/3/4]
Cursor: Bibata-Modern-Classic (24px)
Terminal: claude
CPU: AMD Ryzen 7 8700G (16) @ 5.18 GHz
GPU 1: AMD Radeon RX 9060 XT [Discrete]
GPU 2: AMD Radeon 780M Graphics [Integrated]
Memory: 13.01 GiB / 30.50 GiB (43%)
Swap: 0 B / 2.00 GiB (0%)
Disk (/): 304.55 GiB / 937.33 GiB (32%) - ext4
Disk (/mnt/Data): 359.30 GiB / 1.79 TiB (20%) - ext4
Local IP (wlp9s0): 192.168.1.26/24
Locale: en_US.UTF-8

\u{1b}[40m   \u{1b}[41m   \u{1b}[m
";

#[test]
fn drops_the_title_separator_and_colour_blocks() {
    let ids = parse(SAMPLE).into_iter().map(|field| field.id).collect::<Vec<_>>();
    assert_eq!(
        ids,
        vec![
            "os", "kernel", "uptime", "packages", "shell", "display", "display", "de", "wm",
            "wm_theme", "theme", "icons", "font", "cursor", "terminal", "cpu", "gpu", "gpu",
            "memory", "swap", "disk", "disk", "local_ip", "locale",
        ]
    );
}

#[test]
fn keeps_the_label_verbatim_so_rows_stay_distinguishable() {
    let fields = parse(SAMPLE);
    let displays = fields.iter().filter(|field| field.id == "display").collect::<Vec<_>>();
    assert_eq!(displays.len(), 2);
    assert_eq!(displays[0].label, "Display (LS27DG30X)");
    assert_eq!(displays[1].label, "Display (VG279Q3A)");
    assert_eq!(displays[0].value, "1920x1080 in 27\", 180 Hz [External] *");
}

#[test]
fn strips_a_trailing_index_from_the_id_but_not_the_label() {
    let fields = parse(SAMPLE);
    let gpus = fields.iter().filter(|field| field.id == "gpu").collect::<Vec<_>>();
    assert_eq!(gpus.len(), 2);
    assert_eq!(gpus[0].label, "GPU 1");
    assert_eq!(gpus[1].label, "GPU 2");
}

#[test]
fn splits_on_the_first_separator_so_values_may_contain_colons() {
    let fields = parse("Local IP (wlp9s0): 192.168.1.26/24\nKernel: Linux: weird\n");
    assert_eq!(fields[0].id, "local_ip");
    assert_eq!(fields[0].value, "192.168.1.26/24");
    assert_eq!(fields[1].value, "Linux: weird");
}

#[test]
fn returns_nothing_for_output_without_any_labelled_line() {
    assert!(parse("").is_empty());
    assert!(parse("massick@massick-pc\n-----\n").is_empty());
}
