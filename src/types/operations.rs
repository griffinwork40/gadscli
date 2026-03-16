#![allow(dead_code)]

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MutateOperation<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_mask: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MutateRequest<T: Serialize> {
    pub customer_id: String,
    pub operations: Vec<MutateOperation<T>>,
    pub partial_failure: bool,
    pub validate_only: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct FieldMask {
    pub paths: Vec<String>,
}
