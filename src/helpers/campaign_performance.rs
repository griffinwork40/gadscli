use crate::cli::Cli;
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn run(client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;
    let query = "SELECT campaign.id, campaign.name, campaign.status, \
                 campaign.advertising_channel_type, \
                 metrics.impressions, metrics.clicks, metrics.ctr, \
                 metrics.cost_micros, metrics.conversions, metrics.conversions_value, \
                 metrics.average_cpc \
                 FROM campaign \
                 WHERE campaign.status = 'ENABLED' \
                 ORDER BY metrics.cost_micros DESC";

    let results = client.search_all(&cid, query, cli.page_size).await?;

    if results.is_empty() {
        println!("No enabled campaigns found.");
        return Ok(());
    }

    println!("Campaign Performance Summary");
    println!("{}", "=".repeat(110));
    println!(
        "{:<12} {:<35} {:<10} {:>10} {:>8} {:>7} {:>12} {:>12} {:>10}",
        "ID", "Name", "Type", "Impr.", "Clicks", "CTR%", "Cost", "Conv.", "Avg CPC"
    );
    println!("{}", "-".repeat(110));

    for row in &results {
        let campaign = row.campaign.as_ref();
        let metrics = row.metrics.as_ref();

        let id = campaign
            .and_then(|c| c.id)
            .map(|i| i.to_string())
            .unwrap_or_default();
        let name = campaign
            .and_then(|c| c.name.as_deref())
            .unwrap_or("-");
        let name_truncated = if name.len() > 33 {
            format!("{}.", &name[..33])
        } else {
            name.to_string()
        };
        let ctype = campaign
            .and_then(|c| c.campaign_type.as_ref())
            .map(|t| t.to_string())
            .unwrap_or_default();
        let ctype_truncated = if ctype.len() > 9 {
            format!("{}.", &ctype[..9])
        } else {
            ctype
        };

        let impressions = metrics.and_then(|m| m.impressions).unwrap_or(0);
        let clicks = metrics.and_then(|m| m.clicks).unwrap_or(0);
        let ctr = metrics
            .and_then(|m| m.ctr)
            .map(|c| format!("{:.2}", c * 100.0))
            .unwrap_or_else(|| "0.00".to_string());
        let cost = metrics
            .and_then(|m| m.cost_micros)
            .map(|c| format!("${:.2}", c as f64 / 1_000_000.0))
            .unwrap_or_else(|| "$0.00".to_string());
        let conversions = metrics.and_then(|m| m.conversions).unwrap_or(0.0);
        let avg_cpc = metrics
            .and_then(|m| m.average_cpc)
            .map(|c| format!("${:.2}", c as f64 / 1_000_000.0))
            .unwrap_or_else(|| "$0.00".to_string());

        println!(
            "{:<12} {:<35} {:<10} {:>10} {:>8} {:>7} {:>12} {:>12.1} {:>10}",
            id, name_truncated, ctype_truncated, impressions, clicks, ctr, cost, conversions, avg_cpc
        );
    }

    println!("{}", "-".repeat(110));
    println!("Total: {} enabled campaign(s)", results.len());

    Ok(())
}
