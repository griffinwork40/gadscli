use crate::cli::{Cli, FieldCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn handle(command: &FieldCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    // Field queries are account-agnostic but we still resolve to validate config
    let _customer_id = client.customer_id(cli.customer_id.as_deref()).ok();

    match command {
        FieldCommands::Search { resource } => {
            let query = format!(
                "SELECT name, category, data_type, selectable, filterable, sortable \
                 FROM google_ads_field WHERE name LIKE '%{}%'",
                resource
            );
            let url = format!(
                "{}/googleAdsFields:search",
                client.base_url()
            );
            let request_body = serde_json::json!({ "query": query });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            print_field_results(&response);
        }

        FieldCommands::List { resource } => {
            // List all fields for a specific resource type
            let query = format!(
                "SELECT name, category, data_type, selectable, filterable, sortable \
                 FROM google_ads_field WHERE name LIKE '{}.%'",
                resource
            );
            let url = format!(
                "{}/googleAdsFields:search",
                client.base_url()
            );
            let request_body = serde_json::json!({ "query": query });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            print_field_results(&response);
        }
    }

    Ok(())
}

fn print_field_results(response: &serde_json::Value) {
    match response.get("results").and_then(|r| r.as_array()) {
        None => {
            println!("No fields found.");
        }
        Some(fields) if fields.is_empty() => {
            println!("No fields found.");
        }
        Some(fields) => {
            println!(
                "{:<60} {:<15} {:<15} {:<10} {:<10} {:<8}",
                "Name", "Category", "Data Type", "Selectable", "Filterable", "Sortable"
            );
            println!("{}", "-".repeat(120));
            for field in fields {
                let name = field
                    .get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("-");
                let category = field
                    .get("category")
                    .and_then(|v| v.as_str())
                    .unwrap_or("-");
                let data_type = field
                    .get("dataType")
                    .and_then(|v| v.as_str())
                    .unwrap_or("-");
                let selectable = field
                    .get("selectable")
                    .and_then(|v| v.as_bool())
                    .map(|b| if b { "yes" } else { "no" })
                    .unwrap_or("-");
                let filterable = field
                    .get("filterable")
                    .and_then(|v| v.as_bool())
                    .map(|b| if b { "yes" } else { "no" })
                    .unwrap_or("-");
                let sortable = field
                    .get("sortable")
                    .and_then(|v| v.as_bool())
                    .map(|b| if b { "yes" } else { "no" })
                    .unwrap_or("-");

                println!(
                    "{:<60} {:<15} {:<15} {:<10} {:<10} {:<8}",
                    name, category, data_type, selectable, filterable, sortable
                );
            }
            println!("\nTotal: {} fields", fields.len());
        }
    }
}
