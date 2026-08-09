use serde::Deserialize;

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Deserialize)]
pub struct V1Response {
    pub title: String,
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub rows: Vec<V1Row>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct V1Row {
    pub id: String,
    pub columns: Vec<String>,
    #[serde(default)]
    pub progress: Option<f64>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub actions: Vec<V1Action>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct V1Action {
    pub action: String,
    pub label: String,
    #[serde(default)]
    pub uri: Option<String>,
}

pub fn parse_response(data: &[u8]) -> Result<V1Response, String> {
    serde_json::from_slice(data).map_err(|e| format!("JSON: {e}"))
}
