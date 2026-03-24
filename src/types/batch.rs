use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchJob {
    #[serde(default)]
    pub resource_name: Option<String>,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub metadata: Option<BatchJobMetadata>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchJobMetadata {
    #[serde(default)]
    pub estimated_completion_ratio: Option<f64>,
    #[serde(default)]
    pub operation_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AddOperationsResponse {
    #[serde(default)]
    pub next_sequence_token: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LongRunningOperation {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub done: Option<bool>,
    #[serde(default)]
    pub metadata: Option<serde_json::Value>,
    #[serde(default)]
    pub error: Option<serde_json::Value>,
}
