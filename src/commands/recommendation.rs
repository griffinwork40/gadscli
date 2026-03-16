use crate::cli::{Cli, RecommendationCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn handle(
    command: &RecommendationCommands,
    client: &GoogleAdsClient,
    cli: &Cli,
) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        RecommendationCommands::List { recommendation_type } => {
            let query = if let Some(rec_type) = recommendation_type {
                format!(
                    "SELECT recommendation.resource_name, recommendation.type, \
                     recommendation.impact FROM recommendation \
                     WHERE recommendation.type = '{}'",
                    rec_type
                )
            } else {
                "SELECT recommendation.resource_name, recommendation.type, \
                 recommendation.impact FROM recommendation"
                    .to_string()
            };

            let rows = client.search_all(&customer_id, &query, Some(1000)).await?;
            if rows.is_empty() {
                println!("No recommendations found.");
                return Ok(());
            }
            println!("{:<20} {:<60}", "Type", "Resource Name");
            println!("{}", "-".repeat(80));
            for row in &rows {
                if let Some(rec) = &row.recommendation {
                    println!(
                        "{:<20} {:<60}",
                        rec.recommendation_type.as_deref().unwrap_or("-"),
                        rec.resource_name,
                    );
                    if let Some(impact) = &rec.impact {
                        println!("  Impact: {}", impact);
                    }
                }
            }
        }

        RecommendationCommands::Apply { id } => {
            let url = format!(
                "{}/customers/{}/recommendations:apply",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{ "resourceName": id }]
            });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            println!("Applied recommendation: {}", id);
            if let Some(results) = response.get("results") {
                println!("Results: {}", serde_json::to_string_pretty(results).unwrap_or_default());
            }
        }

        RecommendationCommands::Dismiss { id } => {
            let url = format!(
                "{}/customers/{}/recommendations:dismiss",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{ "resourceName": id }]
            });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            println!("Dismissed recommendation: {}", id);
            if let Some(results) = response.get("results") {
                println!("Results: {}", serde_json::to_string_pretty(results).unwrap_or_default());
            }
        }
    }

    Ok(())
}
