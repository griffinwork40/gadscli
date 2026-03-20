use crate::cli::{BudgetCommands, Cli};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

pub async fn handle(command: &BudgetCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        BudgetCommands::List => {
            let query = "SELECT campaign_budget.id, campaign_budget.name, \
                         campaign_budget.amount_micros, campaign_budget.delivery_method, \
                         campaign_budget.status \
                         FROM campaign_budget \
                         WHERE campaign_budget.status != 'REMOVED' \
                         ORDER BY campaign_budget.name";

            let rows = client.search_all(&customer_id, query, cli.page_size).await?;

            if rows.is_empty() {
                println!("No budgets found.");
                return Ok(());
            }

            println!(
                "{:<12} {:<40} {:<14} {:<12} {:<10}",
                "ID", "Name", "Amount/Day", "Delivery", "Status"
            );
            println!("{}", "-".repeat(88));

            for row in &rows {
                let budget = row.campaign_budget.as_ref();

                let id = budget
                    .and_then(|b| b.id.clone())
                    .unwrap_or_default();
                let name = budget
                    .and_then(|b| b.name.as_deref())
                    .unwrap_or("-")
                    .to_string();
                let name_truncated = if name.len() > 38 {
                    format!("{}…", &name[..37])
                } else {
                    name
                };
                let amount = budget
                    .and_then(|b| b.amount_micros)
                    .map(|a| format!("${:.2}", a as f64 / 1_000_000.0))
                    .unwrap_or_else(|| "$0.00".to_string());
                let delivery = budget
                    .and_then(|b| b.delivery_method.as_deref())
                    .unwrap_or("-")
                    .to_string();
                let status = budget
                    .and_then(|b| b.status.as_deref())
                    .unwrap_or("-")
                    .to_string();

                println!(
                    "{:<12} {:<40} {:<14} {:<12} {:<10}",
                    id, name_truncated, amount, delivery, status
                );
            }

            println!("\nTotal: {} budget(s)", rows.len());
        }

        BudgetCommands::Get { id } => {
            let query = format!(
                "SELECT campaign_budget.id, campaign_budget.name, \
                 campaign_budget.amount_micros, campaign_budget.delivery_method, \
                 campaign_budget.status \
                 FROM campaign_budget \
                 WHERE campaign_budget.id = {}",
                id
            );

            let rows = client.search_all(&customer_id, &query, cli.page_size).await?;

            match rows.first().and_then(|r| r.campaign_budget.as_ref()) {
                None => println!("Budget {} not found.", id),
                Some(budget) => {
                    println!("Budget Details");
                    println!("{}", "=".repeat(40));
                    println!("ID:           {}", budget.id.clone().unwrap_or_default());
                    println!("Name:         {}", budget.name.as_deref().unwrap_or("-"));
                    println!(
                        "Amount/Day:   {}",
                        budget
                            .amount_micros
                            .map(|a| format!("${:.2}", a as f64 / 1_000_000.0))
                            .unwrap_or_else(|| "$0.00".to_string())
                    );
                    println!("Delivery:     {}", budget.delivery_method.as_deref().unwrap_or("-"));
                    println!("Status:       {}", budget.status.as_deref().unwrap_or("-"));
                    println!("Resource:     {}", budget.resource_name);
                }
            }
        }

        BudgetCommands::Create { name, amount_micros } => {
            let payload = serde_json::json!({
                "name": name,
                "amountMicros": amount_micros,
                "deliveryMethod": "STANDARD"
            });

            let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: Some(payload),
                update: None,
                remove: None,
                update_mask: None,
            }];

            if cli.dry_run {
                println!("[dry-run] Would create budget \"{}\"", name);
                println!(
                    "  Amount/Day: ${:.2}",
                    *amount_micros as f64 / 1_000_000.0
                );
                println!("  Delivery method: STANDARD");
                return Ok(());
            }

            let response = client
                .mutate(&customer_id, "campaignBudgets", operations, false, false)
                .await?;

            match response.results.first() {
                Some(result) => println!("Created budget: {}", result.resource_name),
                None => println!("Budget created (no resource name returned)."),
            }
        }

        BudgetCommands::Update {
            id,
            name,
            amount_micros,
        } => {
            let resource_name = format!("customers/{}/campaignBudgets/{}", customer_id, id);

            let mut update_fields: Vec<&str> = Vec::new();
            let mut payload = serde_json::json!({ "resourceName": resource_name });

            if let Some(n) = name {
                payload["name"] = serde_json::Value::String(n.clone());
                update_fields.push("name");
            }
            if let Some(a) = amount_micros {
                payload["amountMicros"] = serde_json::json!(a);
                update_fields.push("amount_micros");
            }

            if update_fields.is_empty() {
                println!("No fields to update.");
                return Ok(());
            }

            let update_mask = update_fields.join(",");

            let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: None,
                update: Some(payload),
                remove: None,
                update_mask: Some(update_mask),
            }];

            if cli.dry_run {
                println!("[dry-run] Would update budget {} (fields: {})", id, update_fields.join(", "));
                return Ok(());
            }

            let response = client
                .mutate(&customer_id, "campaignBudgets", operations, false, false)
                .await?;

            match response.results.first() {
                Some(result) => println!("Updated budget: {}", result.resource_name),
                None => println!("Budget updated."),
            }
        }

        BudgetCommands::Remove { id } => {
            let resource_name = format!("customers/{}/campaignBudgets/{}", customer_id, id);

            let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: None,
                update: None,
                remove: Some(resource_name.clone()),
                update_mask: None,
            }];

            if cli.dry_run {
                println!("[dry-run] Would remove budget {}", id);
                return Ok(());
            }

            client
                .mutate(&customer_id, "campaignBudgets", operations, false, false)
                .await?;

            println!("Removed budget: {}", resource_name);
        }
    }

    Ok(())
}
