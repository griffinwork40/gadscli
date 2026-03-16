use crate::cli::Cli;
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn run(client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;
    let query = "SELECT ad_group_criterion.keyword.text, \
                 ad_group_criterion.keyword.match_type, \
                 metrics.impressions, metrics.clicks, metrics.conversions, \
                 metrics.cost_micros, metrics.average_cpc \
                 FROM keyword_view \
                 WHERE ad_group_criterion.status = 'ENABLED' \
                 AND metrics.impressions > 0 \
                 ORDER BY metrics.conversions DESC \
                 LIMIT 50";

    let results = client.search_all(&cid, query, cli.page_size).await?;

    if results.is_empty() {
        println!("No keyword data found.");
        return Ok(());
    }

    println!("Top Performing Keywords");
    println!("{}", "=".repeat(100));
    println!(
        "{:<40} {:<10} {:>10} {:>8} {:>10} {:>12} {:>10}",
        "Keyword", "Match", "Impr.", "Clicks", "Conv.", "Cost", "Avg CPC"
    );
    println!("{}", "-".repeat(100));

    for row in &results {
        let criterion = row.ad_group_criterion.as_ref();
        let metrics = row.metrics.as_ref();

        let keyword_text = criterion
            .and_then(|c| c.keyword.as_ref())
            .and_then(|k| k.text.as_deref())
            .unwrap_or("-");
        let kw_truncated = if keyword_text.len() > 38 {
            format!("{}.", &keyword_text[..38])
        } else {
            keyword_text.to_string()
        };

        let match_type = criterion
            .and_then(|c| c.keyword.as_ref())
            .and_then(|k| k.match_type.as_ref())
            .map(|m| m.to_string())
            .unwrap_or_default();

        let impressions = metrics.and_then(|m| m.impressions).unwrap_or(0);
        let clicks = metrics.and_then(|m| m.clicks).unwrap_or(0);
        let conversions = metrics.and_then(|m| m.conversions).unwrap_or(0.0);
        let cost = metrics
            .and_then(|m| m.cost_micros)
            .map(|c| format!("${:.2}", c as f64 / 1_000_000.0))
            .unwrap_or_else(|| "$0.00".to_string());
        let avg_cpc = metrics
            .and_then(|m| m.average_cpc)
            .map(|c| format!("${:.2}", c as f64 / 1_000_000.0))
            .unwrap_or_else(|| "$0.00".to_string());

        println!(
            "{:<40} {:<10} {:>10} {:>8} {:>10.1} {:>12} {:>10}",
            kw_truncated, match_type, impressions, clicks, conversions, cost, avg_cpc
        );
    }

    println!("{}", "-".repeat(100));
    println!("{} keyword(s) shown", results.len());

    Ok(())
}
