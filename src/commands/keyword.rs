#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::client::GoogleAdsClient;
use crate::error::Result;

use super::keyword_list;
use super::keyword_mutate;
use super::keyword_exclude;
use super::keyword_ideas;

/// Keyword criterion create/update payload
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdGroupCriterionMutate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ad_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<KeywordInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpc_bid_micros: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative: Option<bool>,
}

/// Campaign-level criterion (used for campaign negative keywords)
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CampaignCriterionMutate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub campaign: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keyword: Option<KeywordInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub negative: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeywordInfo {
    pub text: String,
    pub match_type: String,
}

/// Top-level handle function called by mod.rs dispatcher
pub async fn handle(command: &crate::cli::KeywordCommands, client: &GoogleAdsClient, cli: &crate::cli::Cli) -> Result<()> {
    let cid_override = cli.customer_id.as_deref();
    let dry_run = cli.dry_run;
    match command {
        crate::cli::KeywordCommands::List { ad_group_id, campaign_id } => {
            keyword_list::handle_list(client, cid_override, ad_group_id.as_deref(), campaign_id.as_deref()).await
        }
        crate::cli::KeywordCommands::Add { ad_group_id, text, match_type, cpc_bid_micros, negative } => {
            keyword_mutate::handle_add(client, cid_override, ad_group_id, text, match_type, *cpc_bid_micros, *negative, dry_run).await
        }
        crate::cli::KeywordCommands::Remove { id } => {
            keyword_mutate::handle_remove(client, cid_override, id, dry_run).await
        }
        crate::cli::KeywordCommands::Update { id, status, cpc_bid_micros } => {
            keyword_mutate::handle_update(client, cid_override, id, status.as_deref(), *cpc_bid_micros, dry_run).await
        }
        crate::cli::KeywordCommands::AddNegative { campaign_id, text, match_type } => {
            keyword_mutate::handle_add_campaign_negative(client, cid_override, campaign_id, text, match_type, dry_run).await
        }
        crate::cli::KeywordCommands::ListNegatives { ad_group_id, campaign_id } => {
            keyword_list::handle_list_negatives(client, cid_override, ad_group_id.as_deref(), campaign_id.as_deref()).await
        }
        crate::cli::KeywordCommands::RemoveNegative { id } => {
            keyword_mutate::handle_remove_negative(client, cid_override, id, dry_run).await
        }
        crate::cli::KeywordCommands::AddBulk { ad_group_id, keywords, match_type, cpc_bid_micros } => {
            keyword_mutate::handle_add_bulk(client, cid_override, ad_group_id, keywords, match_type, *cpc_bid_micros, dry_run).await
        }
        crate::cli::KeywordCommands::ExcludeTerms { campaign_id, min_cost_micros, max_conversions } => {
            keyword_exclude::handle_exclude_terms(client, cid_override, campaign_id, *min_cost_micros, *max_conversions, dry_run).await
        }
        crate::cli::KeywordCommands::Ideas { text, url, language, geo_ids } => {
            keyword_ideas::handle_ideas(client, cid_override, text, url.as_deref(), language.as_deref(), geo_ids).await
        }
    }
}
