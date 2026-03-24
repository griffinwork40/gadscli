use crate::cli::{BatchCommands, Cli};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::batch::{AddOperationsResponse, BatchJob, LongRunningOperation};
use indicatif::ProgressBar;
use std::time::{Duration, Instant};

pub async fn handle(command: &BatchCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        BatchCommands::Create => {
            if cli.dry_run {
                println!("[dry-run] Would create a new batch job.");
                return Ok(());
            }

            let url = format!(
                "{}/customers/{}/batchJobs:mutate",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{ "create": {} }]
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

            println!("Created batch job: {}", resource_name);
        }

        BatchCommands::AddOperations {
            job_id,
            file,
            json,
        } => {
            if cli.dry_run {
                println!(
                    "[dry-run] Would add operations to batch job {}.",
                    job_id
                );
                return Ok(());
            }

            let operations_value: serde_json::Value = if let Some(file_path) = file {
                let content = std::fs::read_to_string(file_path)?;
                serde_json::from_str(&content)?
            } else if let Some(json_str) = json {
                serde_json::from_str(json_str)?
            } else {
                return Err(crate::error::GadsError::Other(
                    "Either --file or --json must be provided".to_string(),
                ));
            };

            let url = format!(
                "{}/customers/{}/batchJobs/{}:addOperations",
                client.base_url(),
                customer_id,
                job_id
            );
            let request_body = serde_json::json!({
                "mutateOperations": operations_value
            });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            let add_ops_response: AddOperationsResponse =
                serde_json::from_value(response)?;

            println!(
                "Operations added. Next sequence token: {}",
                add_ops_response
                    .next_sequence_token
                    .unwrap_or_else(|| "none".to_string())
            );
        }

        BatchCommands::Run { id } => {
            if cli.dry_run {
                println!("[dry-run] Would run batch job {}.", id);
                return Ok(());
            }

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

            let lro: LongRunningOperation = serde_json::from_value(response)?;

            println!(
                "Batch job {} started. Operation: {}",
                id,
                lro.name.unwrap_or_else(|| "unknown".to_string())
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

            let batch_job: BatchJob = serde_json::from_value(response)?;

            println!("Batch Job ID:     {}", id);
            println!(
                "Resource Name:    {}",
                batch_job.resource_name.unwrap_or_else(|| "-".to_string())
            );
            println!(
                "Status:           {}",
                batch_job
                    .status
                    .unwrap_or_else(|| "UNKNOWN".to_string())
            );

            if let Some(metadata) = batch_job.metadata {
                if let Some(ratio) = metadata.estimated_completion_ratio {
                    println!("Completion:       {:.1}%", ratio * 100.0);
                }
                if let Some(count) = metadata.operation_count {
                    println!("Operations:       {}", count);
                }
            }
        }

        BatchCommands::Results { id } => {
            let url = format!(
                "{}/customers/{}/batchJobs/{}:listResults",
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

        BatchCommands::Wait {
            id,
            timeout_secs,
            poll_interval_secs,
        } => {
            let start = Instant::now();
            let timeout = Duration::from_secs(*timeout_secs);
            let interval = Duration::from_secs(*poll_interval_secs);

            let pb = ProgressBar::new_spinner();
            pb.set_message(format!("Waiting for batch job {} ...", id));

            loop {
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

                let batch_job: BatchJob = serde_json::from_value(response)?;
                let status = batch_job
                    .status
                    .clone()
                    .unwrap_or_else(|| "UNKNOWN".to_string());

                pb.set_message(format!("Batch job {} status: {}", id, status));

                if status == "DONE" {
                    pb.finish_with_message(format!("Batch job {} completed.", id));
                    return Ok(());
                }

                if start.elapsed() >= timeout {
                    pb.finish_with_message(format!(
                        "Timed out waiting for batch job {} (status: {}).",
                        id, status
                    ));
                    return Ok(());
                }

                tokio::time::sleep(interval).await;
            }
        }
    }

    Ok(())
}
