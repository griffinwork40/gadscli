use std::collections::HashMap;

use crate::cli::Cli;
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn run(client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;
    let query = "SELECT ad_group_criterion.keyword.text, \
                 ad_group_criterion.keyword.match_type, \
                 ad_group.name, ad_group.id, \
                 campaign.name, campaign.id \
                 FROM keyword_view \
                 WHERE ad_group_criterion.status = 'ENABLED'";

    let results = client.search_all(&cid, query, cli.page_size).await?;

    if results.is_empty() {
        println!("No enabled keywords found.");
        return Ok(());
    }

    // Group by (keyword_text, match_type) -> Vec<(campaign_name, ad_group_name)>
    let mut groups: HashMap<(String, String), Vec<(String, String)>> = HashMap::new();

    for row in &results {
        let criterion = row.ad_group_criterion.as_ref();

        let keyword_text = criterion
            .and_then(|c| c.keyword.as_ref())
            .and_then(|k| k.text.as_deref())
            .unwrap_or("")
            .to_string();

        let match_type = criterion
            .and_then(|c| c.keyword.as_ref())
            .and_then(|k| k.match_type.as_ref())
            .map(|m| m.to_string())
            .unwrap_or_default();

        if keyword_text.is_empty() {
            continue;
        }

        let campaign_name = row
            .campaign
            .as_ref()
            .and_then(|c| c.name.as_deref())
            .unwrap_or("-")
            .to_string();

        let ad_group_name = row
            .ad_group
            .as_ref()
            .and_then(|ag| ag.name.as_deref())
            .unwrap_or("-")
            .to_string();

        groups
            .entry((keyword_text, match_type))
            .or_default()
            .push((campaign_name, ad_group_name));
    }

    let mut duplicates: Vec<((String, String), Vec<(String, String)>)> = groups
        .into_iter()
        .filter(|(_, entries)| entries.len() > 1)
        .collect();

    if duplicates.is_empty() {
        println!("No duplicate keywords found across ad groups.");
        println!("Total keywords checked: {}", results.len());
        return Ok(());
    }

    duplicates.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    println!("Duplicate Keywords Report");
    println!("{}", "=".repeat(80));
    println!("Total keywords checked: {}", results.len());
    println!("Duplicate keyword groups found: {}", duplicates.len());
    println!();

    for ((keyword, match_type), entries) in &duplicates {
        println!(
            "Keyword: \"{}\" [{}] -- {} occurrences",
            keyword,
            match_type,
            entries.len()
        );
        println!("{}", "-".repeat(70));
        for (campaign, ad_group) in entries {
            println!("  Campaign: {}  |  Ad Group: {}", campaign, ad_group);
        }
        println!();
    }

    println!("{}", "=".repeat(80));
    println!(
        "Recommendation: review {} duplicate keyword group(s) to avoid internal competition.",
        duplicates.len()
    );

    Ok(())
}
