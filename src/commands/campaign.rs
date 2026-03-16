use crate::cli::{Cli, CampaignCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

pub async fn handle(command: &CampaignCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        CampaignCommands::List { status, limit } => {
            let mut query = String::from(
                "SELECT campaign.id, campaign.name, campaign.status, \
                 campaign.advertising_channel_type, campaign.bidding_strategy_type, \
                 metrics.impressions, metrics.clicks, metrics.cost_micros \
                 FROM campaign \
                 WHERE campaign.status != 'REMOVED'",
            );

            if let Some(s) = status {
                query.push_str(&format!(" AND campaign.status = '{}'", s));
            }

            query.push_str(" ORDER BY campaign.name");

            if let Some(n) = limit {
                query.push_str(&format!(" LIMIT {}", n));
            }

            let rows = client.search_all(&customer_id, &query, cli.page_size).await?;

            if rows.is_empty() {
                println!("No campaigns found.");
                return Ok(());
            }

            println!(
                "{:<12} {:<40} {:<10} {:<16} {:<14} {:<10} {:<12}",
                "ID", "Name", "Status", "Type", "Bidding", "Clicks", "Cost"
            );
            println!("{}", "-".repeat(114));

            for row in &rows {
                let campaign = row.campaign.as_ref();
                let metrics = row.metrics.as_ref();

                let id = campaign
                    .and_then(|c| c.id)
                    .map(|i| i.to_string())
                    .unwrap_or_default();
                let name = campaign
                    .and_then(|c| c.name.as_deref())
                    .unwrap_or("-")
                    .to_string();
                let name_truncated = if name.len() > 38 {
                    format!("{}…", &name[..37])
                } else {
                    name
                };
                let status = campaign
                    .and_then(|c| c.status.as_ref())
                    .map(|s| s.to_string())
                    .unwrap_or_default();
                let ctype = campaign
                    .and_then(|c| c.campaign_type.as_ref())
                    .map(|t| t.to_string())
                    .unwrap_or_default();
                let bidding = campaign
                    .and_then(|c| c.bidding_strategy_type.as_ref())
                    .map(|b| b.to_string())
                    .unwrap_or_default();
                let clicks = metrics
                    .and_then(|m| m.clicks)
                    .map(|c| c.to_string())
                    .unwrap_or_else(|| "0".to_string());
                let cost = metrics
                    .and_then(|m| m.cost_micros)
                    .map(|c| format!("${:.2}", c as f64 / 1_000_000.0))
                    .unwrap_or_else(|| "$0.00".to_string());

                println!(
                    "{:<12} {:<40} {:<10} {:<16} {:<14} {:<10} {:<12}",
                    id, name_truncated, status, ctype, bidding, clicks, cost
                );
            }

            println!("\nTotal: {} campaign(s)", rows.len());
        }

        CampaignCommands::Get { id } => {
            let query = format!(
                "SELECT campaign.id, campaign.name, campaign.status, \
                 campaign.advertising_channel_type, campaign.bidding_strategy_type, \
                 campaign.start_date, campaign.end_date, campaign.campaign_budget \
                 FROM campaign \
                 WHERE campaign.id = {}",
                id
            );

            let rows = client.search_all(&customer_id, &query, cli.page_size).await?;

            match rows.first().and_then(|r| r.campaign.as_ref()) {
                None => println!("Campaign {} not found.", id),
                Some(campaign) => {
                    println!("Campaign Details");
                    println!("{}", "=".repeat(40));
                    println!("ID:           {}", campaign.id.map(|i| i.to_string()).unwrap_or_default());
                    println!("Name:         {}", campaign.name.as_deref().unwrap_or("-"));
                    println!(
                        "Status:       {}",
                        campaign.status.as_ref().map(|s| s.to_string()).unwrap_or_default()
                    );
                    println!(
                        "Type:         {}",
                        campaign.campaign_type.as_ref().map(|t| t.to_string()).unwrap_or_default()
                    );
                    println!(
                        "Bidding:      {}",
                        campaign.bidding_strategy_type.as_ref().map(|b| b.to_string()).unwrap_or_default()
                    );
                    println!("Budget:       {}", campaign.budget.as_deref().unwrap_or("-"));
                    println!("Start Date:   {}", campaign.start_date.as_deref().unwrap_or("-"));
                    println!("End Date:     {}", campaign.end_date.as_deref().unwrap_or("-"));
                    println!("Resource:     {}", campaign.resource_name);
                }
            }
        }

        CampaignCommands::Create {
            name,
            budget_id,
            campaign_type,
            bidding_strategy,
        } => {
            let budget_resource = format!("customers/{}/campaignBudgets/{}", customer_id, budget_id);

            let payload = serde_json::json!({
                "name": name,
                "advertisingChannelType": campaign_type,
                "status": "PAUSED",
                "campaignBudget": budget_resource,
                "biddingStrategyType": bidding_strategy
            });

            let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: Some(payload),
                update: None,
                remove: None,
                update_mask: None,
            }];

            if cli.dry_run {
                println!("[dry-run] Would create campaign \"{}\"", name);
                println!("  Budget resource: {}", budget_resource);
                println!("  Type: {}", campaign_type);
                println!("  Bidding: {}", bidding_strategy);
                println!("  Initial status: PAUSED");
                return Ok(());
            }

            let response = client
                .mutate(&customer_id, "campaigns", operations, false, false)
                .await?;

            match response.results.first() {
                Some(result) => println!("Created campaign: {}", result.resource_name),
                None => println!("Campaign created (no resource name returned)."),
            }
        }

        CampaignCommands::Update { id, name, status } => {
            let resource_name = format!("customers/{}/campaigns/{}", customer_id, id);

            let mut update_fields: Vec<&str> = Vec::new();
            let mut payload = serde_json::json!({ "resourceName": resource_name });

            if let Some(n) = name {
                payload["name"] = serde_json::Value::String(n.clone());
                update_fields.push("name");
            }
            if let Some(s) = status {
                payload["status"] = serde_json::Value::String(s.clone());
                update_fields.push("status");
            }

            if update_fields.is_empty() {
                println!("No fields to update.");
                return Ok(());
            }

            let update_mask = update_fields.join(",");

            let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: None,
                update: Some(payload),
                remove: None,
                update_mask: Some(update_mask),
            }];

            if cli.dry_run {
                println!("[dry-run] Would update campaign {} (fields: {})", id, update_fields.join(", "));
                return Ok(());
            }

            let response = client
                .mutate(&customer_id, "campaigns", operations, false, false)
                .await?;

            match response.results.first() {
                Some(result) => println!("Updated campaign: {}", result.resource_name),
                None => println!("Campaign updated."),
            }
        }

        CampaignCommands::Pause { id } => {
            let resource_name = format!("customers/{}/campaigns/{}", customer_id, id);
            let payload = serde_json::json!({
                "resourceName": resource_name,
                "status": "PAUSED"
            });

            let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: None,
                update: Some(payload),
                remove: None,
                update_mask: Some("status".to_string()),
            }];

            if cli.dry_run {
                println!("[dry-run] Would pause campaign {}", id);
                return Ok(());
            }

            let response = client
                .mutate(&customer_id, "campaigns", operations, false, false)
                .await?;

            match response.results.first() {
                Some(result) => println!("Paused campaign: {}", result.resource_name),
                None => println!("Campaign paused."),
            }
        }

        CampaignCommands::Enable { id } => {
            let resource_name = format!("customers/{}/campaigns/{}", customer_id, id);
            let payload = serde_json::json!({
                "resourceName": resource_name,
                "status": "ENABLED"
            });

            let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: None,
                update: Some(payload),
                remove: None,
                update_mask: Some("status".to_string()),
            }];

            if cli.dry_run {
                println!("[dry-run] Would enable campaign {}", id);
                return Ok(());
            }

            let response = client
                .mutate(&customer_id, "campaigns", operations, false, false)
                .await?;

            match response.results.first() {
                Some(result) => println!("Enabled campaign: {}", result.resource_name),
                None => println!("Campaign enabled."),
            }
        }

        CampaignCommands::Remove { id } => {
            let resource_name = format!("customers/{}/campaigns/{}", customer_id, id);

            let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: None,
                update: None,
                remove: Some(resource_name.clone()),
                update_mask: None,
            }];

            if cli.dry_run {
                println!("[dry-run] Would remove campaign {}", id);
                return Ok(());
            }

            client
                .mutate(&customer_id, "campaigns", operations, false, false)
                .await?;

            println!("Removed campaign: {}", resource_name);
        }
    }

    Ok(())
}
