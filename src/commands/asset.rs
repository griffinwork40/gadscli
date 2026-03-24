use crate::cli::{AssetCommands, Cli};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::resources::Asset;

pub async fn handle(command: &AssetCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        AssetCommands::List { asset_type } => {
            let query = if let Some(t) = asset_type {
                format!(
                    "SELECT asset.id, asset.name, asset.type FROM asset WHERE asset.type = '{}'",
                    t
                )
            } else {
                "SELECT asset.id, asset.name, asset.type FROM asset".to_string()
            };

            let rows = client.search_all(&customer_id, &query, Some(1000)).await?;
            if rows.is_empty() {
                println!("No assets found.");
                return Ok(());
            }
            println!("{:<20} {:<40} {:<20}", "ID", "Name", "Type");
            println!("{}", "-".repeat(80));
            for row in &rows {
                if let Some(asset) = &row.asset {
                    println!(
                        "{:<20} {:<40} {:<20}",
                        asset.id.clone().unwrap_or_default(),
                        asset.name.as_deref().unwrap_or("-"),
                        asset
                            .asset_type
                            .as_ref()
                            .map(|t| t.to_string())
                            .unwrap_or_default(),
                    );
                }
            }
        }

        AssetCommands::Get { id } => {
            let query = format!(
                "SELECT asset.id, asset.name, asset.type FROM asset WHERE asset.id = {}",
                id
            );
            let rows = client.search_all(&customer_id, &query, Some(1)).await?;
            match rows.first().and_then(|r| r.asset.as_ref()) {
                None => println!("Asset {} not found.", id),
                Some(asset) => {
                    println!("ID:   {}", asset.id.clone().unwrap_or_default());
                    println!("Name: {}", asset.name.as_deref().unwrap_or("-"));
                    println!(
                        "Type: {}",
                        asset
                            .asset_type
                            .as_ref()
                            .map(|t| t.to_string())
                            .unwrap_or_default()
                    );
                    println!(
                        "Resource: {}",
                        asset.resource_name
                    );
                }
            }
        }

        AssetCommands::Create {
            name,
            asset_type,
            text_content,
        } => {
            let asset = Asset {
                resource_name: String::new(),
                id: None,
                name: Some(name.clone()),
                asset_type: None,
            };

            // Build the operation body as JSON to include type and optional text asset
            let mut body = serde_json::json!({
                "name": name,
                "type": asset_type,
            });

            if let Some(text) = text_content {
                body["textAsset"] = serde_json::json!({ "text": text });
            }

            // We need to send a raw mutate using the http client directly for flexibility
            let url = format!(
                "{}/customers/{}/assets:mutate",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{ "create": body }]
            });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            let resource_name = response
                .get("results")
                .and_then(|r| r.as_array())
                .and_then(|a| a.first())
                .and_then(|r| r.get("resourceName"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            println!("Created asset: {}", resource_name);
            let _ = asset; // suppress unused warning
        }

        AssetCommands::Link {
            campaign_id,
            asset_id,
            field_type,
        } => {
            if cli.dry_run {
                println!(
                    "[dry-run] Would link asset {} to campaign {} with field type {}.",
                    asset_id, campaign_id, field_type
                );
                return Ok(());
            }

            let url = format!(
                "{}/customers/{}/campaignAssets:mutate",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{
                    "create": {
                        "campaign": format!("customers/{}/campaigns/{}", customer_id, campaign_id),
                        "asset": format!("customers/{}/assets/{}", customer_id, asset_id),
                        "fieldType": field_type.to_uppercase(),
                    }
                }]
            });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            let resource_name = response
                .get("results")
                .and_then(|r| r.as_array())
                .and_then(|a| a.first())
                .and_then(|r| r.get("resourceName"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            println!("Linked asset: {}", resource_name);
        }

        AssetCommands::Unlink { id } => {
            if cli.dry_run {
                println!("[dry-run] Would unlink campaign asset {}.", id);
                return Ok(());
            }

            let resource_name = format!("customers/{}/campaignAssets/{}", customer_id, id);
            let url = format!(
                "{}/customers/{}/campaignAssets:mutate",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{ "remove": resource_name }]
            });

            client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            println!("Unlinked campaign asset: {}", resource_name);
        }

        AssetCommands::ListLinked { campaign_id } => {
            let query = format!(
                "SELECT campaign_asset.resource_name, campaign_asset.asset, campaign_asset.field_type, campaign_asset.status \
                 FROM campaign_asset \
                 WHERE campaign_asset.campaign = 'customers/{}/campaigns/{}'",
                customer_id, campaign_id
            );

            let rows = client.search_all(&customer_id, &query, Some(1000)).await?;
            if rows.is_empty() {
                println!("No linked assets found for campaign {}.", campaign_id);
                return Ok(());
            }
            println!("{:<50} {:<50} {:<20} {:<10}", "Resource", "Asset", "Field Type", "Status");
            println!("{}", "-".repeat(130));
            for row in &rows {
                if let Some(ca) = &row.campaign_asset {
                    println!(
                        "{:<50} {:<50} {:<20} {:<10}",
                        ca.resource_name,
                        ca.asset.as_deref().unwrap_or("-"),
                        ca.field_type.as_deref().unwrap_or("-"),
                        ca.status.as_deref().unwrap_or("-"),
                    );
                }
            }
        }
    }

    Ok(())
}
