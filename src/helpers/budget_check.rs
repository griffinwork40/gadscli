use crate::cli::Cli;
use crate::client::GoogleAdsClient;
use crate::error::Result;

pub async fn run(client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;
    let query = "SELECT campaign.name, campaign.status, \
                 campaign_budget.amount_micros, \
                 metrics.cost_micros \
                 FROM campaign \
                 WHERE campaign.status = 'ENABLED'";

    let results = client.search_all(&cid, query, cli.page_size).await?;

    if results.is_empty() {
        println!("No enabled campaigns found.");
        return Ok(());
    }

    let mut near_limit: Vec<(String, f64, f64, f64)> = Vec::new();
    let mut underspending: Vec<(String, f64, f64, f64)> = Vec::new();
    let mut normal: Vec<(String, f64, f64, f64)> = Vec::new();

    for row in &results {
        let name = row
            .campaign
            .as_ref()
            .and_then(|c| c.name.as_deref())
            .unwrap_or("-")
            .to_string();

        let budget_micros = row
            .campaign_budget
            .as_ref()
            .and_then(|b| b.amount_micros)
            .unwrap_or(0);
        let cost_micros = row
            .metrics
            .as_ref()
            .and_then(|m| m.cost_micros)
            .unwrap_or(0);

        let budget = budget_micros as f64 / 1_000_000.0;
        let cost = cost_micros as f64 / 1_000_000.0;
        let utilization = if budget > 0.0 {
            (cost / budget) * 100.0
        } else {
            0.0
        };

        if utilization >= 90.0 {
            near_limit.push((name, budget, cost, utilization));
        } else if utilization < 10.0 {
            underspending.push((name, budget, cost, utilization));
        } else {
            normal.push((name, budget, cost, utilization));
        }
    }

    println!("Budget Check Report");
    println!("{}", "=".repeat(80));
    println!("Total campaigns checked: {}", results.len());
    println!();

    if !near_limit.is_empty() {
        println!("NEAR BUDGET LIMIT (>=90% utilized):");
        println!("{}", "-".repeat(80));
        println!(
            "{:<40} {:>12} {:>12} {:>10}",
            "Campaign", "Budget", "Spend", "Utilization"
        );
        for (name, budget, cost, util) in &near_limit {
            let name_truncated = if name.len() > 38 {
                format!("{}.", &name[..38])
            } else {
                name.clone()
            };
            println!(
                "{:<40} {:>12} {:>12} {:>9.1}%",
                name_truncated,
                format!("${:.2}", budget),
                format!("${:.2}", cost),
                util
            );
        }
        println!();
    }

    if !underspending.is_empty() {
        println!("UNDERSPENDING (<10% utilized):");
        println!("{}", "-".repeat(80));
        println!(
            "{:<40} {:>12} {:>12} {:>10}",
            "Campaign", "Budget", "Spend", "Utilization"
        );
        for (name, budget, cost, util) in &underspending {
            let name_truncated = if name.len() > 38 {
                format!("{}.", &name[..38])
            } else {
                name.clone()
            };
            println!(
                "{:<40} {:>12} {:>12} {:>9.1}%",
                name_truncated,
                format!("${:.2}", budget),
                format!("${:.2}", cost),
                util
            );
        }
        println!();
    }

    println!(
        "Summary: {} near limit, {} underspending, {} normal",
        near_limit.len(),
        underspending.len(),
        normal.len()
    );

    Ok(())
}
