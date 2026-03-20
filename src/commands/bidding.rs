#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

/// Bidding strategy create/update payload
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct BiddingStrategyMutate {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub strategy_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_cpa: Option<TargetCpaConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_roas: Option<TargetRoasConfig>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TargetCpaConfig {
    pub target_cpa_micros: i64,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct TargetRoasConfig {
    pub target_roas: f64,
}

pub async fn handle_list(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    let query = "SELECT bidding_strategy.id, bidding_strategy.name, bidding_strategy.type, \
                 bidding_strategy.target_cpa.target_cpa_micros, bidding_strategy.target_roas.target_roas \
                 FROM bidding_strategy";

    let rows = client.search_all(&cid, query, Some(1000)).await?;

    if rows.is_empty() {
        println!("No bidding strategies found.");
        return Ok(());
    }

    println!(
        "{:<12} {:<40} {:<30} {:<16} {:<12}",
        "ID", "Name", "Type", "Target CPA", "Target ROAS"
    );
    println!("{}", "-".repeat(115));

    for row in &rows {
        if let Some(strategy) = &row.bidding_strategy {
            let id = strategy
                .id
                .clone()
                .unwrap_or_else(|| "-".to_string());

            let name = strategy
                .name
                .clone()
                .unwrap_or_else(|| "-".to_string());

            let strategy_type = strategy
                .strategy_type
                .as_ref()
                .map(|t| t.to_string())
                .unwrap_or_else(|| "-".to_string());

            let target_cpa = strategy
                .target_cpa_micros
                .map(|v| format!("${:.2}", v as f64 / 1_000_000.0))
                .unwrap_or_else(|| "-".to_string());

            let target_roas = strategy
                .target_roas
                .map(|v| format!("{:.4}", v))
                .unwrap_or_else(|| "-".to_string());

            println!(
                "{:<12} {:<40} {:<30} {:<16} {:<12}",
                id, name, strategy_type, target_cpa, target_roas
            );
        }
    }

    println!("\nTotal: {} bidding strategy/strategies", rows.len());
    Ok(())
}

pub async fn handle_get(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    id: &str,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    let resource_name = if id.starts_with("customers/") {
        id.to_string()
    } else {
        format!("customers/{}/biddingStrategies/{}", cid, id)
    };

    let query = format!(
        "SELECT bidding_strategy.id, bidding_strategy.name, bidding_strategy.type, \
         bidding_strategy.target_cpa.target_cpa_micros, bidding_strategy.target_roas.target_roas \
         FROM bidding_strategy \
         WHERE bidding_strategy.resource_name = '{}'",
        resource_name
    );

    let rows = client.search_all(&cid, &query, Some(10)).await?;

    match rows.first().and_then(|r| r.bidding_strategy.as_ref()) {
        None => println!("Bidding strategy not found: {}", id),
        Some(strategy) => {
            println!("Bidding Strategy Details");
            println!("{}", "-".repeat(40));
            println!("Resource Name: {}", strategy.resource_name);
            println!(
                "ID:            {}",
                strategy.id.clone().unwrap_or_else(|| "-".to_string())
            );
            println!(
                "Name:          {}",
                strategy.name.as_deref().unwrap_or("-")
            );
            println!(
                "Type:          {}",
                strategy.strategy_type.as_ref().map(|t| t.to_string()).unwrap_or_else(|| "-".to_string())
            );
            if let Some(cpa) = strategy.target_cpa_micros {
                println!("Target CPA:    ${:.2}", cpa as f64 / 1_000_000.0);
            }
            if let Some(roas) = strategy.target_roas {
                println!("Target ROAS:   {:.4}", roas);
            }
        }
    }

    Ok(())
}

pub async fn handle_create(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    name: &str,
    strategy_type: &str,
    target_cpa_micros: Option<i64>,
    target_roas: Option<f64>,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    if dry_run {
        println!("[DRY RUN] Would create bidding strategy:");
        println!("  Name: {}", name);
        println!("  Type: {}", strategy_type);
        if let Some(cpa) = target_cpa_micros {
            println!("  Target CPA: ${:.2}", cpa as f64 / 1_000_000.0);
        }
        if let Some(roas) = target_roas {
            println!("  Target ROAS: {:.4}", roas);
        }
        return Ok(());
    }

    let type_upper = strategy_type.to_uppercase();

    let mut payload = BiddingStrategyMutate {
        name: Some(name.to_string()),
        strategy_type: Some(type_upper.clone()),
        ..Default::default()
    };

    match type_upper.as_str() {
        "TARGET_CPA" => {
            if let Some(cpa) = target_cpa_micros {
                payload.target_cpa = Some(TargetCpaConfig {
                    target_cpa_micros: cpa,
                });
            }
        }
        "TARGET_ROAS" => {
            if let Some(roas) = target_roas {
                payload.target_roas = Some(TargetRoasConfig { target_roas: roas });
            }
        }
        // MAXIMIZE_CLICKS, MAXIMIZE_CONVERSIONS, etc. — just name + type
        _ => {}
    }

    let ops: Vec<MutateOperation<BiddingStrategyMutate>> = vec![MutateOperation {
        create: Some(payload),
        update: None,
        remove: None,
        update_mask: None,
    }];

    let response = client
        .mutate(&cid, "biddingStrategies", ops, false, dry_run)
        .await?;

    if let Some(result) = response.results.first() {
        println!("Bidding strategy created: {}", result.resource_name);
    } else {
        println!("Bidding strategy created successfully.");
    }

    Ok(())
}

pub async fn handle_update(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    id: &str,
    name: Option<&str>,
    target_cpa_micros: Option<i64>,
    target_roas: Option<f64>,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    let resource_name = if id.starts_with("customers/") {
        id.to_string()
    } else {
        format!("customers/{}/biddingStrategies/{}", cid, id)
    };

    if dry_run {
        println!("[DRY RUN] Would update bidding strategy: {}", resource_name);
        if let Some(n) = name {
            println!("  Name: {}", n);
        }
        if let Some(cpa) = target_cpa_micros {
            println!("  Target CPA: ${:.2}", cpa as f64 / 1_000_000.0);
        }
        if let Some(roas) = target_roas {
            println!("  Target ROAS: {:.4}", roas);
        }
        return Ok(());
    }

    let mut update_fields: Vec<String> = Vec::new();
    let mut payload = BiddingStrategyMutate {
        resource_name: Some(resource_name.clone()),
        ..Default::default()
    };

    if let Some(n) = name {
        payload.name = Some(n.to_string());
        update_fields.push("name".to_string());
    }

    if let Some(cpa) = target_cpa_micros {
        payload.target_cpa = Some(TargetCpaConfig {
            target_cpa_micros: cpa,
        });
        update_fields.push("target_cpa.target_cpa_micros".to_string());
    }

    if let Some(roas) = target_roas {
        payload.target_roas = Some(TargetRoasConfig { target_roas: roas });
        update_fields.push("target_roas.target_roas".to_string());
    }

    let update_mask = update_fields.join(",");

    let ops: Vec<MutateOperation<BiddingStrategyMutate>> = vec![MutateOperation {
        create: None,
        update: Some(payload),
        remove: None,
        update_mask: Some(update_mask),
    }];

    client
        .mutate(&cid, "biddingStrategies", ops, false, dry_run)
        .await?;

    println!("Bidding strategy updated: {}", resource_name);
    Ok(())
}

pub async fn handle_remove(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    id: &str,
    dry_run: bool,
) -> Result<()> {
    let cid = client.customer_id(customer_id_override)?;

    let resource_name = if id.starts_with("customers/") {
        id.to_string()
    } else {
        format!("customers/{}/biddingStrategies/{}", cid, id)
    };

    if dry_run {
        println!("[DRY RUN] Would remove bidding strategy: {}", resource_name);
        return Ok(());
    }

    let ops: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
        create: None,
        update: None,
        remove: Some(resource_name.clone()),
        update_mask: None,
    }];

    client
        .mutate(&cid, "biddingStrategies", ops, false, dry_run)
        .await?;

    println!("Bidding strategy removed: {}", resource_name);
    Ok(())
}

