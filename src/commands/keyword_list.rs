#![allow(dead_code)]

use crate::client::GoogleAdsClient;
use crate::error::Result;

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

pub async fn handle_list_negatives(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    ad_group_id: Option<&str>,
    campaign_id: Option<&str>,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    // Campaign-level negatives
    let mut campaign_query = String::from(
        "SELECT campaign_criterion.criterion_id, campaign_criterion.keyword.text, \
         campaign_criterion.keyword.match_type, campaign_criterion.negative, \
         campaign.name, campaign.id \
         FROM campaign_criterion \
         WHERE campaign_criterion.type = 'KEYWORD' AND campaign_criterion.negative = true",
    );

    if let Some(cid_filter) = campaign_id {
        campaign_query.push_str(&format!(" AND campaign.id = {}", cid_filter));
    }

    let campaign_rows = client.search_all(&cid, &campaign_query, Some(1000)).await?;

    // Ad group-level negatives
    let mut ag_query = String::from(
        "SELECT ad_group_criterion.criterion_id, ad_group_criterion.keyword.text, \
         ad_group_criterion.keyword.match_type, ad_group_criterion.negative, \
         ad_group.name, campaign.name \
         FROM ad_group_criterion \
         WHERE ad_group_criterion.negative = true AND ad_group_criterion.status != 'REMOVED'",
    );

    if let Some(ag_id) = ad_group_id {
        ag_query.push_str(&format!(
            " AND ad_group_criterion.ad_group = 'customers/{}/adGroups/{}'",
            cid, ag_id
        ));
    }

    if let Some(cid_filter) = campaign_id {
        ag_query.push_str(&format!(
            " AND campaign.resource_name = 'customers/{}/campaigns/{}'",
            cid, cid_filter
        ));
    }

    let ag_rows = client.search_all(&cid, &ag_query, Some(1000)).await?;

    if campaign_rows.is_empty() && ag_rows.is_empty() {
        println!("No negative keywords found.");
        return Ok(());
    }

    if !campaign_rows.is_empty() {
        println!("Campaign-Level Negative Keywords:");
        println!(
            "{:<12} {:<40} {:<10} {:<30}",
            "ID", "Text", "Match Type", "Campaign"
        );
        println!("{}", "-".repeat(95));

        for row in &campaign_rows {
            if let Some(criterion) = &row.campaign_criterion {
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
                let campaign_name = row
                    .campaign
                    .as_ref()
                    .and_then(|c| c.name.clone())
                    .unwrap_or_else(|| "-".to_string());
                println!("{:<12} {:<40} {:<10} {:<30}", id, text, match_type, campaign_name);
            }
        }
        println!("\nTotal: {} campaign negative(s)", campaign_rows.len());
    }

    if !ag_rows.is_empty() {
        if !campaign_rows.is_empty() {
            println!();
        }
        println!("Ad Group-Level Negative Keywords:");
        println!(
            "{:<12} {:<40} {:<10} {:<30}",
            "ID", "Text", "Match Type", "Ad Group"
        );
        println!("{}", "-".repeat(95));

        for row in &ag_rows {
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
                let ag_name = row
                    .ad_group
                    .as_ref()
                    .and_then(|a| a.name.clone())
                    .unwrap_or_else(|| "-".to_string());
                println!("{:<12} {:<40} {:<10} {:<30}", id, text, match_type, ag_name);
            }
        }
        println!("\nTotal: {} ad group negative(s)", ag_rows.len());
    }

    Ok(())
}
