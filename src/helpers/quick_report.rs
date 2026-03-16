use crate::cli::Cli;
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn run(client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;
    let query = "SELECT campaign.name, metrics.impressions, metrics.clicks, \
                 metrics.cost_micros, metrics.conversions \
                 FROM campaign \
                 WHERE campaign.status != 'REMOVED' \
                 AND segments.date DURING LAST_7_DAYS \
                 ORDER BY metrics.cost_micros DESC \
                 LIMIT 20";

    let results = client.search_all(&cid, query, cli.page_size).await?;

    if results.is_empty() {
        println!("No campaign data found for the last 7 days.");
        return Ok(());
    }

    println!("Quick Report -- Last 7 Days (Top 20 Campaigns)");
    println!("{}", "-".repeat(90));
    println!(
        "{:<40} {:>12} {:>8} {:>12} {:>12}",
        "Campaign", "Impressions", "Clicks", "Cost", "Conversions"
    );
    println!("{}", "-".repeat(90));

    for row in &results {
        let name = row
            .campaign
            .as_ref()
            .and_then(|c| c.name.as_deref())
            .unwrap_or("-");
        let name_truncated = if name.len() > 38 {
            format!("{}.", &name[..38])
        } else {
            name.to_string()
        };

        let impressions = row
            .metrics
            .as_ref()
            .and_then(|m| m.impressions)
            .unwrap_or(0);
        let clicks = row
            .metrics
            .as_ref()
            .and_then(|m| m.clicks)
            .unwrap_or(0);
        let cost = row
            .metrics
            .as_ref()
            .and_then(|m| m.cost_micros)
            .map(|c| format!("${:.2}", c as f64 / 1_000_000.0))
            .unwrap_or_else(|| "$0.00".to_string());
        let conversions = row
            .metrics
            .as_ref()
            .and_then(|m| m.conversions)
            .unwrap_or(0.0);

        println!(
            "{:<40} {:>12} {:>8} {:>12} {:>12.1}",
            name_truncated, impressions, clicks, cost, conversions
        );
    }

    println!("{}", "-".repeat(90));
    println!("{} campaign(s) returned", results.len());

    Ok(())
}
