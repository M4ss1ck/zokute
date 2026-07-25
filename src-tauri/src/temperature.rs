use serde::Serialize;
use sysinfo::Components;

#[derive(Clone, Serialize)]
pub struct Temperature {
    pub label: String,
    pub celsius: f32,
}

pub fn select(components: &Components) -> Option<Temperature> {
    pick(components, &["package", "tctl", "tdie", "cpu"])
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
