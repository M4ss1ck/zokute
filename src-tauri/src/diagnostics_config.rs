use crate::{config, config::Config, config::Profile, config_validate, paths};

pub fn validate_config(path: Option<&str>) -> Result<String, String> {
    let config_path = path.map_or_else(paths::config_path, |p| std::path::PathBuf::from(p));
    let source = std::fs::read_to_string(&config_path)
        .map_err(|e| format!("cannot read {}: {}", config_path.display(), e))?;
    let parsed = config::parse(&source)
        .map_err(|e| format!("parse error: {e}"))?;
    config_validate::check(&parsed)
        .map_err(|e| format!("validation error: {e}"))?;
    Ok(format!("{}: valid (schema v{})", config_path.display(), parsed.schema_version))
}

pub fn show_config(redact_plugin_config: bool, config: &Config, profile: &Profile) -> String {
    let mut lines = Vec::new();

    lines.push("=== Global config ===".into());
    lines.push(format!("  schema_version = {}", config.schema_version));
    lines.push(format!("  active_profile = \"{}\"", config.active_profile));
    lines.push(format!("  theme = \"{}\"", config.theme));
    lines.push(format!("  opacity = {}", config.opacity));
    lines.push(format!("  text_opacity = {}", config.text_opacity));
    lines.push(format!("  text_color = \"{}\"", config.text_color));
    lines.push(format!("  density = \"{}\"", config.density));
    lines.push(format!("  font_scale = {}", config.font_scale));
    lines.push(format!("  byte_format = \"{}\"", config.byte_format));
    lines.push(format!("  temperature_unit = \"{}\"", config.temperature_unit));

    lines.push("".into());
    lines.push("=== Profile ===".into());
    lines.push(format!("  schema_version = {}", profile.profile_schema_version));
    lines.push(format!("  sections = {}", profile.sections.len()));
    lines.push(format!("  system_fields = {}", profile.system_fields.len()));
    lines.push(format!("  show_cpu_cores = {}", profile.show_cpu_cores));
    lines.push(format!("  disks = {}", profile.disks.len()));
    lines.push(format!("  collect_interval_ms = {}", profile.collect_interval_ms));

    lines.push("".into());
    lines.push("=== Sections ===".into());
    for section in &profile.sections {
        lines.push(format!("  [{}.{}] enabled={} plugin={}",
            section.id, section.instance, section.enabled, section.is_plugin()));
        if redact_plugin_config && section.plugin_config.is_some() {
            lines.push("    plugin_config = <redacted>".into());
        }
    }

    lines.join("\n")
}

