use crate::cli::{Cli, DeviceCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

pub async fn handle(command: &DeviceCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    match command {
        DeviceCommands::List { campaign_id } => handle_list(client, cli, campaign_id).await,
        DeviceCommands::Set { campaign_id, device, bid_modifier } => {
            handle_set(client, cli, campaign_id, device, *bid_modifier).await
        }
        DeviceCommands::Remove { id } => handle_remove(client, cli, id).await,
    }
}

async fn handle_list(client: &GoogleAdsClient, cli: &Cli, campaign_id: &str) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    let query = format!(
        "SELECT campaign_criterion.resource_name, campaign_criterion.criterion_id, \
         campaign_criterion.type, campaign_criterion.device.type, \
         campaign_criterion.bid_modifier, campaign.id, campaign.name \
         FROM campaign_criterion \
         WHERE campaign_criterion.type = 'DEVICE' \
         AND campaign.id = {}",
        campaign_id
    );

    let rows = client.search_all(&customer_id, &query, cli.page_size).await?;

    if rows.is_empty() {
        println!("No device bid adjustments found for campaign {}.", campaign_id);
        return Ok(());
    }

    println!(
        "{:<14} {:<14} {:<14} {:<14}",
        "Criterion ID", "Device", "Bid Modifier", "Resource"
    );
    println!("{}", "-".repeat(56));

    for row in &rows {
        let cc = row.campaign_criterion.as_ref();
        let cid = cc.and_then(|c| c.criterion_id.as_deref()).unwrap_or("-");
        let device = cc
            .and_then(|c| c.device.as_ref())
            .and_then(|d| d.device_type.as_deref())
            .unwrap_or("-");
        let bid_mod = cc
            .and_then(|c| c.bid_modifier)
            .map(|b| format!("{:.2}", b))
            .unwrap_or_else(|| "-".to_string());
        let resource = cc.map(|c| c.resource_name.as_str()).unwrap_or("-");

        println!("{:<14} {:<14} {:<14} {:<14}", cid, device, bid_mod, resource);
    }

    println!("\nTotal: {} device adjustment(s)", rows.len());
    Ok(())
}

async fn handle_set(
    client: &GoogleAdsClient,
    cli: &Cli,
    campaign_id: &str,
    device: &str,
    bid_modifier: f64,
) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;
    let campaign_resource = format!("customers/{}/campaigns/{}", customer_id, campaign_id);

    // Check if a criterion already exists for this device type
    let query = format!(
        "SELECT campaign_criterion.resource_name, campaign_criterion.device.type \
         FROM campaign_criterion \
         WHERE campaign_criterion.type = 'DEVICE' \
         AND campaign_criterion.device.type = '{}' \
         AND campaign.id = {}",
        device, campaign_id
    );

    let existing = client.search_all(&customer_id, &query, cli.page_size).await?;

    let (operations, action) = if let Some(row) = existing.first() {
        let resource_name = row
            .campaign_criterion
            .as_ref()
            .map(|c| c.resource_name.clone())
            .unwrap_or_default();

        let payload = serde_json::json!({
            "resourceName": resource_name,
            "bidModifier": bid_modifier
        });

        let ops: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
            create: None,
            update: Some(payload),
            remove: None,
            update_mask: Some("bid_modifier".to_string()),
        }];
        (ops, "Updated")
    } else {
        let payload = serde_json::json!({
            "campaign": campaign_resource,
            "device": { "type": device },
            "bidModifier": bid_modifier
        });

        let ops: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
            create: Some(payload),
            update: None,
            remove: None,
            update_mask: None,
        }];
        (ops, "Created")
    };

    if cli.dry_run {
        println!(
            "[dry-run] Would {} device bid adjustment: {} = {:.2}",
            action.to_lowercase(),
            device,
            bid_modifier
        );
        return Ok(());
    }

    let response = client
        .mutate(&customer_id, "campaignCriteria", operations, false, false)
        .await?;

    match response.results.first() {
        Some(result) => println!("{} device bid adjustment: {}", action, result.resource_name),
        None => println!("{} device bid adjustment.", action),
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
        println!("[dry-run] Would remove device criterion: {}", resource_name);
        return Ok(());
    }

    client
        .mutate(&customer_id, "campaignCriteria", operations, false, false)
        .await?;

    println!("Removed device criterion: {}", resource_name);
    Ok(())
}
