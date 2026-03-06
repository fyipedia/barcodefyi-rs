use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResult {
    pub query: String,
    pub results: Vec<SearchItem>,
    pub total: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchItem {
    pub name: String,
    pub slug: String,
    #[serde(rename = "type")]
    pub item_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SymbologyDetail {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FamilyDetail {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StandardDetail {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ComponentDetail {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GlossaryTerm {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompareResult {
    #[serde(flatten)]
    pub data: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IndustryDetail {
    pub name: String,
    pub slug: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

#[derive(Debug, thiserror::Error)]
pub enum BarcodeFyiError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    #[error("API error (HTTP {status}): {body}")]
    Api { status: u16, body: String },
}
