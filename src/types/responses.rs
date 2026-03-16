#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::types::resources::{
    Ad, AdGroup, Asset, BiddingStrategy, Budget, ConversionAction, CustomerClient, Label, Metrics,
    Recommendation,
};

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SearchResponse {
    pub results: Vec<SearchRow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_results_count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_mask: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct SearchRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<crate::types::resources::Campaign>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<AdGroup>,
    #[serde(rename = "adGroupAd", skip_serializing_if = "Option::is_none")]
    pub ad_group_ad: Option<AdGroupAdRow>,
    #[serde(rename = "adGroupCriterion", skip_serializing_if = "Option::is_none")]
    pub ad_group_criterion: Option<AdGroupCriterionRow>,
    #[serde(rename = "campaignBudget", skip_serializing_if = "Option::is_none")]
    pub campaign_budget: Option<Budget>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidding_strategy: Option<BiddingStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset: Option<Asset>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conversion_action: Option<ConversionAction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recommendation: Option<Recommendation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_client: Option<CustomerClient>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metrics: Option<Metrics>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub segments: Option<serde_json::Value>,
}

/// Wrapper for an ad inside a search row (adGroupAd contains the ad nested)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct AdGroupAdRow {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad: Option<Ad>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<String>,
}

/// Wrapper for a keyword/criterion inside a search row
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct AdGroupCriterionRow {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub criterion_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<KeywordCriterion>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpc_bid_micros: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct KeywordCriterion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<crate::types::common::KeywordMatchType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct MutateResponse {
    pub results: Vec<MutateResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub partial_failure_error: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct MutateResult {
    pub resource_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct FieldMetadata {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selectable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filterable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sortable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_repeated: Option<bool>,
}
