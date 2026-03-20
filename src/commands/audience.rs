use crate::cli::{Cli, AudienceCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

pub async fn handle(command: &AudienceCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    match command {
        AudienceCommands::List { campaign_id, ad_group_id } => {
            handle_list(client, cli, campaign_id.as_deref(), ad_group_id.as_deref()).await
        }
        AudienceCommands::Add { campaign_id, ad_group_id, audience_id, bid_modifier } => {
            handle_add(client, cli, campaign_id.as_deref(), ad_group_id.as_deref(), audience_id, *bid_modifier).await
        }
        AudienceCommands::Remove { id } => handle_remove(client, cli, id).await,
    }
}

async fn handle_list(
    client: &GoogleAdsClient,
    cli: &Cli,
    campaign_id: Option<&str>,
    ad_group_id: Option<&str>,
) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    let query = if let Some(ag_id) = ad_group_id {
        format!(
            "SELECT ad_group_criterion.resource_name, ad_group_criterion.criterion_id, \
             ad_group_criterion.user_list.user_list, ad_group_criterion.cpc_bid_micros, \
             ad_group.id, ad_group.name \
             FROM ad_group_criterion \
             WHERE ad_group_criterion.type = 'USER_LIST' \
             AND ad_group.id = {}",
            ag_id
        )
    } else if let Some(c_id) = campaign_id {
        format!(
            "SELECT campaign_criterion.resource_name, campaign_criterion.criterion_id, \
             campaign_criterion.user_list.user_list, campaign_criterion.bid_modifier, \
             campaign.id, campaign.name \
             FROM campaign_criterion \
             WHERE campaign_criterion.type = 'USER_LIST' \
             AND campaign.id = {}",
            c_id
        )
    } else {
        "SELECT campaign_criterion.resource_name, campaign_criterion.criterion_id, \
         campaign_criterion.user_list.user_list, campaign_criterion.bid_modifier, \
         campaign.id, campaign.name \
         FROM campaign_criterion \
         WHERE campaign_criterion.type = 'USER_LIST'"
            .to_string()
    };

    let rows = client.search_all(&customer_id, &query, cli.page_size).await?;

    if rows.is_empty() {
        println!("No audience targets found.");
        return Ok(());
    }

    if ad_group_id.is_some() {
        println!(
            "{:<14} {:<40} {:<18} {:<14}",
            "Criterion ID", "User List", "CPC Bid Micros", "Resource"
        );
        println!("{}", "-".repeat(86));

        for row in &rows {
            let agc = row.ad_group_criterion.as_ref();
            let cid = agc.and_then(|c| c.criterion_id.as_deref()).unwrap_or("-");
            let resource = agc.map(|c| c.resource_name.as_str()).unwrap_or("-");
            let bid = agc
                .and_then(|c| c.cpc_bid_micros)
                .map(|b| b.to_string())
                .unwrap_or_else(|| "-".to_string());

            println!("{:<14} {:<40} {:<18} {:<14}", cid, "-", bid, resource);
        }
    } else {
        println!(
            "{:<14} {:<40} {:<14} {:<14}",
            "Criterion ID", "User List", "Bid Modifier", "Resource"
        );
        println!("{}", "-".repeat(82));

        for row in &rows {
            let cc = row.campaign_criterion.as_ref();
            let cid = cc.and_then(|c| c.criterion_id.as_deref()).unwrap_or("-");
            let user_list = cc
                .and_then(|c| c.user_list.as_ref())
                .and_then(|u| u.user_list.as_deref())
                .unwrap_or("-");
            let bid_mod = cc
                .and_then(|c| c.bid_modifier)
                .map(|b| format!("{:.2}", b))
                .unwrap_or_else(|| "-".to_string());
            let resource = cc.map(|c| c.resource_name.as_str()).unwrap_or("-");

            println!("{:<14} {:<40} {:<14} {:<14}", cid, user_list, bid_mod, resource);
        }
    }

    println!("\nTotal: {} audience target(s)", rows.len());
    Ok(())
}

async fn handle_add(
    client: &GoogleAdsClient,
    cli: &Cli,
    campaign_id: Option<&str>,
    ad_group_id: Option<&str>,
    audience_id: &str,
    bid_modifier: Option<f64>,
) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    if let Some(ag_id) = ad_group_id {
        let ad_group_resource = format!("customers/{}/adGroups/{}", customer_id, ag_id);
        let user_list_resource = format!("customers/{}/userLists/{}", customer_id, audience_id);

        let mut payload = serde_json::json!({
            "adGroup": ad_group_resource,
            "userList": { "userList": user_list_resource }
        });

        if let Some(bm) = bid_modifier {
            payload["cpcBidMicros"] = serde_json::json!((bm * 1_000_000.0) as i64);
        }

        let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
            create: Some(payload),
            update: None,
            remove: None,
            update_mask: None,
        }];

        if cli.dry_run {
            println!("[dry-run] Would add audience {} to ad group {}", audience_id, ag_id);
            return Ok(());
        }

        let response = client
            .mutate(&customer_id, "adGroupCriteria", operations, false, false)
            .await?;

        match response.results.first() {
            Some(result) => println!("Created audience target: {}", result.resource_name),
            None => println!("Audience target created."),
        }
    } else {
        let c_id = campaign_id.unwrap_or("0");
        let campaign_resource = format!("customers/{}/campaigns/{}", customer_id, c_id);
        let user_list_resource = format!("customers/{}/userLists/{}", customer_id, audience_id);

        let mut payload = serde_json::json!({
            "campaign": campaign_resource,
            "userList": { "userList": user_list_resource }
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
            println!("[dry-run] Would add audience {} to campaign {}", audience_id, c_id);
            return Ok(());
        }

        let response = client
            .mutate(&customer_id, "campaignCriteria", operations, false, false)
            .await?;

        match response.results.first() {
            Some(result) => println!("Created audience target: {}", result.resource_name),
            None => println!("Audience target created."),
        }
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

    // Determine the resource type from the resource name
    let resource_type = if resource_name.contains("adGroupCriteria") {
        "adGroupCriteria"
    } else {
        "campaignCriteria"
    };

    let operations: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
        create: None,
        update: None,
        remove: Some(resource_name.clone()),
        update_mask: None,
    }];

    if cli.dry_run {
        println!("[dry-run] Would remove audience target: {}", resource_name);
        return Ok(());
    }

    client
        .mutate(&customer_id, resource_type, operations, false, false)
        .await?;

    println!("Removed audience target: {}", resource_name);
    Ok(())
}
