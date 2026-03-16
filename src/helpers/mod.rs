#![allow(dead_code)]

pub mod audit;
pub mod budget_check;
pub mod campaign_performance;
pub mod duplicate_keywords;
pub mod keyword_ideas;
pub mod quick_report;
pub mod search_terms;

use crate::cli::Cli;
use crate::client::GoogleAdsClient;
use crate::error::Result;

/// Registry of available helper commands
pub fn list_helpers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("quick-report", "Run preset reports (spend, clicks, conversions)"),
        ("campaign-performance", "Campaign performance summary with key metrics"),
        ("keyword-ideas", "Get keyword suggestions (KeywordPlanIdeaService)"),
        ("budget-check", "Check for over/under spending campaigns"),
        ("search-terms", "View search terms triggering your ads"),
        ("audit", "Run an account health check scorecard"),
        ("duplicate-keywords", "Find duplicate keywords across ad groups"),
    ]
}

/// Dispatch a helper command by name
pub async fn dispatch(name: &str, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    match name {
        "quick-report" => quick_report::run(client, cli).await,
        "campaign-performance" => campaign_performance::run(client, cli).await,
        "keyword-ideas" => keyword_ideas::run(client, cli).await,
        "budget-check" => budget_check::run(client, cli).await,
        "search-terms" => search_terms::run(client, cli).await,
        "audit" => audit::run(client, cli).await,
        "duplicate-keywords" => duplicate_keywords::run(client, cli).await,
        _ => {
            eprintln!("Unknown helper: {}", name);
            eprintln!("\nAvailable helpers:");
            for (name, desc) in list_helpers() {
                eprintln!("  +{:<25} {}", name, desc);
            }
            Err(crate::error::GadsError::Validation(format!(
                "Unknown helper command: {}",
                name
            )))
        }
    }
}
