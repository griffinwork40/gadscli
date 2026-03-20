#![allow(dead_code)]

use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

use super::keyword::{AdGroupCriterionMutate, CampaignCriterionMutate, KeywordInfo};

pub async fn handle_add(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    ad_group_id: &str,
    text: &str,
    match_type: &str,
    cpc_bid_micros: Option<i64>,
    negative: bool,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;
    let neg_label = if negative { "negative keyword" } else { "keyword" };

    if dry_run {
        println!("[DRY RUN] Would add {}:", neg_label);
        println!("  Ad Group: customers/{}/adGroups/{}", cid, ad_group_id);
        println!("  Text: {}", text);
        println!("  Match Type: {}", match_type);
        if let Some(bid) = cpc_bid_micros {
            println!("  CPC Bid: ${:.2}", bid as f64 / 1_000_000.0);
        }
        return Ok(());
    }

    let mut criterion = AdGroupCriterionMutate {
        ad_group: Some(format!("customers/{}/adGroups/{}", cid, ad_group_id)),
        status: Some("ENABLED".to_string()),
        keyword: Some(KeywordInfo {
            text: text.to_string(),
            match_type: match_type.to_uppercase(),
        }),
        negative: if negative { Some(true) } else { None },
        ..Default::default()
    };

    if let Some(bid) = cpc_bid_micros {
        criterion.cpc_bid_micros = Some(bid);
    }

    let ops: Vec<MutateOperation<AdGroupCriterionMutate>> = vec![MutateOperation {
        create: Some(criterion),
        update: None,
        remove: None,
        update_mask: None,
    }];

    let response = client
        .mutate(&cid, "adGroupCriteria", ops, false, dry_run)
        .await?;

    if let Some(result) = response.results.first() {
        println!("{} added: {}", if negative { "Negative keyword" } else { "Keyword" }, result.resource_name);
    } else {
        println!("{} added successfully.", if negative { "Negative keyword" } else { "Keyword" });
    }

    Ok(())
}

pub async fn handle_remove(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    id: &str,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    // id may be a full resource name or just the criterion ID
    let resource_name = if id.starts_with("customers/") {
        id.to_string()
    } else {
        // We need the ad group too; treat id as "adGroupId~criterionId"
        // If no tilde, treat as bare criterion path segment
        format!("customers/{}/adGroupCriteria/{}", cid, id)
    };

    if dry_run {
        println!("[DRY RUN] Would remove keyword: {}", resource_name);
        return Ok(());
    }

    let ops: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
        create: None, update: None, remove: Some(resource_name.clone()), update_mask: None,
    }];
    client.mutate(&cid, "adGroupCriteria", ops, false, dry_run).await?;
    println!("Keyword removed: {}", resource_name);
    Ok(())
}

pub async fn handle_update(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    id: &str,
    status: Option<&str>,
    cpc_bid_micros: Option<i64>,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    let resource_name = if id.starts_with("customers/") {
        id.to_string()
    } else {
        format!("customers/{}/adGroupCriteria/{}", cid, id)
    };

    if dry_run {
        println!("[DRY RUN] Would update keyword: {}", resource_name);
        if let Some(s) = status {
            println!("  Status: {}", s);
        }
        if let Some(bid) = cpc_bid_micros {
            println!("  CPC Bid: ${:.2}", bid as f64 / 1_000_000.0);
        }
        return Ok(());
    }

    let mut update_fields: Vec<String> = Vec::new();
    let mut criterion = AdGroupCriterionMutate {
        resource_name: Some(resource_name.clone()),
        ..Default::default()
    };

    if let Some(s) = status {
        criterion.status = Some(s.to_uppercase());
        update_fields.push("status".to_string());
    }

    if let Some(bid) = cpc_bid_micros {
        criterion.cpc_bid_micros = Some(bid);
        update_fields.push("cpc_bid_micros".to_string());
    }

    let update_mask = update_fields.join(",");

    let ops: Vec<MutateOperation<AdGroupCriterionMutate>> = vec![MutateOperation {
        create: None,
        update: Some(criterion),
        remove: None,
        update_mask: Some(update_mask),
    }];

    client
        .mutate(&cid, "adGroupCriteria", ops, false, dry_run)
        .await?;

    println!("Keyword updated: {}", resource_name);
    Ok(())
}

/// Add a campaign-level negative keyword
pub async fn handle_add_campaign_negative(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    campaign_id: &str,
    text: &str,
    match_type: &str,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    if dry_run {
        println!("[DRY RUN] Would add campaign negative keyword:");
        println!("  Campaign: customers/{}/campaigns/{}", cid, campaign_id);
        println!("  Text: {}", text);
        println!("  Match Type: {}", match_type);
        return Ok(());
    }

    let criterion = CampaignCriterionMutate {
        campaign: Some(format!("customers/{}/campaigns/{}", cid, campaign_id)),
        keyword: Some(KeywordInfo {
            text: text.to_string(),
            match_type: match_type.to_uppercase(),
        }),
        negative: Some(true),
        ..Default::default()
    };

    let ops: Vec<MutateOperation<CampaignCriterionMutate>> = vec![MutateOperation {
        create: Some(criterion),
        update: None,
        remove: None,
        update_mask: None,
    }];

    let response = client
        .mutate(&cid, "campaignCriteria", ops, false, dry_run)
        .await?;

    if let Some(result) = response.results.first() {
        println!("Campaign negative keyword added: {}", result.resource_name);
    } else {
        println!("Campaign negative keyword added successfully.");
    }

    Ok(())
}

/// Add multiple keywords at once
pub async fn handle_add_bulk(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    ad_group_id: &str,
    keywords: &[String],
    match_type: &str,
    cpc_bid_micros: Option<i64>,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    if dry_run {
        println!("[DRY RUN] Would add {} keyword(s):", keywords.len());
        for kw in keywords {
            println!("  - {} ({})", kw, match_type);
        }
        return Ok(());
    }

    let ops: Vec<MutateOperation<AdGroupCriterionMutate>> = keywords
        .iter()
        .map(|kw| {
            let mut criterion = AdGroupCriterionMutate {
                ad_group: Some(format!("customers/{}/adGroups/{}", cid, ad_group_id)),
                status: Some("ENABLED".to_string()),
                keyword: Some(KeywordInfo {
                    text: kw.clone(),
                    match_type: match_type.to_uppercase(),
                }),
                ..Default::default()
            };
            if let Some(bid) = cpc_bid_micros {
                criterion.cpc_bid_micros = Some(bid);
            }
            MutateOperation {
                create: Some(criterion),
                update: None,
                remove: None,
                update_mask: None,
            }
        })
        .collect();

    let response = client
        .mutate(&cid, "adGroupCriteria", ops, true, false)
        .await?;

    println!("Added {} keyword(s).", response.results.len());
    for result in &response.results {
        println!("  {}", result.resource_name);
    }

    if let Some(errors) = &response.partial_failure_error {
        eprintln!("Partial failures: {}", errors);
    }

    Ok(())
}

/// Remove a campaign-level negative keyword
pub async fn handle_remove_negative(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    id: &str,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    let resource_name = if id.starts_with("customers/") {
        id.to_string()
    } else {
        format!("customers/{}/campaignCriteria/{}", cid, id)
    };

    if dry_run {
        println!("[DRY RUN] Would remove negative keyword: {}", resource_name);
        return Ok(());
    }

    let ops: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
        create: None, update: None, remove: Some(resource_name.clone()), update_mask: None,
    }];
    client.mutate(&cid, "campaignCriteria", ops, false, dry_run).await?;
    println!("Negative keyword removed: {}", resource_name);
    Ok(())
}
