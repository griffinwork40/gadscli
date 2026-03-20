#![allow(dead_code)]

use serde::{Deserialize, Deserializer, Serialize};

use crate::types::common::{
    AdGroupStatus, AdType, AssetType, BiddingStrategyType, CampaignStatus, CampaignType,
    ConversionActionType, KeywordMatchType,
};

/// Deserialize a value that may be either a number or a string containing a number.
/// The Google Ads REST API returns all numeric values as JSON strings.
pub fn deserialize_optional_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrNum {
        Int(i64),
        Float(f64),
        Str(String),
    }

    Option::<StringOrNum>::deserialize(deserializer)?
        .map(|v| match v {
            StringOrNum::Int(i) => Ok(i),
            StringOrNum::Float(f) => Ok(f as i64),
            StringOrNum::Str(s) => s.parse::<i64>().map_err(de::Error::custom),
        })
        .transpose()
}

pub fn deserialize_optional_f64<'de, D>(deserializer: D) -> Result<Option<f64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrFloat {
        Float(f64),
        Str(String),
    }

    Option::<StringOrFloat>::deserialize(deserializer)?
        .map(|v| match v {
            StringOrFloat::Float(f) => Ok(f),
            StringOrFloat::Str(s) => s.parse::<f64>().map_err(de::Error::custom),
        })
        .transpose()
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Campaign {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CampaignStatus>,
    #[serde(rename = "advertisingChannelType", skip_serializing_if = "Option::is_none")]
    pub campaign_type: Option<CampaignType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bidding_strategy_type: Option<BiddingStrategyType>,
    #[serde(rename = "campaignBudget", skip_serializing_if = "Option::is_none")]
    pub budget: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct AdGroup {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AdGroupStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64", skip_serializing_if = "Option::is_none")]
    pub cpc_bid_micros: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Ad {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub ad_type: Option<AdType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub final_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsive_search_ad: Option<ResponsiveSearchAdInfo>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ResponsiveSearchAdInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headlines: Option<Vec<AdTextAsset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptions: Option<Vec<AdTextAsset>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct AdTextAsset {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_field: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Keyword {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "keyword.text", skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_type: Option<KeywordMatchType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AdGroupStatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64", skip_serializing_if = "Option::is_none")]
    pub cpc_bid_micros: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Budget {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, deserialize_with = "deserialize_optional_i64", skip_serializing_if = "Option::is_none")]
    pub amount_micros: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct BiddingStrategy {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<BiddingStrategyType>,
    #[serde(default, deserialize_with = "deserialize_optional_i64", skip_serializing_if = "Option::is_none")]
    pub target_cpa_micros: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_f64", skip_serializing_if = "Option::is_none")]
    pub target_roas: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Asset {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub asset_type: Option<AssetType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Label {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_label: Option<TextLabel>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct TextLabel {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_color: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct ConversionAction {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub action_type: Option<ConversionActionType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Recommendation {
    pub resource_name: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub recommendation_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub impact: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct CustomerClient {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Customer {
    pub resource_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptive_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub manager: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase", default)]
pub struct Metrics {
    #[serde(default, deserialize_with = "deserialize_optional_i64", skip_serializing_if = "Option::is_none")]
    pub impressions: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64", skip_serializing_if = "Option::is_none")]
    pub clicks: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_i64", skip_serializing_if = "Option::is_none")]
    pub cost_micros: Option<i64>,
    #[serde(default, deserialize_with = "deserialize_optional_f64", skip_serializing_if = "Option::is_none")]
    pub conversions: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_optional_f64", skip_serializing_if = "Option::is_none")]
    pub conversions_value: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_optional_f64", skip_serializing_if = "Option::is_none")]
    pub ctr: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_optional_f64", skip_serializing_if = "Option::is_none")]
    pub average_cpc: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_optional_f64", skip_serializing_if = "Option::is_none")]
    pub average_cpm: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_optional_f64", skip_serializing_if = "Option::is_none")]
    pub all_conversions: Option<f64>,
    #[serde(default, deserialize_with = "deserialize_optional_f64", skip_serializing_if = "Option::is_none")]
    pub interaction_rate: Option<f64>,
}
