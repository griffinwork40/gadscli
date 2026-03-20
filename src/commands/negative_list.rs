use crate::cli::{Cli, NegativeListCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn handle(command: &NegativeListCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        NegativeListCommands::List => {
            let query = "SELECT shared_set.id, shared_set.name, shared_set.type, shared_set.status, \
                         shared_set.member_count FROM shared_set WHERE shared_set.type = 'NEGATIVE_KEYWORDS'";
            let rows = client.search_all(&customer_id, query, Some(1000)).await?;
            if rows.is_empty() {
                println!("No shared negative keyword lists found.");
                return Ok(());
            }
            println!("{:<15} {:<40} {:<20} {:<10} {:<10}", "ID", "Name", "Type", "Status", "Members");
            println!("{}", "-".repeat(95));
            for row in &rows {
                if let Some(ss) = &row.shared_set {
                    println!(
                        "{:<15} {:<40} {:<20} {:<10} {:<10}",
                        ss.id.as_deref().unwrap_or("-"),
                        ss.name.as_deref().unwrap_or("-"),
                        ss.set_type.as_deref().unwrap_or("-"),
                        ss.status.as_deref().unwrap_or("-"),
                        ss.member_count.map(|c| c.to_string()).unwrap_or_else(|| "-".to_string()),
                    );
                }
            }
        }

        NegativeListCommands::Create { name } => {
            if cli.dry_run {
                println!("[dry-run] Would create shared negative keyword list '{}'.", name);
                return Ok(());
            }
            let url = format!("{}/customers/{}/sharedSets:mutate", client.base_url(), customer_id);
            let request_body = serde_json::json!({
                "operations": [{ "create": { "name": name, "type": "NEGATIVE_KEYWORDS" } }]
            });
            let response = client.http().execute(reqwest::Method::POST, &url, Some(request_body)).await?;
            let resource_name = response.get("results").and_then(|r| r.as_array())
                .and_then(|a| a.first()).and_then(|r| r.get("resourceName"))
                .and_then(|v| v.as_str()).unwrap_or("unknown");
            println!("Created shared set: {}", resource_name);
        }

        NegativeListCommands::Remove { id } => {
            if cli.dry_run {
                println!("[dry-run] Would remove shared set {}.", id);
                return Ok(());
            }
            let resource_name = format!("customers/{}/sharedSets/{}", customer_id, id);
            let url = format!("{}/customers/{}/sharedSets:mutate", client.base_url(), customer_id);
            let request_body = serde_json::json!({
                "operations": [{ "remove": resource_name }]
            });
            client.http().execute(reqwest::Method::POST, &url, Some(request_body)).await?;
            println!("Removed shared set: {}", resource_name);
        }

        NegativeListCommands::AddKeyword { list_id, text, match_type } => {
            if cli.dry_run {
                println!("[dry-run] Would add keyword '{}' ({}) to shared set {}.", text, match_type, list_id);
                return Ok(());
            }
            let url = format!("{}/customers/{}/sharedCriteria:mutate", client.base_url(), customer_id);
            let request_body = serde_json::json!({
                "operations": [{
                    "create": {
                        "sharedSet": format!("customers/{}/sharedSets/{}", customer_id, list_id),
                        "keyword": { "text": text, "matchType": match_type.to_uppercase() }
                    }
                }]
            });
            let response = client.http().execute(reqwest::Method::POST, &url, Some(request_body)).await?;
            let resource_name = response.get("results").and_then(|r| r.as_array())
                .and_then(|a| a.first()).and_then(|r| r.get("resourceName"))
                .and_then(|v| v.as_str()).unwrap_or("unknown");
            println!("Added keyword to shared set: {}", resource_name);
        }

        NegativeListCommands::RemoveKeyword { id } => {
            if cli.dry_run {
                println!("[dry-run] Would remove shared criterion {}.", id);
                return Ok(());
            }
            let resource_name = format!("customers/{}/sharedCriteria/{}", customer_id, id);
            let url = format!("{}/customers/{}/sharedCriteria:mutate", client.base_url(), customer_id);
            let request_body = serde_json::json!({
                "operations": [{ "remove": resource_name }]
            });
            client.http().execute(reqwest::Method::POST, &url, Some(request_body)).await?;
            println!("Removed shared criterion: {}", resource_name);
        }

        NegativeListCommands::ListKeywords { list_id } => {
            let query = format!(
                "SELECT shared_criterion.keyword.text, shared_criterion.keyword.match_type, \
                 shared_criterion.criterion_id FROM shared_criterion \
                 WHERE shared_criterion.shared_set = 'customers/{}/sharedSets/{}'",
                customer_id, list_id
            );
            let rows = client.search_all(&customer_id, &query, Some(1000)).await?;
            if rows.is_empty() {
                println!("No keywords found in shared set {}.", list_id);
                return Ok(());
            }
            println!("{:<40} {:<20} {:<15}", "Text", "Match Type", "Criterion ID");
            println!("{}", "-".repeat(75));
            for row in &rows {
                if let Some(sc) = &row.shared_criterion {
                    let (text, mt) = sc.keyword.as_ref()
                        .map(|k| (
                            k.text.as_deref().unwrap_or("-"),
                            k.match_type.as_ref().map(|m| m.to_string()).unwrap_or_else(|| "-".to_string()),
                        ))
                        .unwrap_or(("-", "-".to_string()));
                    println!(
                        "{:<40} {:<20} {:<15}",
                        text,
                        mt,
                        sc.criterion_id.as_deref().unwrap_or("-"),
                    );
                }
            }
        }

        NegativeListCommands::Apply { list_id, campaign_id } => {
            if cli.dry_run {
                println!("[dry-run] Would apply shared set {} to campaign {}.", list_id, campaign_id);
                return Ok(());
            }
            let url = format!("{}/customers/{}/campaignSharedSets:mutate", client.base_url(), customer_id);
            let request_body = serde_json::json!({
                "operations": [{
                    "create": {
                        "campaign": format!("customers/{}/campaigns/{}", customer_id, campaign_id),
                        "sharedSet": format!("customers/{}/sharedSets/{}", customer_id, list_id),
                    }
                }]
            });
            let response = client.http().execute(reqwest::Method::POST, &url, Some(request_body)).await?;
            let resource_name = response.get("results").and_then(|r| r.as_array())
                .and_then(|a| a.first()).and_then(|r| r.get("resourceName"))
                .and_then(|v| v.as_str()).unwrap_or("unknown");
            println!("Applied shared set to campaign: {}", resource_name);
        }

        NegativeListCommands::Unapply { id } => {
            if cli.dry_run {
                println!("[dry-run] Would remove campaign shared set {}.", id);
                return Ok(());
            }
            let resource_name = format!("customers/{}/campaignSharedSets/{}", customer_id, id);
            let url = format!("{}/customers/{}/campaignSharedSets:mutate", client.base_url(), customer_id);
            let request_body = serde_json::json!({
                "operations": [{ "remove": resource_name }]
            });
            client.http().execute(reqwest::Method::POST, &url, Some(request_body)).await?;
            println!("Removed campaign shared set: {}", resource_name);
        }

        NegativeListCommands::ListCampaigns { list_id } => {
            let query = format!(
                "SELECT campaign.name, campaign.id, campaign_shared_set.status \
                 FROM campaign_shared_set \
                 WHERE campaign_shared_set.shared_set = 'customers/{}/sharedSets/{}'",
                customer_id, list_id
            );
            let rows = client.search_all(&customer_id, &query, Some(1000)).await?;
            if rows.is_empty() {
                println!("No campaigns using shared set {}.", list_id);
                return Ok(());
            }
            println!("{:<40} {:<20} {:<10}", "Campaign Name", "Campaign ID", "Status");
            println!("{}", "-".repeat(70));
            for row in &rows {
                let name = row.campaign.as_ref().and_then(|c| c.name.as_deref()).unwrap_or("-");
                let id = row.campaign.as_ref().and_then(|c| c.id.clone()).unwrap_or_default();
                let status = row.campaign_shared_set.as_ref().and_then(|cs| cs.status.as_deref()).unwrap_or("-");
                println!("{:<40} {:<20} {:<10}", name, id, status);
            }
        }
    }

    Ok(())
}
