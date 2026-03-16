use crate::cli::{Cli, LabelCommands};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;
use crate::types::resources::Label;

pub async fn handle(command: &LabelCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        LabelCommands::List => {
            let query =
                "SELECT label.id, label.name, label.description FROM label";
            let rows = client.search_all(&customer_id, query, Some(1000)).await?;
            if rows.is_empty() {
                println!("No labels found.");
                return Ok(());
            }
            println!("{:<20} {:<40} {:<40}", "ID", "Name", "Description");
            println!("{}", "-".repeat(100));
            for row in &rows {
                if let Some(label) = &row.label {
                    println!(
                        "{:<20} {:<40} {:<40}",
                        label.id.map(|i| i.to_string()).unwrap_or_default(),
                        label.name.as_deref().unwrap_or("-"),
                        label.description.as_deref().unwrap_or("-"),
                    );
                }
            }
        }

        LabelCommands::Get { id } => {
            let query = format!(
                "SELECT label.id, label.name, label.description FROM label WHERE label.id = {}",
                id
            );
            let rows = client.search_all(&customer_id, &query, Some(1)).await?;
            match rows.first().and_then(|r| r.label.as_ref()) {
                None => println!("Label {} not found.", id),
                Some(label) => {
                    println!("ID:          {}", label.id.map(|i| i.to_string()).unwrap_or_default());
                    println!("Name:        {}", label.name.as_deref().unwrap_or("-"));
                    println!("Description: {}", label.description.as_deref().unwrap_or("-"));
                    if let Some(text_label) = &label.text_label {
                        println!(
                            "Color:       {}",
                            text_label.background_color.as_deref().unwrap_or("-")
                        );
                    }
                    println!("Resource:    {}", label.resource_name);
                }
            }
        }

        LabelCommands::Create {
            name,
            description,
            color,
        } => {
            let mut create_body = serde_json::json!({
                "name": name,
            });
            if let Some(desc) = description {
                create_body["description"] = serde_json::Value::String(desc.clone());
            }
            if let Some(c) = color {
                create_body["textLabel"] = serde_json::json!({ "backgroundColor": c });
            }

            let url = format!(
                "{}/customers/{}/labels:mutate",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{ "create": create_body }]
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

            println!("Created label: {}", resource_name);
        }

        LabelCommands::Update {
            id,
            name,
            description,
        } => {
            let resource_name = format!("customers/{}/labels/{}", customer_id, id);
            let mut update_body = serde_json::json!({
                "resourceName": resource_name,
            });
            let mut mask_fields: Vec<&str> = Vec::new();

            if let Some(n) = name {
                update_body["name"] = serde_json::Value::String(n.clone());
                mask_fields.push("name");
            }
            if let Some(d) = description {
                update_body["description"] = serde_json::Value::String(d.clone());
                mask_fields.push("description");
            }

            if mask_fields.is_empty() {
                println!("Nothing to update.");
                return Ok(());
            }

            let update_mask = mask_fields.join(",");
            let url = format!(
                "{}/customers/{}/labels:mutate",
                client.base_url(),
                customer_id
            );
            let request_body = serde_json::json!({
                "operations": [{
                    "updateMask": update_mask,
                    "update": update_body,
                }]
            });

            client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            println!("Updated label: {}", resource_name);
        }

        LabelCommands::Remove { id } => {
            let resource_name = format!("customers/{}/labels/{}", customer_id, id);
            let op: MutateOperation<Label> = MutateOperation {
                create: None,
                update: None,
                remove: Some(resource_name.clone()),
                update_mask: None,
            };
            client
                .mutate(&customer_id, "labels", vec![op], false, false)
                .await?;
            println!("Removed label: {}", resource_name);
        }

        LabelCommands::Assign {
            label_id,
            resource_type,
            resource_id,
        } => {
            let label_resource = format!("customers/{}/labels/{}", customer_id, label_id);

            // Map resource_type to the appropriate assignment endpoint and field name
            let (endpoint, resource_field, resource_resource) = match resource_type.to_lowercase().as_str() {
                "campaign" | "campaigns" => (
                    "campaignLabels",
                    "campaign",
                    format!("customers/{}/campaigns/{}", customer_id, resource_id),
                ),
                "adgroup" | "ad_group" | "adgroups" | "ad_groups" => (
                    "adGroupLabels",
                    "adGroup",
                    format!("customers/{}/adGroups/{}", customer_id, resource_id),
                ),
                "ad" | "ads" => (
                    "adGroupAdLabels",
                    "adGroupAd",
                    format!("customers/{}/adGroupAds/{}", customer_id, resource_id),
                ),
                "keyword" | "keywords" => (
                    "adGroupCriterionLabels",
                    "adGroupCriterion",
                    format!("customers/{}/adGroupCriteria/{}", customer_id, resource_id),
                ),
                other => {
                    return Err(crate::error::GadsError::Validation(format!(
                        "Unknown resource type '{}'. Valid types: campaign, adgroup, ad, keyword",
                        other
                    )));
                }
            };

            let url = format!(
                "{}/customers/{}/{}:mutate",
                client.base_url(),
                customer_id,
                endpoint
            );

            let create_body = serde_json::json!({
                "label": label_resource,
                resource_field: resource_resource,
            });

            let request_body = serde_json::json!({
                "operations": [{ "create": create_body }]
            });

            let response = client
                .http()
                .execute(reqwest::Method::POST, &url, Some(request_body))
                .await?;

            let result_resource = response
                .get("results")
                .and_then(|r| r.as_array())
                .and_then(|a| a.first())
                .and_then(|r| r.get("resourceName"))
                .and_then(|v| v.as_str())
                .unwrap_or("unknown");

            println!("Assigned label {} to {}: {}", label_id, resource_type, result_resource);
        }
    }

    Ok(())
}
