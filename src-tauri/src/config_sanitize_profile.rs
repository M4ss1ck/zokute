use crate::config::{self, Profile};
use super::helpers::is_hex_color;
use crate::window;

pub fn sanitize_profile(mut profile: Profile) -> Profile {
    profile = config::normalize_profile_instances(profile);
    profile.sections.retain(|section| {
        window::LABELS.contains(&section.id.as_str()) || section.id == "plugin" || section.id == "panel"
    });
    for section in &mut profile.sections {
        if !section.scale.is_finite() || section.scale <= 0.0 { section.scale = 1.0; }
        if section.color_a.as_deref().is_some_and(|c| !is_hex_color(c)) { section.color_a = None; }
        if section.color_b.as_deref().is_some_and(|c| !is_hex_color(c)) { section.color_b = None; }
        if section.clock_color.as_deref().is_some_and(|c| !is_hex_color(c)) { section.clock_color = None; }
        if section.date_color.as_deref().is_some_and(|c| !is_hex_color(c)) { section.date_color = None; }
        if section.accent_color.as_deref().is_some_and(|c| !is_hex_color(c)) { section.accent_color = None; }
        if let Some(o) = section.opacity_override { section.opacity_override = Some(o.clamp(0.1, 1.0)); }
        if let Some(r) = section.radius_override { section.radius_override = Some(r.clamp(2, 24)); }
        if let Some(p) = section.padding_override { section.padding_override = Some(p.clamp(0, 32)); }
        if let Some(f) = section.font_scale { section.font_scale = Some(f.clamp(0.5, 2.0)); }
        if section.chart_colors.as_ref().is_some_and(|colors| colors.iter().any(|c| !is_hex_color(c))) { section.chart_colors = None; }
    }
    let mut seen_fields: Vec<String> = Vec::new();
    profile.system_fields.retain(|field| {
        if seen_fields.iter().any(|s| s == field) { return false; }
        seen_fields.push(field.clone());
        true
    });
    let mut seen_disks: Vec<String> = Vec::new();
    profile.disks.retain(|disk| {
        if seen_disks.iter().any(|s| s == &disk.id) { return false; }
        seen_disks.push(disk.id.clone());
        true
    });
    profile
}
