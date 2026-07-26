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
