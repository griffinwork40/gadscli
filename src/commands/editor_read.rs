#![allow(dead_code)]

use crate::cli::Cli;
use crate::config::Config;
use crate::editor::binary;
use crate::editor::database::EditorDatabase;
use crate::error::Result;

use super::editor_cmd::{campaign_type_str, opt_id, resolve_customer_id, truncate};

pub fn handle_status(cli: &Cli, config: &Config, ecfg: Option<&crate::config::EditorConfig>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    println!("Editor Status");
    println!("{}", "=".repeat(50));

    match binary::editor_binary_path(ecfg) {
        Ok(path) => {
            println!("Binary:   {}", path.display());
            match binary::editor_version(ecfg) {
                Ok(v) => println!("Version:  {}", v),
                Err(_) => println!("Version:  unknown"),
            }
        }
        Err(e) => println!("Binary:   NOT FOUND ({})", e),
    }

    let db_path = EditorDatabase::db_path(customer_id);
    if db_path.exists() {
        println!("Database: {}", db_path.display());
        if let Ok(db) = EditorDatabase::new(customer_id) {
            let changes = db.pending_changes().unwrap_or_default();
            println!("Pending:  {} change(s)", changes.len());
            if let Ok(settings) = db.get_account_settings() {
                println!("Account:  {}", settings.name.as_deref().unwrap_or("-"));
            }
        }
    } else {
        println!("Database: NOT FOUND (run 'editor download' first)");
    }

    if let Some(ref cfg) = config.editor {
        if let Some(ref email) = cfg.user_email {
            println!("Email:    {}", email);
        }
    }
    Ok(())
}

pub fn handle_campaigns(cli: &Cli, config: &Config, status: Option<&str>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let campaigns = db.list_campaigns()?;

    let filtered: Vec<_> = if let Some(s) = status {
        let s_lower = s.to_lowercase();
        campaigns.into_iter().filter(|c| c.status_str().to_lowercase() == s_lower).collect()
    } else {
        campaigns
    };

    if filtered.is_empty() {
        println!("No campaigns found.");
        return Ok(());
    }

    println!("{:<10} {:<12} {:<40} {:<10} {:<16} {:<12} {:<10}", "LocalID", "RemoteID", "Name", "Status", "Type", "Budget", "State");
    println!("{}", "-".repeat(110));
    for c in &filtered {
        println!("{:<10} {:<12} {:<40} {:<10} {:<16} ${:<11.2} {:<10}", c.local_id, opt_id(c.remote_id), truncate(&c.name, 38), c.status_str(), campaign_type_str(c.campaign_type), c.budget_dollars(), c.state_str());
    }
    println!("\nTotal: {} campaign(s)", filtered.len());
    Ok(())
}

pub fn handle_ad_groups(cli: &Cli, config: &Config, campaign_id: Option<i64>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let ad_groups = db.list_ad_groups(campaign_id)?;
    if ad_groups.is_empty() { println!("No ad groups found."); return Ok(()); }

    println!("{:<10} {:<12} {:<30} {:<30} {:<10} {:<10} {:<10}", "LocalID", "RemoteID", "Campaign", "Ad Group", "Status", "Max CPC", "State");
    println!("{}", "-".repeat(112));
    for (ag, campaign_name) in &ad_groups {
        let bid = ag.bid_dollars().map(|b| format!("${:.2}", b)).unwrap_or_else(|| "-".to_string());
        println!("{:<10} {:<12} {:<30} {:<30} {:<10} {:<10} {:<10}", ag.local_id, opt_id(ag.remote_id), truncate(campaign_name, 28), truncate(&ag.name, 28), ag.status_str(), bid, ag.state_str());
    }
    println!("\nTotal: {} ad group(s)", ad_groups.len());
    Ok(())
}

