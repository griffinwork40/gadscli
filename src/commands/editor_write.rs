#![allow(dead_code)]

use crate::cli::Cli;
use crate::config::Config;
use crate::editor::binary;
use crate::editor::csv_writer;
use crate::editor::database::EditorDatabaseWriter;
use crate::editor::types::*;
use crate::error::{GadsError, Result};

use super::editor_cmd::{print_editor_output, resolve_customer_id};

pub fn handle_add_keywords(
    cli: &Cli, config: &Config, campaign: &str, ad_group: &str, keywords: &[String], match_type: &str, bid: Option<f64>,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    if keywords.is_empty() { println!("No keywords specified."); return Ok(()); }
    let writer = EditorDatabaseWriter::new(customer_id)?;
    let ag_local_id = writer.find_ad_group(campaign, ad_group)?.ok_or_else(|| {
        GadsError::Validation(format!("Ad group '{}' not found in campaign '{}'", ad_group, campaign))
    })?;
    let criterion_type = match match_type.to_lowercase().as_str() { "exact" => 1, "phrase" => 2, _ => 0 };
    let max_cpc_micros = bid.map(|b| (b * 1_000_000.0) as i64).unwrap_or(0);
    let mut added = 0;
    for kw in keywords {
        let local_id = writer.add_keyword(ag_local_id, kw, criterion_type, max_cpc_micros)?;
        println!("  Added keyword '{}' (localId: {})", kw, local_id);
        added += 1;
    }
    println!("\nAdded {} keyword(s). Run 'editor post' to push to Google.", added);
    Ok(())
}

pub fn handle_pause_keyword(cli: &Cli, config: &Config, local_id: i64) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    EditorDatabaseWriter::new(customer_id)?.pause_keyword(local_id)?;
    println!("Keyword {} paused. Run 'editor post' to push to Google.", local_id);
    Ok(())
}

pub fn handle_enable_keyword(cli: &Cli, config: &Config, local_id: i64) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    EditorDatabaseWriter::new(customer_id)?.enable_keyword(local_id)?;
    println!("Keyword {} enabled. Run 'editor post' to push to Google.", local_id);
    Ok(())
}

pub fn handle_remove_keyword(cli: &Cli, config: &Config, local_id: i64) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    EditorDatabaseWriter::new(customer_id)?.remove_keyword(local_id)?;
    println!("Keyword {} marked for removal. Run 'editor post' to push to Google.", local_id);
    Ok(())
}

pub fn handle_set_campaign_status(cli: &Cli, config: &Config, local_id: i64, status: &str) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let status_code = match status.to_lowercase().as_str() {
        "enabled" => 2, "paused" => 3, "removed" => 4,
        _ => return Err(GadsError::Validation(format!("Invalid status '{}'. Use: enabled, paused, removed", status))),
    };
    EditorDatabaseWriter::new(customer_id)?.set_campaign_status(local_id, status_code)?;
    println!("Campaign {} set to {}. Run 'editor post' to push to Google.", local_id, status);
    Ok(())
}

pub fn handle_set_campaign_budget(cli: &Cli, config: &Config, local_id: i64, amount: f64) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let micros = (amount * 1_000_000.0) as i64;
    EditorDatabaseWriter::new(customer_id)?.set_campaign_budget(local_id, micros)?;
    println!("Campaign {} budget set to ${:.2}. Run 'editor post' to push to Google.", local_id, amount);
    Ok(())
}

fn make_temp_csv() -> Result<tempfile::NamedTempFile> {
    tempfile::NamedTempFile::with_suffix(".csv")
        .map_err(|e| GadsError::Other(format!("Failed to create temp file: {}", e)))
}

