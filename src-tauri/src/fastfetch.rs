use std::process::Command;
use crate::system_info::SystemField;

pub fn parse(output: &str) -> Vec<SystemField> {
    output
        .lines()
        .filter(|line| !line.starts_with('\u{1b}'))
        .filter_map(|line| line.split_once(": "))
        .map(|(label, value)| SystemField {
            id: id_from_label(label),
            label: label.to_string(),
            value: value.trim().to_string(),
        })
        .collect()
}

// fastfetch labels a repeated module either by subject, "Display (LS27DG30X)",
// or by ordinal, "GPU 1". Both collapse to one id so a single settings toggle
// drives the whole group, while the label keeps telling the rows apart.
fn id_from_label(label: &str) -> String {
    let base = label.split(" (").next().unwrap_or(label);
    base.trim_end_matches(|character: char| character.is_ascii_digit())
        .trim_end()
        .to_lowercase()
        .replace(' ', "_")
}

// fastfetch exits non-zero when any single module fails -- on this machine the
// Host module errors because the OEM left the DMI fields blank -- while still
// printing every module that succeeded. So the exit code is ignored and the
// parsed field count decides whether the run was useful.
pub fn collect() -> Option<Vec<SystemField>> {
    let output = Command::new("fastfetch").args(["--pipe", "--logo", "none"]).output().ok()?;
    let fields = parse(&String::from_utf8_lossy(&output.stdout));
    (!fields.is_empty()).then_some(fields)
}
