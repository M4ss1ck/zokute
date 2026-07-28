use serde::{Deserialize, Serialize};

pub const PROTOCOL_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize)]
pub struct V1Request {
    pub protocol: u32,
    pub instance_id: String,
    pub config: toml::Value,
}

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

#[derive(Debug, Clone)]
pub enum PluginOutput {
    Success(V1Response),
    Timeout,
    ExitError { code: Option<i32>, stderr: String },
    ParseError(String),
    Oversized,
    Cancelled,
}

pub fn parse_response(data: &[u8]) -> Result<V1Response, String> {
    serde_json::from_slice(data).map_err(|e| format!("JSON: {e}"))
}
