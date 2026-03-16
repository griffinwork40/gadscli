use crate::cli::Cli;
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn run(client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;

    println!("Account Audit Report");
    println!("{}", "=".repeat(60));
    println!();

    // 1. Campaign status distribution
    let campaign_query = "SELECT campaign.status, metrics.cost_micros FROM campaign";
    let campaign_rows = client
        .search_all(&cid, campaign_query, cli.page_size)
        .await?;

    let mut enabled_campaigns = 0usize;
    let mut paused_campaigns = 0usize;
    let mut removed_campaigns = 0usize;
    let mut total_cost_micros: i64 = 0;

    for row in &campaign_rows {
        if let Some(campaign) = &row.campaign {
            match &campaign.status {
                Some(crate::types::common::CampaignStatus::Enabled) => enabled_campaigns += 1,
                Some(crate::types::common::CampaignStatus::Paused) => paused_campaigns += 1,
                Some(crate::types::common::CampaignStatus::Removed) => removed_campaigns += 1,
                _ => {}
            }
        }
        if let Some(metrics) = &row.metrics {
            total_cost_micros += metrics.cost_micros.unwrap_or(0);
        }
    }

    println!("Campaigns");
    println!("{}", "-".repeat(40));
    println!("  Enabled:  {}", enabled_campaigns);
    println!("  Paused:   {}", paused_campaigns);
    println!("  Removed:  {}", removed_campaigns);
    println!("  Total:    {}", campaign_rows.len());
    println!(
        "  Total spend (all time): ${:.2}",
        total_cost_micros as f64 / 1_000_000.0
    );
    println!();

    // 2. Ad group count
    let ad_group_query = "SELECT ad_group.status FROM ad_group";
    let ad_group_rows = client
        .search_all(&cid, ad_group_query, cli.page_size)
        .await?;

    let mut enabled_ag = 0usize;
    let mut paused_ag = 0usize;

    for row in &ad_group_rows {
        if let Some(ag) = &row.ad_group {
            match &ag.status {
                Some(crate::types::common::AdGroupStatus::Enabled) => enabled_ag += 1,
                Some(crate::types::common::AdGroupStatus::Paused) => paused_ag += 1,
                _ => {}
            }
        }
    }

    println!("Ad Groups");
    println!("{}", "-".repeat(40));
    println!("  Enabled:  {}", enabled_ag);
    println!("  Paused:   {}", paused_ag);
    println!("  Total:    {}", ad_group_rows.len());
    println!();

    // 3. Keyword quality scores
    let quality_query = "SELECT ad_group_criterion.quality_info.quality_score \
                         FROM keyword_view \
                         WHERE ad_group_criterion.status = 'ENABLED'";
    let quality_rows = client
        .search_all(&cid, quality_query, cli.page_size)
        .await?;

    let qs_count = quality_rows
        .iter()
        .filter(|row| {
            row.ad_group_criterion
                .as_ref()
                .map(|c| !c.resource_name.is_empty())
                .unwrap_or(false)
        })
        .count();

    println!("Keywords (Enabled)");
    println!("{}", "-".repeat(40));
    println!("  Total enabled keywords: {}", qs_count);
    if qs_count == 0 {
        println!("  No enabled keywords found.");
    }
    println!();

    // 4. Recommendations
    let rec_query = "SELECT recommendation.type FROM recommendation";
    let rec_rows = client.search_all(&cid, rec_query, cli.page_size).await?;

    let mut rec_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    for row in &rec_rows {
        if let Some(rec) = &row.recommendation {
            let rec_type = rec
                .recommendation_type
                .clone()
                .unwrap_or_else(|| "UNKNOWN".to_string());
            *rec_counts.entry(rec_type).or_insert(0) += 1;
        }
    }

    println!("Recommendations");
    println!("{}", "-".repeat(40));
    println!("  Total pending: {}", rec_rows.len());
    let mut rec_types: Vec<(&String, &usize)> = rec_counts.iter().collect();
    rec_types.sort_by(|a, b| b.1.cmp(a.1));
    for (rec_type, count) in rec_types.iter().take(5) {
        println!("  {}: {}", rec_type, count);
    }
    println!();

    println!("{}", "=".repeat(60));
    println!("Audit complete.");

    Ok(())
}
