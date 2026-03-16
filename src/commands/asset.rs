use crate::cli::{AssetCommands, Cli};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;
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
                        asset.id.map(|i| i.to_string()).unwrap_or_default(),
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
                    println!("ID:   {}", asset.id.map(|i| i.to_string()).unwrap_or_default());
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

        AssetCommands::Remove { id } => {
            let resource_name = format!("customers/{}/assets/{}", customer_id, id);
            let op: MutateOperation<Asset> = MutateOperation {
                create: None,
                update: None,
                remove: Some(resource_name.clone()),
                update_mask: None,
            };
            client
                .mutate(&customer_id, "assets", vec![op], false, false)
                .await?;
            println!("Removed asset: {}", resource_name);
        }
    }

    Ok(())
}
