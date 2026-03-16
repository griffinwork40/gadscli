use crate::cli::AccountCommands;
use crate::client::GoogleAdsClient;
use crate::config::Config;
use crate::error::{GadsError, Result};

pub async fn handle(command: &AccountCommands, client: &GoogleAdsClient, config: &Config) -> Result<()> {
    match command {
        AccountCommands::List => list(client, config).await,
        AccountCommands::Info => info(client, config).await,
        AccountCommands::Hierarchy => hierarchy(client, config).await,
    }
}

fn resolve_customer_id(config: &Config) -> Result<&str> {
    config.customer_id.as_deref().ok_or_else(|| {
        GadsError::Config(
            "No customer ID set. Use --customer-id or 'gadscli config set customer_id <id>'.".into(),
        )
    })
}

async fn list(client: &GoogleAdsClient, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(config)?;

    let query = "SELECT customer_client.descriptive_name, customer_client.id, \
                 customer_client.manager, customer_client.currency_code \
                 FROM customer_client";

    let results = client.search_all(customer_id, query, Some(config.page_size)).await?;

    if results.is_empty() {
        println!("No accessible accounts found.");
        return Ok(());
    }

    println!(
        "{:<15} {:<40} {:<10} {:<10}",
        "ID", "NAME", "MANAGER", "CURRENCY"
    );
    println!("{}", "-".repeat(80));

    for row in &results {
        if let Some(cc) = &row.customer_client {
            let id = cc.id.map(|i| i.to_string()).unwrap_or_default();
            let name = cc.descriptive_name.as_deref().unwrap_or("(unnamed)");
            let manager = cc.manager.map(|b| if b { "yes" } else { "no" }).unwrap_or("no");
            let currency = cc.currency_code.as_deref().unwrap_or("");
            println!("{:<15} {:<40} {:<10} {:<10}", id, name, manager, currency);
        }
    }

    Ok(())
}

async fn info(client: &GoogleAdsClient, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(config)?;

    // Query customer info via the REST resource endpoint
    let url = format!("{}/customers/{}", client.base_url(), customer_id);
    let response = client.http().execute(reqwest::Method::GET, &url, None).await?;

    println!("Account Information");
    println!("-------------------");

    if let Some(id) = response.get("id").and_then(|v| v.as_str()) {
        println!("{:<25} {}", "Customer ID:", id);
    } else if let Some(id) = response.get("id") {
        println!("{:<25} {}", "Customer ID:", id);
    }

    if let Some(name) = response.get("descriptiveName").and_then(|v| v.as_str()) {
        println!("{:<25} {}", "Name:", name);
    }
    if let Some(currency) = response.get("currencyCode").and_then(|v| v.as_str()) {
        println!("{:<25} {}", "Currency:", currency);
    }
    if let Some(tz) = response.get("timeZone").and_then(|v| v.as_str()) {
        println!("{:<25} {}", "Time zone:", tz);
    }
    if let Some(status) = response.get("status").and_then(|v| v.as_str()) {
        println!("{:<25} {}", "Status:", status);
    }
    if let Some(manager) = response.get("manager").and_then(|v| v.as_bool()) {
        println!("{:<25} {}", "Manager account:", if manager { "yes" } else { "no" });
    }
    if let Some(auto_tag) = response.get("autoTaggingEnabled").and_then(|v| v.as_bool()) {
        println!("{:<25} {}", "Auto-tagging:", if auto_tag { "yes" } else { "no" });
    }
    if let Some(template) = response.get("trackingUrlTemplate").and_then(|v| v.as_str()) {
        if !template.is_empty() {
            println!("{:<25} {}", "Tracking template:", template);
        }
    }

    Ok(())
}

async fn hierarchy(client: &GoogleAdsClient, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(config)?;

    let query = "SELECT customer_client.descriptive_name, customer_client.id, \
                 customer_client.manager, customer_client.currency_code, \
                 customer_client.level \
                 FROM customer_client ORDER BY customer_client.level ASC";

    let results = client.search_all(customer_id, query, Some(config.page_size)).await?;

    if results.is_empty() {
        println!("No account hierarchy found.");
        return Ok(());
    }

    println!("Account Hierarchy");
    println!("-----------------");
    println!(
        "{:<5} {:<15} {:<40} {:<10} {:<10}",
        "LEVEL", "ID", "NAME", "MANAGER", "CURRENCY"
    );
    println!("{}", "-".repeat(85));

    for row in &results {
        if let Some(cc) = &row.customer_client {
            let level = cc.level.unwrap_or(0) as usize;
            let level_str = level.to_string();
            let id = cc.id.map(|i| i.to_string()).unwrap_or_default();
            let name = cc.descriptive_name.as_deref().unwrap_or("(unnamed)");
            let manager = cc.manager.map(|b| if b { "yes" } else { "no" }).unwrap_or("no");
            let currency = cc.currency_code.as_deref().unwrap_or("");
            let indent = "  ".repeat(level);
            let indented_name = format!("{}{}", indent, name);
            println!(
                "{:<5} {:<15} {:<40} {:<10} {:<10}",
                level_str, id, indented_name, manager, currency
            );
        }
    }

    Ok(())
}