pub fn handle_add_ad_groups(cli: &Cli, config: &Config, ecfg: Option<&crate::config::EditorConfig>, campaign: &str, ad_groups: &[String], bid: Option<f64>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    if ad_groups.is_empty() { println!("No ad groups specified."); return Ok(()); }
    let entries: Vec<AdGroupEntry> = ad_groups.iter().map(|ag| AdGroupEntry { campaign: campaign.to_string(), ad_group: ag.clone(), max_cpc: bid, status: "Enabled".to_string() }).collect();
    let tmp = make_temp_csv()?;
    csv_writer::write_ad_group_csv(tmp.path(), &entries)?;
    let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
    print_editor_output(&output);
    println!("Added {} ad group(s) via CSV import.", entries.len());
    Ok(())
}

pub fn handle_add_negative_keywords(cli: &Cli, config: &Config, ecfg: Option<&crate::config::EditorConfig>, campaign: &str, ad_group: &Option<String>, keywords: &[String], match_type: &str) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    if keywords.is_empty() { println!("No negative keywords specified."); return Ok(()); }
    let entries: Vec<NegativeKeywordEntry> = keywords.iter().map(|kw| NegativeKeywordEntry { campaign: campaign.to_string(), ad_group: ad_group.clone(), keyword: kw.clone(), match_type: match_type.to_string() }).collect();
    let tmp = make_temp_csv()?;
    csv_writer::write_negative_keyword_csv(tmp.path(), &entries)?;
    let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
    print_editor_output(&output);
    println!("Added {} negative keyword(s) via CSV import.", entries.len());
    Ok(())
}

pub fn handle_add_sitelinks(cli: &Cli, config: &Config, ecfg: Option<&crate::config::EditorConfig>, campaign: &str, texts: &[String], urls: &[String]) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    if texts.is_empty() || urls.is_empty() { println!("Both --texts and --urls are required."); return Ok(()); }
    if texts.len() != urls.len() { return Err(GadsError::Validation("Number of --texts must match number of --urls".to_string())); }
    let entries: Vec<SitelinkEntry> = texts.iter().zip(urls.iter()).map(|(text, url)| SitelinkEntry { campaign: campaign.to_string(), ad_group: None, sitelink_text: text.clone(), final_url: url.clone(), description1: None, description2: None }).collect();
    let tmp = make_temp_csv()?;
    csv_writer::write_sitelink_csv(tmp.path(), &entries)?;
    let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
    print_editor_output(&output);
    println!("Added {} sitelink(s) via CSV import.", entries.len());
    Ok(())
}

pub fn handle_add_callouts(cli: &Cli, config: &Config, ecfg: Option<&crate::config::EditorConfig>, campaign: &str, texts: &[String]) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    if texts.is_empty() { println!("No callout texts specified."); return Ok(()); }
    let entries: Vec<CalloutEntry> = texts.iter().map(|t| CalloutEntry { campaign: campaign.to_string(), ad_group: None, callout_text: t.clone() }).collect();
    let tmp = make_temp_csv()?;
    csv_writer::write_callout_csv(tmp.path(), &entries)?;
    let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
    print_editor_output(&output);
    println!("Added {} callout(s) via CSV import.", entries.len());
    Ok(())
}

pub fn handle_add_labels(cli: &Cli, config: &Config, ecfg: Option<&crate::config::EditorConfig>, names: &[String]) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    if names.is_empty() { println!("No label names specified."); return Ok(()); }
    let entries: Vec<LabelEntry> = names.iter().map(|n| LabelEntry { label_name: n.clone(), description: None, color: None }).collect();
    let tmp = make_temp_csv()?;
    csv_writer::write_label_csv(tmp.path(), &entries)?;
    let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
    print_editor_output(&output);
    println!("Added {} label(s) via CSV import.", entries.len());
    Ok(())
}

pub fn handle_update_budgets(cli: &Cli, config: &Config, ecfg: Option<&crate::config::EditorConfig>, campaign: &str, amount: f64) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let entries = vec![BudgetEntry { budget_name: campaign.to_string(), amount, status: "Enabled".to_string() }];
    let tmp = make_temp_csv()?;
    csv_writer::write_budget_csv(tmp.path(), &entries)?;
    let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
    print_editor_output(&output);
    println!("Updated budget to ${:.2} via CSV import.", amount);
    Ok(())
}
