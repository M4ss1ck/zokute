const MIN_OPACITY: f64 = 0.1;

pub fn should_reload(last_written: &str, current: &str) -> bool {
    last_written != current
}

pub fn is_hex_color(value: &str) -> bool {
    value.len() == 7 && value.starts_with('#') && value[1..].chars().all(|character| character.is_ascii_hexdigit())
}

pub fn is_valid_theme(value: &str) -> bool {
    matches!(value, "light" | "dark" | "system")
}

pub fn clamp_opacity(value: f64) -> f64 {
    if value.is_finite() { value.clamp(MIN_OPACITY, 1.0) } else { 1.0 }
}
