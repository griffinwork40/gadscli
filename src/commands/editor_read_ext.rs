#![allow(dead_code)]

use crate::cli::Cli;
use crate::config::Config;
use crate::editor::database::EditorDatabase;
use crate::error::Result;

use super::editor_cmd::{opt_id, resolve_customer_id, truncate};

pub fn handle_negative_keywords(cli: &Cli, config: &Config, campaign_id: Option<i64>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let nkws = db.list_negative_keywords(campaign_id)?;
    if nkws.is_empty() { println!("No negative keywords found."); return Ok(()); }

    println!("{:<10} {:<40} {:<10} {:<10} {:<10}", "LocalID", "Text", "Match", "Status", "State");
    println!("{}", "-".repeat(80));
    for nk in &nkws {
        println!("{:<10} {:<40} {:<10} {:<10} {:<10}", nk.local_id, truncate(&nk.text, 38), nk.match_type_str(), nk.status_str(), nk.state_str());
    }
    println!("\nTotal: {} negative keyword(s)", nkws.len());
    Ok(())
}

pub fn handle_bidding_strategies(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let strategies = db.list_bidding_strategies()?;
    if strategies.is_empty() { println!("No bidding strategies found."); return Ok(()); }

    println!("{:<10} {:<12} {:<30} {:<20} {:<10}", "LocalID", "RemoteID", "Name", "Type", "State");
    println!("{}", "-".repeat(82));
    for s in &strategies {
        println!("{:<10} {:<12} {:<30} {:<20} {:<10}", s.local_id, opt_id(s.remote_id), truncate(&s.name, 28), s.strategy_type_str(), s.state_str());
    }
    println!("\nTotal: {} strategy(ies)", strategies.len());
    Ok(())
}

pub fn handle_sitelinks(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let sitelinks = db.list_sitelinks()?;
    if sitelinks.is_empty() { println!("No sitelinks found."); return Ok(()); }

    println!("{:<10} {:<25} {:<40} {:<10}", "LocalID", "Text", "URL", "State");
    println!("{}", "-".repeat(85));
    for sl in &sitelinks {
        println!("{:<10} {:<25} {:<40} {:<10}", sl.local_id, truncate(&sl.link_text, 23), truncate(sl.final_urls.as_deref().unwrap_or("-"), 38), sl.state_str());
    }
    println!("\nTotal: {} sitelink(s)", sitelinks.len());
    Ok(())
}

pub fn handle_callouts(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let callouts = db.list_callouts()?;
    if callouts.is_empty() { println!("No callouts found."); return Ok(()); }

    println!("{:<10} {:<40} {:<10}", "LocalID", "Text", "State");
    println!("{}", "-".repeat(60));
    for co in &callouts {
        println!("{:<10} {:<40} {:<10}", co.local_id, truncate(&co.text, 38), co.state_str());
    }
    println!("\nTotal: {} callout(s)", callouts.len());
    Ok(())
}

pub fn handle_structured_snippets(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let snippets = db.list_structured_snippets()?;
    if snippets.is_empty() { println!("No structured snippets found."); return Ok(()); }

    println!("{:<10} {:<20} {:<50} {:<10}", "LocalID", "Header", "Values", "State");
    println!("{}", "-".repeat(90));
    for sn in &snippets {
        println!("{:<10} {:<20} {:<50} {:<10}", sn.local_id, truncate(&sn.header, 18), truncate(sn.values.as_deref().unwrap_or("-"), 48), sn.state_str());
    }
    println!("\nTotal: {} snippet(s)", snippets.len());
    Ok(())
}

pub fn handle_geo_targets(cli: &Cli, config: &Config, campaign_id: Option<i64>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let targets = db.list_geo_targets(campaign_id)?;
    if targets.is_empty() { println!("No geo targets found."); return Ok(()); }

    println!("{:<10} {:<15} {:<40} {:<10}", "LocalID", "LocationID", "Location Name", "State");
    println!("{}", "-".repeat(75));
    for gt in &targets {
        println!("{:<10} {:<15} {:<40} {:<10}", gt.local_id, gt.location_id.map(|i| i.to_string()).unwrap_or_else(|| "-".to_string()), truncate(gt.location_name.as_deref().unwrap_or("-"), 38), gt.state_str());
    }
    println!("\nTotal: {} geo target(s)", targets.len());
    Ok(())
}

pub fn handle_audiences(cli: &Cli, config: &Config, campaign_id: Option<i64>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let audiences = db.list_audiences(campaign_id)?;
    if audiences.is_empty() { println!("No audiences found."); return Ok(()); }

    println!("{:<10} {:<15} {:<40} {:<10}", "LocalID", "AudienceID", "Audience Name", "State");
    println!("{}", "-".repeat(75));
    for a in &audiences {
        println!("{:<10} {:<15} {:<40} {:<10}", a.local_id, a.audience_id.map(|i| i.to_string()).unwrap_or_else(|| "-".to_string()), truncate(a.audience_name.as_deref().unwrap_or("-"), 38), a.state_str());
    }
    println!("\nTotal: {} audience(s)", audiences.len());
    Ok(())
}

pub fn handle_placements(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let placements = db.list_placements()?;
    if placements.is_empty() { println!("No placements found."); return Ok(()); }

    println!("{:<10} {:<50} {:<10}", "LocalID", "URL", "State");
    println!("{}", "-".repeat(70));
    for p in &placements {
        println!("{:<10} {:<50} {:<10}", p.local_id, truncate(&p.url, 48), p.state_str());
    }
    println!("\nTotal: {} placement(s)", placements.len());
    Ok(())
}

pub fn handle_search_terms(cli: &Cli, config: &Config, ad_group_id: Option<i64>) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let terms = db.list_search_terms(ad_group_id)?;
    if terms.is_empty() { println!("No search terms found."); return Ok(()); }

    println!("{:<10} {:<40} {:<40}", "LocalID", "Search Term", "Keyword");
    println!("{}", "-".repeat(90));
    for t in &terms {
        println!("{:<10} {:<40} {:<40}", t.local_id, truncate(&t.search_term, 38), truncate(t.keyword_text.as_deref().unwrap_or("-"), 38));
    }
    println!("\nTotal: {} search term(s)", terms.len());
    Ok(())
}

pub fn handle_negative_keyword_lists(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let lists = db.list_negative_keyword_lists()?;
    if lists.is_empty() { println!("No negative keyword lists found."); return Ok(()); }

    println!("{:<10} {:<12} {:<40} {:<10}", "LocalID", "RemoteID", "Name", "State");
    println!("{}", "-".repeat(72));
    for l in &lists {
        println!("{:<10} {:<12} {:<40} {:<10}", l.local_id, opt_id(l.remote_id), truncate(&l.name, 38), l.state_str());
    }
    println!("\nTotal: {} list(s)", lists.len());
    Ok(())
}

pub fn handle_asset_groups(cli: &Cli, config: &Config) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let db = EditorDatabase::new(customer_id)?;
    let groups = db.list_asset_groups()?;
    if groups.is_empty() { println!("No asset groups found."); return Ok(()); }

    println!("{:<10} {:<12} {:<40} {:<10}", "LocalID", "RemoteID", "Name", "State");
    println!("{}", "-".repeat(72));
    for g in &groups {
        println!("{:<10} {:<12} {:<40} {:<10}", g.local_id, opt_id(g.remote_id), truncate(&g.name, 38), g.state_str());
    }
    println!("\nTotal: {} asset group(s)", groups.len());
    Ok(())
}
