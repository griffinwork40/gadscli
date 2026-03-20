#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::types::resources::deserialize_optional_i64;
use super::responses::KeywordCriterion;

// Campaign criterion info types

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct LocationInfo {
    pub geo_target_constant: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct DeviceInfo {
    #[serde(rename = "type")]
    pub device_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct AdScheduleInfo {
    pub day_of_week: Option<String>,
    pub start_hour: Option<i32>,
    pub end_hour: Option<i32>,
    pub start_minute: Option<String>,
    pub end_minute: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct UserListInfo {
    pub user_list: Option<String>,
}

// Campaign asset linking

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CampaignAssetRow {
    pub resource_name: String,
    pub campaign: Option<String>,
    pub asset: Option<String>,
    pub field_type: Option<String>,
    pub status: Option<String>,
}

// Shared sets (negative keyword lists)

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SharedSet {
    pub resource_name: String,
    pub id: Option<String>,
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub set_type: Option<String>,
    pub status: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub member_count: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SharedCriterionRow {
    pub resource_name: String,
    pub shared_set: Option<String>,
    pub keyword: Option<KeywordCriterion>,
    pub criterion_id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CampaignSharedSetRow {
    pub resource_name: String,
    pub campaign: Option<String>,
    pub shared_set: Option<String>,
    pub status: Option<String>,
}

// Search term view

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SearchTermViewRow {
    pub resource_name: String,
    pub search_term: Option<String>,
    pub ad_group: Option<String>,
    pub status: Option<String>,
}

// Geo target constant (for location search)

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct GeoTargetConstantRow {
    pub resource_name: String,
    pub id: Option<String>,
    pub name: Option<String>,
    pub canonical_name: Option<String>,
    pub country_code: Option<String>,
    pub target_type: Option<String>,
    pub status: Option<String>,
}

// Keyword ideas

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct KeywordIdeaMetrics {
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub avg_monthly_searches: Option<i64>,
    pub competition: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub low_top_of_page_bid_micros: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub high_top_of_page_bid_micros: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct KeywordIdeaResult {
    pub text: Option<String>,
    pub keyword_idea_metrics: Option<KeywordIdeaMetrics>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct GenerateKeywordIdeasResponse {
    pub results: Vec<KeywordIdeaResult>,
}
