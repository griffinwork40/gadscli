#![allow(dead_code)]

use crate::client::GoogleAdsClient;
use crate::commands::keyword::{CampaignCriterionMutate, KeywordInfo};
use crate::error::Result;
use crate::types::operations::MutateOperation;

pub async fn handle_exclude_terms(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    campaign_id: &str,
    min_cost_micros: Option<i64>,
    max_conversions: Option<f64>,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    let mut query = format!(
        "SELECT search_term_view.search_term, search_term_view.status, \
         metrics.cost_micros, metrics.conversions \
         FROM search_term_view \
         WHERE campaign.resource_name = 'customers/{}/campaigns/{}' \
         AND search_term_view.status = 'NONE'",
        cid, campaign_id
    );

    if let Some(min_cost) = min_cost_micros {
        query.push_str(&format!(" AND metrics.cost_micros >= {}", min_cost));
    }
    if let Some(max_conv) = max_conversions {
        query.push_str(&format!(" AND metrics.conversions <= {}", max_conv));
    }

    let rows = client.search_all(&cid, &query, Some(1000)).await?;

    let terms: Vec<String> = rows
        .iter()
        .filter_map(|r| r.search_term_view.as_ref())
        .filter_map(|stv| stv.search_term.clone())
        .collect();

    if terms.is_empty() {
        println!("No search terms match the filters.");
        return Ok(());
    }

    println!("Found {} search term(s) to exclude:", terms.len());
    for term in &terms {
        println!("  - {}", term);
    }

    if dry_run {
        println!(
            "\n[DRY RUN] Would add {} campaign negative keyword(s).",
            terms.len()
        );
        return Ok(());
    }

    let ops: Vec<MutateOperation<CampaignCriterionMutate>> = terms
        .iter()
        .map(|term| MutateOperation {
            create: Some(CampaignCriterionMutate {
                campaign: Some(format!("customers/{}/campaigns/{}", cid, campaign_id)),
                keyword: Some(KeywordInfo {
                    text: term.clone(),
                    match_type: "EXACT".to_string(),
                }),
                negative: Some(true),
                ..Default::default()
            }),
            update: None,
            remove: None,
            update_mask: None,
        })
        .collect();

    let response = client
        .mutate(&cid, "campaignCriteria", ops, true, false)
        .await?;
    println!("Added {} campaign negative keyword(s).", response.results.len());

    if let Some(errors) = &response.partial_failure_error {
        eprintln!("Partial failures: {}", errors);
    }

    Ok(())
}
