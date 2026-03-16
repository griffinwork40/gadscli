use crate::cli::Cli;
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn run(client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;
    let query = "SELECT search_term_view.search_term, \
                 search_term_view.status, \
                 campaign.name, ad_group.name, \
                 metrics.impressions, metrics.clicks, \
                 metrics.cost_micros, metrics.conversions \
                 FROM search_term_view \
                 ORDER BY metrics.impressions DESC \
                 LIMIT 100";

    let results = client.search_all(&cid, query, cli.page_size).await?;

    if results.is_empty() {
        println!("No search term data found.");
        return Ok(());
    }

    println!("Search Terms Report (Top 100 by Impressions)");
    println!("{}", "=".repeat(110));
    println!(
        "{:<35} {:<25} {:<20} {:>8} {:>8} {:>12} {:>10}",
        "Search Term", "Campaign", "Ad Group", "Impr.", "Clicks", "Cost", "Conv."
    );
    println!("{}", "-".repeat(110));

    for row in &results {
        // search_term_view data is returned in segments as JSON
        let search_term = row
            .segments
            .as_ref()
            .and_then(|s| s.get("searchTermView"))
            .and_then(|stv| stv.get("searchTerm"))
            .and_then(|t| t.as_str())
            .unwrap_or("-");

        let st_truncated = if search_term.len() > 33 {
            format!("{}.", &search_term[..33])
        } else {
            search_term.to_string()
        };

        let campaign_name = row
            .campaign
            .as_ref()
            .and_then(|c| c.name.as_deref())
            .unwrap_or("-");
        let campaign_truncated = if campaign_name.len() > 23 {
            format!("{}.", &campaign_name[..23])
        } else {
            campaign_name.to_string()
        };

        let ad_group_name = row
            .ad_group
            .as_ref()
            .and_then(|ag| ag.name.as_deref())
            .unwrap_or("-");
        let ag_truncated = if ad_group_name.len() > 18 {
            format!("{}.", &ad_group_name[..18])
        } else {
            ad_group_name.to_string()
        };

        let impressions = row.metrics.as_ref().and_then(|m| m.impressions).unwrap_or(0);
        let clicks = row.metrics.as_ref().and_then(|m| m.clicks).unwrap_or(0);
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
            "{:<35} {:<25} {:<20} {:>8} {:>8} {:>12} {:>10.1}",
            st_truncated, campaign_truncated, ag_truncated, impressions, clicks, cost, conversions
        );
    }

    println!("{}", "-".repeat(110));
    println!("{} search term(s) shown", results.len());

    Ok(())
}
