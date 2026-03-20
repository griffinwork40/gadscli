use crate::cli::{AdCommands, Cli};
use crate::client::GoogleAdsClient;
use crate::error::Result;
use crate::types::operations::MutateOperation;

/// Parse a pin string like "Text:1" into (text, pinned_field)
/// For headlines: "My Headline:1" -> ("My Headline", "HEADLINE_1")
/// For descriptions: "My Desc:2" -> ("My Desc", "DESCRIPTION_2")
pub fn parse_pin(s: &str, prefix: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = s.rsplitn(2, ':').collect();
    if parts.len() == 2 {
        let position = parts[0];
        let text = parts[1];
        Some((text.to_string(), format!("{}_{}", prefix, position)))
    } else {
        None
    }
}

fn ad_resource_name(cid: &str, id: &str) -> String {
    if id.starts_with("customers/") { id.to_string() }
    else { format!("customers/{}/adGroupAds/{}", cid, id) }
}

pub async fn handle(command: &AdCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let cid = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        AdCommands::List { ad_group_id, campaign_id } => {
            let mut query = String::from(
                "SELECT ad_group_ad.ad.id, ad_group_ad.ad.name, ad_group_ad.ad.type, \
                 ad_group_ad.status, ad_group_ad.ad.responsive_search_ad.headlines, \
                 ad_group_ad.ad.final_urls, metrics.impressions, metrics.clicks \
                 FROM ad_group_ad \
                 WHERE ad_group_ad.status != 'REMOVED'",
            );

            if let Some(ag_id) = ad_group_id {
                query.push_str(&format!(
                    " AND ad_group_ad.ad_group = 'customers/{}/adGroups/{}'",
                    cid, ag_id
                ));
            }

            if let Some(cid_filter) = campaign_id {
                query.push_str(&format!(
                    " AND ad_group.campaign = 'customers/{}/campaigns/{}'",
                    cid, cid_filter
                ));
            }

            let rows = client.search_all(&cid, &query, Some(1000)).await?;

            if rows.is_empty() {
                println!("No ads found.");
                return Ok(());
            }

            println!(
                "{:<12} {:<30} {:<25} {:<10} {:<10} {:<10}",
                "ID", "Name", "Type", "Status", "Impr.", "Clicks"
            );
            println!("{}", "-".repeat(100));

            for row in &rows {
                if let Some(aga) = &row.ad_group_ad {
                    if let Some(ad) = &aga.ad {
                        let id = ad.id.clone().unwrap_or_default();
                        let name = ad.name.as_deref().unwrap_or("-");
                        let ad_type = ad
                            .ad_type
                            .as_ref()
                            .map(|t| t.to_string())
                            .unwrap_or_default();
                        let status = aga.status.as_deref().unwrap_or("-");
                        let impressions = row
                            .metrics
                            .as_ref()
                            .and_then(|m| m.impressions)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "0".into());
                        let clicks = row
                            .metrics
                            .as_ref()
                            .and_then(|m| m.clicks)
                            .map(|v| v.to_string())
                            .unwrap_or_else(|| "0".into());

                        println!(
                            "{:<12} {:<30} {:<25} {:<10} {:<10} {:<10}",
                            id,
                            if name.len() > 28 { &name[..28] } else { name },
                            if ad_type.len() > 23 {
                                &ad_type[..23]
                            } else {
                                &ad_type
                            },
                            status,
                            impressions,
                            clicks
                        );
                    }
                }
            }
        }

        AdCommands::Get { id } => {
            let query = format!(
                "SELECT ad_group_ad.ad.id, ad_group_ad.ad.name, ad_group_ad.ad.type, \
                 ad_group_ad.status, ad_group_ad.ad.responsive_search_ad.headlines, \
                 ad_group_ad.ad.responsive_search_ad.descriptions, \
                 ad_group_ad.ad.final_urls, ad_group_ad.ad_group, \
                 metrics.impressions, metrics.clicks \
                 FROM ad_group_ad \
                 WHERE ad_group_ad.ad.id = {}",
                id
            );

            let rows = client.search_all(&cid, &query, Some(1)).await?;

            match rows.first().and_then(|r| r.ad_group_ad.as_ref()) {
                None => println!("Ad '{}' not found.", id),
                Some(aga) => {
                    let ad = match &aga.ad {
                        Some(a) => a,
                        None => {
                            println!("Ad '{}' not found.", id);
                            return Ok(());
                        }
                    };

                    println!("Ad Details");
                    println!("{}", "-".repeat(40));
                    println!("ID:       {}", ad.id.clone().unwrap_or_default());
                    println!("Name:     {}", ad.name.as_deref().unwrap_or("-"));
                    println!(
                        "Type:     {}",
                        ad.ad_type.as_ref().map(|t| t.to_string()).unwrap_or_default()
                    );
                    println!("Status:   {}", aga.status.as_deref().unwrap_or("-"));
                    println!("Ad Group: {}", aga.ad_group.as_deref().unwrap_or("-"));

                    if let Some(final_urls) = &ad.final_urls {
                        println!("Final URLs:");
                        for url in final_urls {
                            println!("  - {}", url);
                        }
                    }

                    if let Some(rsa) = &ad.responsive_search_ad {
                        if let Some(headlines) = &rsa.headlines {
                            println!("Headlines:");
                            for h in headlines {
                                println!("  - {}", h.text.as_deref().unwrap_or("-"));
                            }
                        }
                        if let Some(descriptions) = &rsa.descriptions {
                            println!("Descriptions:");
                            for d in descriptions {
                                println!("  - {}", d.text.as_deref().unwrap_or("-"));
                            }
                        }
                    }

                    if let Some(metrics) = rows.first().and_then(|r| r.metrics.as_ref()) {
                        println!("Impressions: {}", metrics.impressions.unwrap_or(0));
                        println!("Clicks:      {}", metrics.clicks.unwrap_or(0));
                    }
                }
            }
        }

        AdCommands::Create {
            ad_group_id,
            headlines,
            descriptions,
            final_url,
            pin_headline,
            pin_description,
        } => {
            // Build headline/description assets with optional pinning
            let pin_h: std::collections::HashMap<String, String> = pin_headline
                .iter()
                .filter_map(|p| parse_pin(p, "HEADLINE"))
                .collect();
            let pin_d: std::collections::HashMap<String, String> = pin_description
                .iter()
                .filter_map(|p| parse_pin(p, "DESCRIPTION"))
                .collect();

            let headline_assets: Vec<serde_json::Value> = headlines
                .iter()
                .map(|h| {
                    let mut asset = serde_json::json!({ "text": h });
                    if let Some(field) = pin_h.get(h.as_str()) {
                        asset["pinnedField"] = serde_json::json!(field);
                    }
                    asset
                })
                .collect();

            let description_assets: Vec<serde_json::Value> = descriptions
                .iter()
                .map(|d| {
                    let mut asset = serde_json::json!({ "text": d });
                    if let Some(field) = pin_d.get(d.as_str()) {
                        asset["pinnedField"] = serde_json::json!(field);
                    }
                    asset
                })
                .collect();

            let resource = serde_json::json!({
                "adGroup": format!("customers/{}/adGroups/{}", cid, ad_group_id),
                "status": "ENABLED",
                "ad": {
                    "responsiveSearchAd": {
                        "headlines": headline_assets,
                        "descriptions": description_assets
                    },
                    "finalUrls": [final_url]
                }
            });

            let ops: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: Some(resource),
                update: None,
                remove: None,
                update_mask: None,
            }];

            if cli.dry_run {
                println!(
                    "[dry-run] Would create RSA in ad group {} with {} headlines and {} descriptions.",
                    ad_group_id,
                    headlines.len(),
                    descriptions.len()
                );
                return Ok(());
            }

            let resp = client.mutate(&cid, "adGroupAds", ops, false, false).await?;
            if let Some(result) = resp.results.first() {
                println!("Created ad: {}", result.resource_name);
            } else {
                println!("Ad created.");
            }
        }

        AdCommands::Pause { id } | AdCommands::Enable { id } => {
            let (status, verb) = if matches!(command, AdCommands::Pause { .. }) {
                ("PAUSED", "paused")
            } else {
                ("ENABLED", "enabled")
            };
            let resource_name = ad_resource_name(&cid, id);
            let resource = serde_json::json!({ "resourceName": resource_name, "status": status });
            let ops: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: None, update: Some(resource), remove: None,
                update_mask: Some("status".into()),
            }];
            if cli.dry_run {
                println!("[dry-run] Would {} ad {}.", verb, id);
                return Ok(());
            }
            client.mutate(&cid, "adGroupAds", ops, false, false).await?;
            println!("Ad {} {}.", id, verb);
        }

        AdCommands::Remove { id } => {
            let resource_name = ad_resource_name(&cid, id);
            let ops: Vec<MutateOperation<serde_json::Value>> = vec![MutateOperation {
                create: None, update: None,
                remove: Some(resource_name.clone()), update_mask: None,
            }];
            if cli.dry_run {
                println!("[dry-run] Would remove ad {}.", id);
                return Ok(());
            }
            client.mutate(&cid, "adGroupAds", ops, false, false).await?;
            println!("Ad {} removed.", id);
        }
    }

    Ok(())
}

