#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

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
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct KeywordInfo {
    pub text: String,
    pub match_type: String,
}

pub async fn handle_list(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    ad_group_id: Option<&str>,
    campaign_id: Option<&str>,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    let mut query = String::from(
        "SELECT ad_group_criterion.criterion_id, ad_group_criterion.keyword.text, \
         ad_group_criterion.keyword.match_type, ad_group_criterion.status, \
         ad_group_criterion.ad_group, ad_group_criterion.cpc_bid_micros, \
         metrics.impressions, metrics.clicks, metrics.cost_micros \
         FROM keyword_view \
         WHERE ad_group_criterion.status != 'REMOVED'",
    );

    if let Some(ag_id) = ad_group_id {
        query.push_str(&format!(
            " AND ad_group_criterion.ad_group = 'customers/{}/adGroups/{}'",
            cid, ag_id
        ));
    }

    if let Some(cid_filter) = campaign_id {
        query.push_str(&format!(
            " AND campaign.resource_name = 'customers/{}/campaigns/{}'",
            cid, cid_filter
        ));
    }

    let rows = client.search_all(&cid, &query, Some(1000)).await?;

    if rows.is_empty() {
        println!("No keywords found.");
        return Ok(());
    }

    println!(
        "{:<12} {:<40} {:<10} {:<10} {:<14} {:<14} {:<14}",
        "ID", "Text", "Match Type", "Status", "CPC Bid", "Impressions", "Clicks"
    );
    println!("{}", "-".repeat(120));

    for row in &rows {
        if let Some(criterion) = &row.ad_group_criterion {
            let id = criterion
                .criterion_id
                .clone()
                .unwrap_or_else(|| "-".to_string());

            let text = criterion
                .keyword
                .as_ref()
                .and_then(|k| k.text.clone())
                .unwrap_or_else(|| "-".to_string());

            let match_type = criterion
                .keyword
                .as_ref()
                .and_then(|k| k.match_type.as_ref())
                .map(|m| m.to_string())
                .unwrap_or_else(|| "-".to_string());

            let status = criterion
                .status
                .clone()
                .unwrap_or_else(|| "-".to_string());

            let cpc = criterion
                .cpc_bid_micros
                .map(|v| format!("${:.2}", v as f64 / 1_000_000.0))
                .unwrap_or_else(|| "-".to_string());

            let impressions = row
                .metrics
                .as_ref()
                .and_then(|m| m.impressions)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "0".to_string());

            let clicks = row
                .metrics
                .as_ref()
                .and_then(|m| m.clicks)
                .map(|v| v.to_string())
                .unwrap_or_else(|| "0".to_string());

            println!(
                "{:<12} {:<40} {:<10} {:<10} {:<14} {:<14} {:<14}",
                id, text, match_type, status, cpc, impressions, clicks
            );
        }
    }

    println!("\nTotal: {} keyword(s)", rows.len());
    Ok(())
}

pub async fn handle_add(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    ad_group_id: &str,
    text: &str,
    match_type: &str,
    cpc_bid_micros: Option<i64>,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    if dry_run {
        println!("[DRY RUN] Would add keyword:");
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
        println!("Keyword added: {}", result.resource_name);
    } else {
        println!("Keyword added successfully.");
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
        create: None,
        update: None,
        remove: Some(resource_name.clone()),
        update_mask: None,
    }];

    client
        .mutate(&cid, "adGroupCriteria", ops, false, dry_run)
        .await?;

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

/// Top-level handle function called by mod.rs dispatcher
pub async fn handle(command: &crate::cli::KeywordCommands, client: &GoogleAdsClient, cli: &crate::cli::Cli) -> Result<()> {
    let cid_override = cli.customer_id.as_deref();
    let dry_run = cli.dry_run;
    match command {
        crate::cli::KeywordCommands::List { ad_group_id, campaign_id } => {
            handle_list(client, cid_override, ad_group_id.as_deref(), campaign_id.as_deref()).await
        }
        crate::cli::KeywordCommands::Add { ad_group_id, text, match_type, cpc_bid_micros } => {
            handle_add(client, cid_override, ad_group_id, text, match_type, *cpc_bid_micros, dry_run).await
        }
        crate::cli::KeywordCommands::Remove { id } => {
            handle_remove(client, cid_override, id, dry_run).await
        }
        crate::cli::KeywordCommands::Update { id, status, cpc_bid_micros } => {
            handle_update(client, cid_override, id, status.as_deref(), *cpc_bid_micros, dry_run).await
        }
    }
}

/// Dispatch keyword subcommands
pub enum KeywordCommand {
    List {
        ad_group_id: Option<String>,
        campaign_id: Option<String>,
    },
    Add {
        ad_group_id: String,
        text: String,
        match_type: String,
        cpc_bid_micros: Option<i64>,
    },
    Remove {
        id: String,
    },
    Update {
        id: String,
        status: Option<String>,
        cpc_bid_micros: Option<i64>,
    },
}

pub async fn execute(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    cmd: KeywordCommand,
    dry_run: bool,
) -> Result<()> {
    match cmd {
        KeywordCommand::List {
            ad_group_id,
            campaign_id,
        } => {
            handle_list(
                client,
                customer_id_override,
                ad_group_id.as_deref(),
                campaign_id.as_deref(),
            )
            .await
        }
        KeywordCommand::Add {
            ad_group_id,
            text,
            match_type,
            cpc_bid_micros,
        } => {
            handle_add(
                client,
                customer_id_override,
                &ad_group_id,
                &text,
                &match_type,
                cpc_bid_micros,
                dry_run,
            )
            .await
        }
        KeywordCommand::Remove { id } => {
            handle_remove(client, customer_id_override, &id, dry_run).await
        }
        KeywordCommand::Update {
            id,
            status,
            cpc_bid_micros,
        } => {
            handle_update(
                client,
                customer_id_override,
                &id,
                status.as_deref(),
                cpc_bid_micros,
                dry_run,
            )
            .await
        }
    }
}
