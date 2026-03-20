use crate::cli::{Cli, LocationCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

pub async fn handle(command: &LocationCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    match command {
        LocationCommands::List { campaign_id } => handle_list(client, cli, campaign_id).await,
        LocationCommands::Add { campaign_id, geo_id, negative, bid_modifier } => {
            handle_add(client, cli, campaign_id, geo_id, *negative, *bid_modifier).await
        }
        LocationCommands::Remove { id } => handle_remove(client, cli, id).await,
        LocationCommands::Search { query } => handle_search(client, cli, query).await,
    }
}

async fn handle_list(client: &GoogleAdsClient, cli: &Cli, campaign_id: &str) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    let query = format!(
        "SELECT campaign_criterion.resource_name, campaign_criterion.criterion_id, \
         campaign_criterion.location.geo_target_constant, \
         campaign_criterion.negative, campaign_criterion.bid_modifier, \
         campaign.id, campaign.name \
         FROM campaign_criterion \
         WHERE campaign_criterion.type = 'LOCATION' \
         AND campaign.id = {}",
        campaign_id
    );

    let rows = client.search_all(&customer_id, &query, cli.page_size).await?;

    if rows.is_empty() {
        println!("No location targets found for campaign {}.", campaign_id);
        return Ok(());
    }

    println!(
        "{:<14} {:<30} {:<10} {:<14} {:<14}",
        "Criterion ID", "Geo Target", "Negative", "Bid Modifier", "Resource"
    );
    println!("{}", "-".repeat(82));

    for row in &rows {
        let cc = row.campaign_criterion.as_ref();
        let cid = cc.and_then(|c| c.criterion_id.as_deref()).unwrap_or("-");
        let geo = cc
            .and_then(|c| c.location.as_ref())
            .and_then(|l| l.geo_target_constant.as_deref())
            .unwrap_or("-");
        let negative = cc
            .and_then(|c| c.negative)
            .map(|n| if n { "Yes" } else { "No" })
            .unwrap_or("-");
        let bid_mod = cc
            .and_then(|c| c.bid_modifier)
            .map(|b| format!("{:.2}", b))
            .unwrap_or_else(|| "-".to_string());
        let resource = cc.map(|c| c.resource_name.as_str()).unwrap_or("-");

        println!("{:<14} {:<30} {:<10} {:<14} {:<14}", cid, geo, negative, bid_mod, resource);
    }

    println!("\nTotal: {} location target(s)", rows.len());
    Ok(())
}

async fn handle_add(
    client: &GoogleAdsClient,
    cli: &Cli,
    campaign_id: &str,
    geo_id: &str,
    negative: bool,
    bid_modifier: Option<f64>,
) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;
    let campaign_resource = format!("customers/{}/campaigns/{}", customer_id, campaign_id);
    let geo_constant = format!("geoTargetConstants/{}", geo_id);

    let mut payload = serde_json::json!({
        "campaign": campaign_resource,
        "location": { "geoTargetConstant": geo_constant },
        "negative": negative
    });

    if let Some(bm) = bid_modifier {
        payload["bidModifier"] = serde_json::json!(bm);
    }

    let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
        create: Some(payload),
        update: None,
        remove: None,
        update_mask: None,
    }];

    if cli.dry_run {
        println!(
            "[dry-run] Would add location target: {} (negative: {}, bid modifier: {})",
            geo_constant,
            negative,
            bid_modifier.map(|b| format!("{:.2}", b)).unwrap_or_else(|| "none".to_string())
        );
        return Ok(());
    }

    let response = client
        .mutate(&customer_id, "campaignCriteria", operations, false, false)
        .await?;

    match response.results.first() {
        Some(result) => println!("Created location target: {}", result.resource_name),
        None => println!("Location target created."),
    }

    Ok(())
}

async fn handle_remove(client: &GoogleAdsClient, cli: &Cli, id: &str) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    let resource_name = if id.contains('/') {
        id.to_string()
    } else {
        format!("customers/{}/campaignCriteria/{}", customer_id, id)
    };

    let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
        create: None,
        update: None,
        remove: Some(resource_name.clone()),
        update_mask: None,
    }];

    if cli.dry_run {
        println!("[dry-run] Would remove location target: {}", resource_name);
        return Ok(());
    }

    client
        .mutate(&customer_id, "campaignCriteria", operations, false, false)
        .await?;

    println!("Removed location target: {}", resource_name);
    Ok(())
}

async fn handle_search(client: &GoogleAdsClient, cli: &Cli, search_query: &str) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    let query = format!(
        "SELECT geo_target_constant.resource_name, geo_target_constant.id, \
         geo_target_constant.name, geo_target_constant.canonical_name, \
         geo_target_constant.country_code, geo_target_constant.target_type, \
         geo_target_constant.status \
         FROM geo_target_constant \
         WHERE geo_target_constant.name LIKE '%{}%'",
        search_query
    );

    let rows = client.search_all(&customer_id, &query, cli.page_size).await?;

    if rows.is_empty() {
        println!("No geo target constants found matching \"{}\".", search_query);
        return Ok(());
    }

    println!(
        "{:<10} {:<30} {:<40} {:<8} {:<12} {:<10}",
        "ID", "Name", "Canonical Name", "Country", "Type", "Status"
    );
    println!("{}", "-".repeat(110));

    for row in &rows {
        let gtc = row.geo_target_constant.as_ref();
        let id = gtc.and_then(|g| g.id.as_deref()).unwrap_or("-");
        let name = gtc.and_then(|g| g.name.as_deref()).unwrap_or("-");
        let canonical = gtc.and_then(|g| g.canonical_name.as_deref()).unwrap_or("-");
        let country = gtc.and_then(|g| g.country_code.as_deref()).unwrap_or("-");
        let target_type = gtc.and_then(|g| g.target_type.as_deref()).unwrap_or("-");
        let status = gtc.and_then(|g| g.status.as_deref()).unwrap_or("-");

        println!("{:<10} {:<30} {:<40} {:<8} {:<12} {:<10}", id, name, canonical, country, target_type, status);
    }

    println!("\nTotal: {} result(s)", rows.len());
    Ok(())
}