pub fn handle_keywords(cli: &Cli, config: &Config, ad_group_id: Option<i64>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let keywords = db.list_keywords(ad_group_id)?;
    if keywords.is_empty() { println!("No keywords found."); return Ok(()); }

    println!("{:<10} {:<25} {:<25} {:<30} {:<10} {:<10} {:<10} {:<10}", "LocalID", "Campaign", "Ad Group", "Keyword", "Match", "Status", "Max CPC", "State");
    println!("{}", "-".repeat(130));
    for (kw, ag_name, c_name) in &keywords {
        let bid = kw.bid_dollars().map(|b| format!("${:.2}", b)).unwrap_or_else(|| "-".to_string());
        println!("{:<10} {:<25} {:<25} {:<30} {:<10} {:<10} {:<10} {:<10}", kw.local_id, truncate(c_name, 23), truncate(ag_name, 23), truncate(&kw.text, 28), kw.match_type_str(), kw.status_str(), bid, kw.state_str());
    }
    println!("\nTotal: {} keyword(s)", keywords.len());
    Ok(())
}

pub fn handle_ads(cli: &Cli, config: &Config, ad_group_id: Option<i64>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let ads = db.list_ads(ad_group_id)?;
    if ads.is_empty() { println!("No ads found."); return Ok(()); }

    println!("{:<10} {:<25} {:<25} {:<40} {:<10} {:<10}", "LocalID", "Campaign", "Ad Group", "Headline 1", "Status", "State");
    println!("{}", "-".repeat(120));
    for (ad, ag_name, c_name) in &ads {
        let h1 = ad.headline1.as_deref().unwrap_or("-");
        println!("{:<10} {:<25} {:<25} {:<40} {:<10} {:<10}", ad.local_id, truncate(c_name, 23), truncate(ag_name, 23), truncate(h1, 38), ad.status_str(), ad.state_str());
    }
    println!("\nTotal: {} ad(s)", ads.len());
    Ok(())
}

pub fn handle_budgets(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let budgets = db.list_budgets()?;
    if budgets.is_empty() { println!("No budgets found."); return Ok(()); }

    println!("{:<10} {:<12} {:<30} {:<12} {:<10} {:<10}", "LocalID", "RemoteID", "Name", "Amount", "Status", "State");
    println!("{}", "-".repeat(84));
    for b in &budgets {
        println!("{:<10} {:<12} {:<30} ${:<11.2} {:<10} {:<10}", b.local_id, opt_id(b.remote_id), truncate(b.name.as_deref().unwrap_or("-"), 28), b.budget_dollars(), b.status_str(), b.state_str());
    }
    println!("\nTotal: {} budget(s)", budgets.len());
    Ok(())
}

pub fn handle_labels(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let labels = db.list_labels()?;
    if labels.is_empty() { println!("No labels found."); return Ok(()); }

    println!("{:<10} {:<12} {:<30} {:<30} {:<10} {:<10}", "LocalID", "RemoteID", "Name", "Description", "Color", "State");
    println!("{}", "-".repeat(102));
    for l in &labels {
        println!("{:<10} {:<12} {:<30} {:<30} {:<10} {:<10}", l.local_id, opt_id(l.remote_id), l.name, truncate(l.description.as_deref().unwrap_or("-"), 28), l.color.as_deref().unwrap_or("-"), l.state_str());
    }
    println!("\nTotal: {} label(s)", labels.len());
    Ok(())
}

pub fn handle_account(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let settings = db.get_account_settings()?;
    println!("Account Settings");
    println!("{}", "=".repeat(40));
    println!("Name:               {}", settings.name.as_deref().unwrap_or("-"));
    println!("Currency:            {}", settings.currency_code.as_deref().unwrap_or("-"));
    println!("Time Zone:           {}", settings.time_zone.as_deref().unwrap_or("-"));
    println!("Optimization Score:  {}", settings.optimization_score.map(|s| format!("{:.1}%", s * 100.0)).unwrap_or_else(|| "-".to_string()));
    Ok(())
}

pub fn handle_pending(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let changes = db.pending_changes()?;
    if changes.is_empty() { println!("No pending changes."); return Ok(()); }

    println!("{:<20} {:<10} {:<40} {:<10}", "Entity", "LocalID", "Name", "State");
    println!("{}", "-".repeat(80));
    for change in &changes {
        println!("{:<20} {:<10} {:<40} {:<10}", change.entity_type, change.local_id, truncate(&change.name, 38), change.state_str());
    }
    println!("\nTotal: {} pending change(s)", changes.len());
    Ok(())
}
