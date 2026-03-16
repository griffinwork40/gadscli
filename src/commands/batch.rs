use crate::cli::{BatchCommands, Cli};
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn handle(command: &BatchCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        BatchCommands::Create => {
            let url = format!(
                "{}/customers/{}/batchJobs:create",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({});

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            let resource_name = response
                .get("resourceName")
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            println!("Created batch job: {}", resource_name);
            println!(
                "Full response: {}",
                serde_json::to_string_pretty(&response).unwrap_or_default()
            );
        }

        BatchCommands::Run { id } => {
            let url = format!(
                "{}/customers/{}/batchJobs/{}:run",
                client.base_url(),
                customer_id,
                id
            );

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(serde_json::json!({})))
                .await?;

            println!("Batch job {} started.", id);
            println!(
                "Operation: {}",
                serde_json::to_string_pretty(&response).unwrap_or_default()
            );
        }

        BatchCommands::Status { id } => {
            let url = format!(
                "{}/customers/{}/batchJobs/{}",
                client.base_url(),
                customer_id,
                id
            );

            let response = client
                .http()
                .execute(reqwest::Method::GET, &url, None)
                .await?;

            let status = response
                .get("status")
                .and_then(|v| v.as_str())
                .unwrap_or("UNKNOWN");
            let resource_name = response
                .get("resourceName")
                .and_then(|v| v.as_str())
                .unwrap_or("-");

            println!("Batch Job ID:     {}", id);
            println!("Resource Name:    {}", resource_name);
            println!("Status:           {}", status);

            if let Some(metadata) = response.get("metadata") {
                println!(
                    "Metadata: {}",
                    serde_json::to_string_pretty(metadata).unwrap_or_default()
                );
            }
        }

        BatchCommands::Results { id } => {
            let url = format!(
                "{}/customers/{}/batchJobs/{}/results",
                client.base_url(),
                customer_id,
                id
            );

            let response = client
                .http()
                .execute(reqwest::Method::GET, &url, None)
                .await?;

            match response.get("results").and_then(|r| r.as_array()) {
                None => {
                    println!("No results found for batch job {}.", id);
                }
                Some(results) if results.is_empty() => {
                    println!("No results found for batch job {}.", id);
                }
                Some(results) => {
                    println!("Batch job {} results ({} total):", id, results.len());
                    for (i, result) in results.iter().enumerate() {
                        println!(
                            "[{}] {}",
                            i + 1,
                            serde_json::to_string_pretty(result).unwrap_or_default()
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
