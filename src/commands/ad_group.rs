use crate::cli::{AdGroupCommands, Cli};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;
use crate::types::resources::AdGroup;

pub async fn handle(command: &AdGroupCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        AdGroupCommands::List {
            campaign_id,
            status,
        } => {
            let mut query = String::from(
                "SELECT ad_group.id, ad_group.name, ad_group.status, ad_group.campaign, \
                 ad_group.cpc_bid_micros, metrics.impressions, metrics.clicks, \
                 metrics.cost_micros \
                 FROM ad_group \
                 WHERE ad_group.status != 'REMOVED'",
            );

            if let Some(cid_filter) = campaign_id {
                query.push_str(&format!(
                    " AND ad_group.campaign = 'customers/{}/campaigns/{}'",
                    cid, cid_filter
                ));
            }

            if let Some(s) = status {
                query.push_str(&format!(" AND ad_group.status = '{}'", s));
            }

            let rows = client.search_all(&cid, &query, Some(1000)).await?;

            if rows.is_empty() {
                println!("No ad groups found.");
                return Ok(());
            }

            println!(
                "{:<12} {:<40} {:<10} {:<14} {:<14} {:<10} {:<10}",
                "ID", "Name", "Status", "CPC Bid (μ)", "Campaign", "Impr.", "Clicks"
            );
            println!("{}", "-".repeat(115));

            for row in &rows {
                if let Some(ag) = &row.ad_group {
                    let id = ag.id.clone().unwrap_or_default();
                    let name = ag.name.as_deref().unwrap_or("-");
                    let status = ag
                        .status
                        .as_ref()
                        .map(|s| s.to_string())
                        .unwrap_or_default();
                    let cpc = ag
                        .cpc_bid_micros
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "-".into());
                    let campaign = ag.campaign.as_deref().unwrap_or("-");
                    let impressions = row
                        .metrics
                        .as_ref()
                        .and_then(|m| m.impressions)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "0".into());
                    let clicks = row
                        .metrics
                        .as_ref()
                        .and_then(|m| m.clicks)
                        .map(|v| v.to_string())
                        .unwrap_or_else(|| "0".into());

                    println!(
                        "{:<12} {:<40} {:<10} {:<14} {:<14} {:<10} {:<10}",
                        id,
                        if name.len() > 38 {
                            &name[..38]
                        } else {
                            name
                        },
                        status,
                        cpc,
                        if campaign.len() > 12 {
                            &campaign[campaign.len() - 12..]
                        } else {
                            campaign
                        },
                        impressions,
                        clicks
                    );
                }
            }
        }

        AdGroupCommands::Get { id } => {
            let query = format!(
                "SELECT ad_group.id, ad_group.name, ad_group.status, ad_group.campaign, \
                 ad_group.cpc_bid_micros, metrics.impressions, metrics.clicks, \
                 metrics.cost_micros \
                 FROM ad_group \
                 WHERE ad_group.id = {}",
                id
            );

            let rows = client.search_all(&cid, &query, Some(1)).await?;

            match rows.first().and_then(|r| r.ad_group.as_ref()) {
                None => println!("Ad group '{}' not found.", id),
                Some(ag) => {
                    println!("Ad Group Details");
                    println!("{}", "-".repeat(40));
                    println!("ID:           {}", ag.id.clone().unwrap_or_default());
                    println!("Name:         {}", ag.name.as_deref().unwrap_or("-"));
                    println!(
                        "Status:       {}",
                        ag.status.as_ref().map(|s| s.to_string()).unwrap_or_default()
                    );
                    println!("Campaign:     {}", ag.campaign.as_deref().unwrap_or("-"));
                    println!(
                        "CPC Bid (μ):  {}",
                        ag.cpc_bid_micros.map(|v| v.to_string()).unwrap_or_else(|| "-".into())
                    );
                    if let Some(metrics) = rows.first().and_then(|r| r.metrics.as_ref()) {
                        println!(
                            "Impressions:  {}",
                            metrics.impressions.unwrap_or(0)
                        );
                        println!("Clicks:       {}", metrics.clicks.unwrap_or(0));
                        println!(
                            "Cost (μ):     {}",
                            metrics.cost_micros.unwrap_or(0)
                        );
                    }
                }
            }
        }

        AdGroupCommands::Create {
            campaign_id,
            name,
            cpc_bid_micros,
        } => {
            let mut resource = AdGroup {
                resource_name: String::new(),
                name: Some(name.clone()),
                campaign: Some(format!("customers/{}/campaigns/{}", cid, campaign_id)),
                status: Some(crate::types::common::AdGroupStatus::Enabled),
                cpc_bid_micros: *cpc_bid_micros,
                ..Default::default()
            };
            // resource_name must be empty for creates; clear it
            resource.resource_name = String::new();

            let ops: Vec<MutateOperation<AdGroup>> = vec![MutateOperation {
                create: Some(resource),
                update: None,
                remove: None,
                update_mask: None,
            }];

            if cli.dry_run {
                println!("[dry-run] Would create ad group '{}' in campaign {}.", name, campaign_id);
                return Ok(());
            }

            let resp = client.mutate(&cid, "adGroups", ops, false, false).await?;
            if let Some(result) = resp.results.first() {
                println!("Created ad group: {}", result.resource_name);
            } else {
                println!("Ad group created.");
            }
        }

        AdGroupCommands::Update {
            id,
            name,
            status,
            cpc_bid_micros,
        } => {
            let resource_name = format!("customers/{}/adGroups/{}", cid, id);

            let parsed_status = match status.as_deref() {
                Some("ENABLED") => Some(crate::types::common::AdGroupStatus::Enabled),
                Some("PAUSED") => Some(crate::types::common::AdGroupStatus::Paused),
                Some("REMOVED") => Some(crate::types::common::AdGroupStatus::Removed),
                Some(other) => {
                    return Err(crate::error::GadsError::Validation(format!(
                        "Unknown status: {}",
                        other
                    )))
                }
                None => None,
            };

            let mut mask_fields: Vec<&str> = Vec::new();
            if name.is_some() {
                mask_fields.push("name");
            }
            if parsed_status.is_some() {
                mask_fields.push("status");
            }
            if cpc_bid_micros.is_some() {
                mask_fields.push("cpc_bid_micros");
            }

            if mask_fields.is_empty() {
                println!("Nothing to update.");
                return Ok(());
            }

            let resource = AdGroup {
                resource_name: resource_name.clone(),
                name: name.clone(),
                status: parsed_status,
                cpc_bid_micros: *cpc_bid_micros,
                ..Default::default()
            };

            let ops: Vec<MutateOperation<AdGroup>> = vec![MutateOperation {
                create: None,
                update: Some(resource),
                remove: None,
                update_mask: Some(mask_fields.join(",")),
            }];

            if cli.dry_run {
                println!("[dry-run] Would update ad group {}.", id);
                return Ok(());
            }

            let resp = client.mutate(&cid, "adGroups", ops, false, false).await?;
            if let Some(result) = resp.results.first() {
                println!("Updated ad group: {}", result.resource_name);
            } else {
                println!("Ad group updated.");
            }
        }

        AdGroupCommands::Pause { id } => {
            let resource_name = format!("customers/{}/adGroups/{}", cid, id);

            let resource = AdGroup {
                resource_name: resource_name.clone(),
                status: Some(crate::types::common::AdGroupStatus::Paused),
                ..Default::default()
            };

            let ops: Vec<MutateOperation<AdGroup>> = vec![MutateOperation {
                create: None,
                update: Some(resource),
                remove: None,
                update_mask: Some("status".into()),
            }];

            if cli.dry_run {
                println!("[dry-run] Would pause ad group {}.", id);
                return Ok(());
            }

            client.mutate(&cid, "adGroups", ops, false, false).await?;
            println!("Ad group {} paused.", id);
        }

        AdGroupCommands::Enable { id } => {
            let resource_name = format!("customers/{}/adGroups/{}", cid, id);

            let resource = AdGroup {
                resource_name: resource_name.clone(),
                status: Some(crate::types::common::AdGroupStatus::Enabled),
                ..Default::default()
            };

            let ops: Vec<MutateOperation<AdGroup>> = vec![MutateOperation {
                create: None,
                update: Some(resource),
                remove: None,
                update_mask: Some("status".into()),
            }];

            if cli.dry_run {
                println!("[dry-run] Would enable ad group {}.", id);
                return Ok(());
            }

            client.mutate(&cid, "adGroups", ops, false, false).await?;
            println!("Ad group {} enabled.", id);
        }

        AdGroupCommands::Remove { id } => {
            let resource_name = format!("customers/{}/adGroups/{}", cid, id);

            let ops: Vec<MutateOperation<AdGroup>> = vec![MutateOperation {
                create: None,
                update: None,
                remove: Some(resource_name.clone()),
                update_mask: None,
            }];

            if cli.dry_run {
                println!("[dry-run] Would remove ad group {}.", id);
                return Ok(());
            }

            client.mutate(&cid, "adGroups", ops, false, false).await?;
            println!("Ad group {} removed.", id);
        }
    }

    Ok(())
}
