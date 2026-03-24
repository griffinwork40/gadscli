#![allow(dead_code)]

use crate::client::GoogleAdsClient;
use crate::error::{GadsError, Result};
use crate::types::responses_ext::GenerateKeywordIdeasResponse;

pub async fn handle_ideas(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    texts: &[String],
    url: Option<&str>,
    language: Option<&str>,
    geo_ids: &[String],
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    if texts.is_empty() && url.is_none() {
        return Err(GadsError::Validation(
            "At least one of --text or --url must be provided.".to_string(),
        ));
    }

    let mut body = serde_json::json!({});

    match (!texts.is_empty(), url.is_some()) {
        (true, true) => {
            body["keywordAndUrlSeed"] = serde_json::json!({
                "keywords": texts,
                "url": url.unwrap()
            });
        }
        (true, false) => {
            body["keywordSeed"] = serde_json::json!({ "keywords": texts });
        }
        (false, true) => {
            body["urlSeed"] = serde_json::json!({ "url": url.unwrap() });
        }
        (false, false) => unreachable!(),
    }
    if let Some(lang) = language {
        body["language"] = serde_json::json!(lang);
    }
    if !geo_ids.is_empty() {
        let constants: Vec<String> = geo_ids
            .iter()
            .map(|id| format!("geoTargetConstants/{}", id))
            .collect();
        body["geoTargetConstants"] = serde_json::json!(constants);
    }

    let api_url = format!(
        "{}/customers/{}:generateKeywordIdeas",
        client.base_url(),
        cid
    );

    let response_json = client
        .http()
        .execute(reqwest::Method::POST, &api_url, Some(body))
        .await?;
    let response: GenerateKeywordIdeasResponse = serde_json::from_value(response_json)
        .unwrap_or_else(|_| GenerateKeywordIdeasResponse { results: vec![] });

    if response.results.is_empty() {
        println!("No keyword ideas found.");
        return Ok(());
    }

    println!(
        "{:<40} {:<15} {:<12} {:<15} {:<15}",
        "Keyword", "Avg Searches", "Competition", "Low Bid", "High Bid"
    );
    println!("{}", "-".repeat(100));

    for idea in &response.results {
        let text = idea.text.as_deref().unwrap_or("-");
        let metrics = idea.keyword_idea_metrics.as_ref();
        let searches = metrics
            .and_then(|m| m.avg_monthly_searches)
            .map(|v| v.to_string())
            .unwrap_or_else(|| "-".to_string());
        let competition = metrics
            .and_then(|m| m.competition.as_deref())
            .unwrap_or("-");
        let low_bid = metrics
            .and_then(|m| m.low_top_of_page_bid_micros)
            .map(|v| format!("${:.2}", v as f64 / 1_000_000.0))
            .unwrap_or_else(|| "-".to_string());
        let high_bid = metrics
            .and_then(|m| m.high_top_of_page_bid_micros)
            .map(|v| format!("${:.2}", v as f64 / 1_000_000.0))
            .unwrap_or_else(|| "-".to_string());

        println!(
            "{:<40} {:<15} {:<12} {:<15} {:<15}",
            text, searches, competition, low_bid, high_bid
        );
    }

    println!("\nTotal: {} keyword idea(s)", response.results.len());
    Ok(())
}