/// Top-level handle function called by mod.rs dispatcher
pub async fn handle(command: &crate::cli::BiddingCommands, client: &GoogleAdsClient, cli: &crate::cli::Cli) -> Result<()> {
    let cid_override = cli.customer_id.as_deref();
    let dry_run = cli.dry_run;
    match command {
        crate::cli::BiddingCommands::List => {
            handle_list(client, cid_override).await
        }
        crate::cli::BiddingCommands::Get { id } => {
            handle_get(client, cid_override, id).await
        }
        crate::cli::BiddingCommands::Create { name, strategy_type, target_cpa_micros, target_roas } => {
            handle_create(client, cid_override, name, strategy_type, *target_cpa_micros, *target_roas, dry_run).await
        }
        crate::cli::BiddingCommands::Update { id, name, target_cpa_micros, target_roas } => {
            handle_update(client, cid_override, id, name.as_deref(), *target_cpa_micros, *target_roas, dry_run).await
        }
        crate::cli::BiddingCommands::Remove { id } => {
            handle_remove(client, cid_override, id, dry_run).await
        }
    }
}

/// Dispatch bidding subcommands
pub enum BiddingCommand {
    List,
    Get {
        id: String,
    },
    Create {
        name: String,
        strategy_type: String,
        target_cpa_micros: Option<i64>,
        target_roas: Option<f64>,
    },
    Update {
        id: String,
        name: Option<String>,
        target_cpa_micros: Option<i64>,
        target_roas: Option<f64>,
    },
    Remove {
        id: String,
    },
}

pub async fn execute(
    client: &GoogleAdsClient,
    customer_id_override: Option<&str>,
    cmd: BiddingCommand,
    dry_run: bool,
) -> Result<()> {
    match cmd {
        BiddingCommand::List => handle_list(client, customer_id_override).await,
        BiddingCommand::Get { id } => handle_get(client, customer_id_override, &id).await,
        BiddingCommand::Create {
            name,
            strategy_type,
            target_cpa_micros,
            target_roas,
        } => {
            handle_create(
                client,
                customer_id_override,
                &name,
                &strategy_type,
                target_cpa_micros,
                target_roas,
                dry_run,
            )
            .await
        }
        BiddingCommand::Update {
            id,
            name,
            target_cpa_micros,
            target_roas,
        } => {
            handle_update(
                client,
                customer_id_override,
                &id,
                name.as_deref(),
                target_cpa_micros,
                target_roas,
                dry_run,
            )
            .await
        }
        BiddingCommand::Remove { id } => {
            handle_remove(client, customer_id_override, &id, dry_run).await
        }
    }
}
