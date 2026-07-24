use serde::Serialize;
use sysinfo::Components;

#[derive(Clone, Serialize)]
pub struct Temperature {
    pub label: String,
    pub celsius: f32,
}

pub fn select(components: &Components) -> (Option<Temperature>, Vec<Temperature>) {
    (
        pick(components, &["package", "tctl", "tdie", "cpu"]),
        pick_all(components),
    )
}

fn pick(components: &Components, needles: &[&str]) -> Option<Temperature> {
    components.iter().find_map(|component| {
        let label = component.label().to_lowercase();
        needles
            .iter()
            .any(|needle| label.contains(needle))
            .then(|| {
                component.temperature().filter(|celsius| celsius.is_finite()).map(|celsius| Temperature { label: component.label().to_string(), celsius })
            })
            .flatten()
    })
}

fn pick_all(components: &Components) -> Vec<Temperature> {
    components
        .iter()
        .filter_map(|component| {
            let label = component.label().to_lowercase();
            (label.contains("gpu") || label.contains("amdgpu") || label.contains("nvidia"))
                .then(|| {
                    component.temperature().filter(|celsius| celsius.is_finite()).map(|celsius| Temperature { label: component.label().to_string(), celsius })
                })
                .flatten()
        })
        .collect()
}
