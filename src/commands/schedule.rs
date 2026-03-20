use crate::cli::{Cli, ScheduleCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

pub async fn handle(command: &ScheduleCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    match command {
        ScheduleCommands::List { campaign_id } => handle_list(client, cli, campaign_id).await,
        ScheduleCommands::Add { campaign_id, day, start_hour, end_hour, bid_modifier } => {
            handle_add(client, cli, campaign_id, day, *start_hour, *end_hour, *bid_modifier).await
        }
        ScheduleCommands::Remove { id } => handle_remove(client, cli, id).await,
    }
}

async fn handle_list(client: &GoogleAdsClient, cli: &Cli, campaign_id: &str) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    let query = format!(
        "SELECT campaign_criterion.resource_name, campaign_criterion.criterion_id, \
         campaign_criterion.ad_schedule.day_of_week, \
         campaign_criterion.ad_schedule.start_hour, campaign_criterion.ad_schedule.end_hour, \
         campaign_criterion.ad_schedule.start_minute, campaign_criterion.ad_schedule.end_minute, \
         campaign_criterion.bid_modifier, campaign.id \
         FROM campaign_criterion \
         WHERE campaign_criterion.type = 'AD_SCHEDULE' \
         AND campaign.id = {}",
        campaign_id
    );

    let rows = client.search_all(&customer_id, &query, cli.page_size).await?;

    if rows.is_empty() {
        println!("No ad schedules found for campaign {}.", campaign_id);
        return Ok(());
    }

    println!(
        "{:<14} {:<12} {:<12} {:<12} {:<14} {:<14}",
        "Criterion ID", "Day", "Start", "End", "Bid Modifier", "Resource"
    );
    println!("{}", "-".repeat(78));

    for row in &rows {
        let cc = row.campaign_criterion.as_ref();
        let cid = cc.and_then(|c| c.criterion_id.as_deref()).unwrap_or("-");
        let sched = cc.and_then(|c| c.ad_schedule.as_ref());
        let day = sched.and_then(|s| s.day_of_week.as_deref()).unwrap_or("-");
        let start = sched.and_then(|s| s.start_hour).map(|h| format!("{:02}:00", h)).unwrap_or_else(|| "-".to_string());
        let end = sched.and_then(|s| s.end_hour).map(|h| format!("{:02}:00", h)).unwrap_or_else(|| "-".to_string());
        let bid_mod = cc
            .and_then(|c| c.bid_modifier)
            .map(|b| format!("{:.2}", b))
            .unwrap_or_else(|| "-".to_string());
        let resource = cc.map(|c| c.resource_name.as_str()).unwrap_or("-");

        println!("{:<14} {:<12} {:<12} {:<12} {:<14} {:<14}", cid, day, start, end, bid_mod, resource);
    }

    println!("\nTotal: {} schedule(s)", rows.len());
    Ok(())
}

async fn handle_add(
    client: &GoogleAdsClient,
    cli: &Cli,
    campaign_id: &str,
    day: &str,
    start_hour: i32,
    end_hour: i32,
    bid_modifier: Option<f64>,
) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;
    let campaign_resource = format!("customers/{}/campaigns/{}", customer_id, campaign_id);

    let mut payload = serde_json::json!({
        "campaign": campaign_resource,
        "adSchedule": {
            "dayOfWeek": day,
            "startHour": start_hour,
            "endHour": end_hour,
            "startMinute": "ZERO",
            "endMinute": "ZERO"
        }
    });

    if let Some(bm) = bid_modifier {
        payload["bidModifier"] = serde_json::json!(bm);
    }

    let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
        create: Some(payload),
        update: None,
        remove: None,
        update_mask: None,
    }];

    if cli.dry_run {
        println!(
            "[dry-run] Would add ad schedule: {} {}-{} (bid modifier: {})",
            day,
            start_hour,
            end_hour,
            bid_modifier.map(|b| format!("{:.2}", b)).unwrap_or_else(|| "none".to_string())
        );
        return Ok(());
    }

    let response = client
        .mutate(&customer_id, "campaignCriteria", operations, false, false)
        .await?;

    match response.results.first() {
        Some(result) => println!("Created ad schedule: {}", result.resource_name),
        None => println!("Ad schedule created."),
    }

    Ok(())
}

async fn handle_remove(client: &GoogleAdsClient, cli: &Cli, id: &str) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    let resource_name = if id.contains('/') {
        id.to_string()
    } else {
        format!("customers/{}/campaignCriteria/{}", customer_id, id)
    };

    let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
        create: None,
        update: None,
        remove: Some(resource_name.clone()),
        update_mask: None,
    }];

    if cli.dry_run {
        println!("[dry-run] Would remove ad schedule: {}", resource_name);
        return Ok(());
    }

    client
        .mutate(&customer_id, "campaignCriteria", operations, false, false)
        .await?;

    println!("Removed ad schedule: {}", resource_name);
    Ok(())
}
