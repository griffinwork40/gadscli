use crate::cli::{Cli, ConversionCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn handle(
    command: &ConversionCommands,
    client: &GoogleAdsClient,
    cli: &Cli,
) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        ConversionCommands::List => {
            let query = "SELECT conversion_action.id, conversion_action.name, \
                         conversion_action.type, conversion_action.status \
                         FROM conversion_action";
            let rows = client.search_all(&customer_id, query, Some(1000)).await?;
            if rows.is_empty() {
                println!("No conversion actions found.");
                return Ok(());
            }
            println!("{:<20} {:<40} {:<20} {:<10}", "ID", "Name", "Type", "Status");
            println!("{}", "-".repeat(90));
            for row in &rows {
                if let Some(ca) = &row.conversion_action {
                    println!(
                        "{:<20} {:<40} {:<20} {:<10}",
                        ca.id.map(|i| i.to_string()).unwrap_or_default(),
                        ca.name.as_deref().unwrap_or("-"),
                        ca.action_type
                            .as_ref()
                            .map(|t| t.to_string())
                            .unwrap_or_default(),
                        ca.status.as_deref().unwrap_or("-"),
                    );
                }
            }
        }

        ConversionCommands::Get { id } => {
            let query = format!(
                "SELECT conversion_action.id, conversion_action.name, \
                 conversion_action.type, conversion_action.status \
                 FROM conversion_action WHERE conversion_action.id = {}",
                id
            );
            let rows = client.search_all(&customer_id, &query, Some(1)).await?;
            match rows.first().and_then(|r| r.conversion_action.as_ref()) {
                None => println!("Conversion action {} not found.", id),
                Some(ca) => {
                    println!("ID:     {}", ca.id.map(|i| i.to_string()).unwrap_or_default());
                    println!("Name:   {}", ca.name.as_deref().unwrap_or("-"));
                    println!(
                        "Type:   {}",
                        ca.action_type
                            .as_ref()
                            .map(|t| t.to_string())
                            .unwrap_or_default()
                    );
                    println!("Status: {}", ca.status.as_deref().unwrap_or("-"));
                    println!("Resource: {}", ca.resource_name);
                }
            }
        }

        ConversionCommands::Create {
            name,
            action_type,
            category,
        } => {
            let mut create_body = serde_json::json!({
                "name": name,
                "type": action_type,
            });
            if let Some(cat) = category {
                create_body["category"] = serde_json::Value::String(cat.clone());
            }

            let url = format!(
                "{}/customers/{}/conversionActions:mutate",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{ "create": create_body }]
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

            println!("Created conversion action: {}", resource_name);
        }

        ConversionCommands::Update { id, name, status } => {
            let resource_name = format!("customers/{}/conversionActions/{}", customer_id, id);
            let mut update_body = serde_json::json!({
                "resourceName": resource_name,
            });
            let mut mask_fields: Vec<&str> = Vec::new();

            if let Some(n) = name {
                update_body["name"] = serde_json::Value::String(n.clone());
                mask_fields.push("name");
            }
            if let Some(s) = status {
                update_body["status"] = serde_json::Value::String(s.clone());
                mask_fields.push("status");
            }

            if mask_fields.is_empty() {
                println!("Nothing to update.");
                return Ok(());
            }

            let update_mask = mask_fields.join(",");
            let url = format!(
                "{}/customers/{}/conversionActions:mutate",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{
                    "updateMask": update_mask,
                    "update": update_body,
                }]
            });

            client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            println!("Updated conversion action: {}", resource_name);
        }

        ConversionCommands::Upload {
            conversion_action_id,
            gclid,
            conversion_date_time,
            conversion_value,
        } => {
            let conversion_action_resource = format!(
                "customers/{}/conversionActions/{}",
                customer_id, conversion_action_id
            );

            let mut conversion = serde_json::json!({
                "conversionAction": conversion_action_resource,
                "gclid": gclid,
                "conversionDateTime": conversion_date_time,
            });

            if let Some(value) = conversion_value {
                conversion["conversionValue"] = serde_json::json!(value);
            }

            let url = format!(
                "{}/customers/{}/conversionAdjustments:upload",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "conversions": [conversion],
                "partialFailure": true,
            });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            if let Some(err) = response.get("partialFailureError") {
                println!("Upload partial failure: {}", err);
            } else {
                println!("Conversion uploaded successfully.");
                println!("Response: {}", serde_json::to_string_pretty(&response).unwrap_or_default());
            }
        }
    }

    Ok(())
}
