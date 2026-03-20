#![allow(dead_code)]

use std::path::Path;

use crate::cli::{Cli, EditorCommands};
use crate::config::Config;
use crate::editor::binary;
use crate::editor::csv_writer;
use crate::editor::database::{EditorDatabase, EditorDatabaseWriter};
use crate::editor::types::*;
use crate::error::{GadsError, Result};

fn resolve_customer_id(cli: &Cli, config: &Config) -> Result<u64> {
    let raw = cli
        .customer_id
        .clone()
        .or_else(|| config.customer_id.clone())
        .ok_or_else(|| {
            GadsError::Config(
                "Customer ID required. Use --customer-id or set GADS_CUSTOMER_ID.".to_string(),
            )
        })?;

    let cleaned: String = raw.chars().filter(|c| c.is_ascii_digit()).collect();
    cleaned.parse::<u64>().map_err(|_| {
        GadsError::Validation(format!("Invalid customer ID: {}", raw))
    })
}

fn editor_config(config: &Config) -> Option<&crate::config::EditorConfig> {
    config.editor.as_ref()
}

pub async fn handle(command: &EditorCommands, cli: &Cli, config: &Config) -> Result<()> {
    let ecfg = editor_config(config);

    match command {
        EditorCommands::Status => {
            let customer_id = resolve_customer_id(cli, config)?;
            println!("Editor Status");
            println!("{}", "=".repeat(50));

            // Binary check
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

            // Database check
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

            // Config email
            if let Some(ref cfg) = config.editor {
                if let Some(ref email) = cfg.user_email {
                    println!("Email:    {}", email);
                }
            }
        }

        EditorCommands::Campaigns { status } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let campaigns = db.list_campaigns()?;

            let filtered: Vec<_> = if let Some(s) = status {
                let s_lower = s.to_lowercase();
                campaigns
                    .into_iter()
                    .filter(|c| c.status_str().to_lowercase() == s_lower)
                    .collect()
            } else {
                campaigns
            };

            if filtered.is_empty() {
                println!("No campaigns found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<12} {:<40} {:<10} {:<16} {:<12} {:<10}",
                "LocalID", "RemoteID", "Name", "Status", "Type", "Budget", "State"
            );
            println!("{}", "-".repeat(110));

            for c in &filtered {
                let name = truncate(&c.name, 38);
                println!(
                    "{:<10} {:<12} {:<40} {:<10} {:<16} ${:<11.2} {:<10}",
                    c.local_id,
                    opt_id(c.remote_id),
                    name,
                    c.status_str(),
                    campaign_type_str(c.campaign_type),
                    c.budget_dollars(),
                    c.state_str(),
                );
            }
            println!("\nTotal: {} campaign(s)", filtered.len());
        }

        EditorCommands::AdGroups { campaign_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let ad_groups = db.list_ad_groups(*campaign_id)?;

            if ad_groups.is_empty() {
                println!("No ad groups found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<12} {:<30} {:<30} {:<10} {:<10} {:<10}",
                "LocalID", "RemoteID", "Campaign", "Ad Group", "Status", "Max CPC", "State"
            );
            println!("{}", "-".repeat(112));

            for (ag, campaign_name) in &ad_groups {
                let bid = ag
                    .bid_dollars()
                    .map(|b| format!("${:.2}", b))
                    .unwrap_or_else(|| "-".to_string());
                println!(
                    "{:<10} {:<12} {:<30} {:<30} {:<10} {:<10} {:<10}",
                    ag.local_id,
                    opt_id(ag.remote_id),
                    truncate(campaign_name, 28),
                    truncate(&ag.name, 28),
                    ag.status_str(),
                    bid,
                    ag.state_str(),
                );
            }
            println!("\nTotal: {} ad group(s)", ad_groups.len());
        }

        EditorCommands::Keywords {
            ad_group_id,
            campaign_id: _,
        } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let keywords = db.list_keywords(*ad_group_id)?;

            if keywords.is_empty() {
                println!("No keywords found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<25} {:<25} {:<30} {:<10} {:<10} {:<10} {:<10}",
                "LocalID", "Campaign", "Ad Group", "Keyword", "Match", "Status", "Max CPC", "State"
            );
            println!("{}", "-".repeat(130));

            for (kw, ag_name, c_name) in &keywords {
                let bid = kw
                    .bid_dollars()
                    .map(|b| format!("${:.2}", b))
                    .unwrap_or_else(|| "-".to_string());
                println!(
                    "{:<10} {:<25} {:<25} {:<30} {:<10} {:<10} {:<10} {:<10}",
                    kw.local_id,
                    truncate(c_name, 23),
                    truncate(ag_name, 23),
                    truncate(&kw.text, 28),
                    kw.match_type_str(),
                    kw.status_str(),
                    bid,
                    kw.state_str(),
                );
            }
            println!("\nTotal: {} keyword(s)", keywords.len());
        }

        EditorCommands::Ads { ad_group_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let ads = db.list_ads(*ad_group_id)?;

            if ads.is_empty() {
                println!("No ads found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<25} {:<25} {:<40} {:<10} {:<10}",
                "LocalID", "Campaign", "Ad Group", "Headline 1", "Status", "State"
            );
            println!("{}", "-".repeat(120));

            for (ad, ag_name, c_name) in &ads {
                let h1 = ad.headline1.as_deref().unwrap_or("-");
                println!(
                    "{:<10} {:<25} {:<25} {:<40} {:<10} {:<10}",
                    ad.local_id,
                    truncate(c_name, 23),
                    truncate(ag_name, 23),
                    truncate(h1, 38),
                    ad.status_str(),
                    ad.state_str(),
                );
            }
            println!("\nTotal: {} ad(s)", ads.len());
        }

        EditorCommands::Budgets => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let budgets = db.list_budgets()?;

            if budgets.is_empty() {
                println!("No budgets found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<12} {:<30} {:<12} {:<10} {:<10}",
                "LocalID", "RemoteID", "Name", "Amount", "Status", "State"
            );
            println!("{}", "-".repeat(84));

            for b in &budgets {
                let name = b.name.as_deref().unwrap_or("-");
                println!(
                    "{:<10} {:<12} {:<30} ${:<11.2} {:<10} {:<10}",
                    b.local_id,
                    opt_id(b.remote_id),
                    truncate(name, 28),
                    b.budget_dollars(),
                    b.status_str(),
                    b.state_str(),
                );
            }
            println!("\nTotal: {} budget(s)", budgets.len());
        }

        EditorCommands::Labels => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let labels = db.list_labels()?;

            if labels.is_empty() {
                println!("No labels found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<12} {:<30} {:<30} {:<10} {:<10}",
                "LocalID", "RemoteID", "Name", "Description", "Color", "State"
            );
            println!("{}", "-".repeat(102));

            for l in &labels {
                let desc = l.description.as_deref().unwrap_or("-");
                println!(
                    "{:<10} {:<12} {:<30} {:<30} {:<10} {:<10}",
                    l.local_id,
                    opt_id(l.remote_id),
                    l.name,
                    truncate(desc, 28),
                    l.color.as_deref().unwrap_or("-"),
                    l.state_str(),
                );
            }
            println!("\nTotal: {} label(s)", labels.len());
        }

        EditorCommands::Account => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let settings = db.get_account_settings()?;

            println!("Account Settings");
            println!("{}", "=".repeat(40));
            println!("Name:               {}", settings.name.as_deref().unwrap_or("-"));
            println!("Currency:            {}", settings.currency_code.as_deref().unwrap_or("-"));
            println!("Time Zone:           {}", settings.time_zone.as_deref().unwrap_or("-"));
            println!(
                "Optimization Score:  {}",
                settings
                    .optimization_score
                    .map(|s| format!("{:.1}%", s * 100.0))
                    .unwrap_or_else(|| "-".to_string())
            );
        }

        EditorCommands::Pending => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let changes = db.pending_changes()?;

            if changes.is_empty() {
                println!("No pending changes.");
                return Ok(());
            }

            println!(
                "{:<20} {:<10} {:<40} {:<10}",
                "Entity", "LocalID", "Name", "State"
            );
            println!("{}", "-".repeat(80));

            for change in &changes {
                println!(
                    "{:<20} {:<10} {:<40} {:<10}",
                    change.entity_type,
                    change.local_id,
                    truncate(&change.name, 38),
                    change.state_str(),
                );
            }
            println!("\nTotal: {} pending change(s)", changes.len());
        }

        EditorCommands::NegativeKeywords { campaign_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let nkws = db.list_negative_keywords(*campaign_id)?;

            if nkws.is_empty() {
                println!("No negative keywords found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<40} {:<10} {:<10} {:<10}",
                "LocalID", "Text", "Match", "Status", "State"
            );
            println!("{}", "-".repeat(80));

            for nk in &nkws {
                println!(
                    "{:<10} {:<40} {:<10} {:<10} {:<10}",
                    nk.local_id,
                    truncate(&nk.text, 38),
                    nk.match_type_str(),
                    nk.status_str(),
                    nk.state_str(),
                );
            }
            println!("\nTotal: {} negative keyword(s)", nkws.len());
        }

        EditorCommands::BiddingStrategies => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let strategies = db.list_bidding_strategies()?;

            if strategies.is_empty() {
                println!("No bidding strategies found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<12} {:<30} {:<20} {:<10}",
                "LocalID", "RemoteID", "Name", "Type", "State"
            );
            println!("{}", "-".repeat(82));

            for s in &strategies {
                println!(
                    "{:<10} {:<12} {:<30} {:<20} {:<10}",
                    s.local_id,
                    opt_id(s.remote_id),
                    truncate(&s.name, 28),
                    s.strategy_type_str(),
                    s.state_str(),
                );
            }
            println!("\nTotal: {} strategy(ies)", strategies.len());
        }

        EditorCommands::Sitelinks => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let sitelinks = db.list_sitelinks()?;

            if sitelinks.is_empty() {
                println!("No sitelinks found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<25} {:<40} {:<10}",
                "LocalID", "Text", "URL", "State"
            );
            println!("{}", "-".repeat(85));

            for sl in &sitelinks {
                println!(
                    "{:<10} {:<25} {:<40} {:<10}",
                    sl.local_id,
                    truncate(&sl.link_text, 23),
                    truncate(sl.final_urls.as_deref().unwrap_or("-"), 38),
                    sl.state_str(),
                );
            }
            println!("\nTotal: {} sitelink(s)", sitelinks.len());
        }

        EditorCommands::Callouts => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let callouts = db.list_callouts()?;

            if callouts.is_empty() {
                println!("No callouts found.");
                return Ok(());
            }

            println!("{:<10} {:<40} {:<10}", "LocalID", "Text", "State");
            println!("{}", "-".repeat(60));

            for co in &callouts {
                println!("{:<10} {:<40} {:<10}", co.local_id, truncate(&co.text, 38), co.state_str());
            }
            println!("\nTotal: {} callout(s)", callouts.len());
        }

        EditorCommands::StructuredSnippets => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let snippets = db.list_structured_snippets()?;

            if snippets.is_empty() {
                println!("No structured snippets found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<20} {:<50} {:<10}",
                "LocalID", "Header", "Values", "State"
            );
            println!("{}", "-".repeat(90));

            for sn in &snippets {
                println!(
                    "{:<10} {:<20} {:<50} {:<10}",
                    sn.local_id,
                    truncate(&sn.header, 18),
                    truncate(sn.values.as_deref().unwrap_or("-"), 48),
                    sn.state_str(),
                );
            }
            println!("\nTotal: {} snippet(s)", snippets.len());
        }

        EditorCommands::GeoTargets { campaign_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let targets = db.list_geo_targets(*campaign_id)?;

            if targets.is_empty() {
                println!("No geo targets found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<15} {:<40} {:<10}",
                "LocalID", "LocationID", "Location Name", "State"
            );
            println!("{}", "-".repeat(75));

            for gt in &targets {
                println!(
                    "{:<10} {:<15} {:<40} {:<10}",
                    gt.local_id,
                    gt.location_id.map(|i| i.to_string()).unwrap_or_else(|| "-".to_string()),
                    truncate(gt.location_name.as_deref().unwrap_or("-"), 38),
                    gt.state_str(),
                );
            }
            println!("\nTotal: {} geo target(s)", targets.len());
        }

        EditorCommands::Audiences { campaign_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let audiences = db.list_audiences(*campaign_id)?;

            if audiences.is_empty() {
                println!("No audiences found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<15} {:<40} {:<10}",
                "LocalID", "AudienceID", "Audience Name", "State"
            );
            println!("{}", "-".repeat(75));

            for a in &audiences {
                println!(
                    "{:<10} {:<15} {:<40} {:<10}",
                    a.local_id,
                    a.audience_id.map(|i| i.to_string()).unwrap_or_else(|| "-".to_string()),
                    truncate(a.audience_name.as_deref().unwrap_or("-"), 38),
                    a.state_str(),
                );
            }
            println!("\nTotal: {} audience(s)", audiences.len());
        }

        EditorCommands::Placements => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let placements = db.list_placements()?;

            if placements.is_empty() {
                println!("No placements found.");
                return Ok(());
            }

            println!("{:<10} {:<50} {:<10}", "LocalID", "URL", "State");
            println!("{}", "-".repeat(70));

            for p in &placements {
                println!(
                    "{:<10} {:<50} {:<10}",
                    p.local_id,
                    truncate(&p.url, 48),
                    p.state_str(),
                );
            }
            println!("\nTotal: {} placement(s)", placements.len());
        }

        EditorCommands::SearchTerms { ad_group_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let terms = db.list_search_terms(*ad_group_id)?;

            if terms.is_empty() {
                println!("No search terms found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<40} {:<40}",
                "LocalID", "Search Term", "Keyword"
            );
            println!("{}", "-".repeat(90));

            for t in &terms {
                println!(
                    "{:<10} {:<40} {:<40}",
                    t.local_id,
                    truncate(&t.search_term, 38),
                    truncate(t.keyword_text.as_deref().unwrap_or("-"), 38),
                );
            }
            println!("\nTotal: {} search term(s)", terms.len());
        }

        EditorCommands::NegativeKeywordLists => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let lists = db.list_negative_keyword_lists()?;

            if lists.is_empty() {
                println!("No negative keyword lists found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<12} {:<40} {:<10}",
                "LocalID", "RemoteID", "Name", "State"
            );
            println!("{}", "-".repeat(72));

            for l in &lists {
                println!(
                    "{:<10} {:<12} {:<40} {:<10}",
                    l.local_id,
                    opt_id(l.remote_id),
                    truncate(&l.name, 38),
                    l.state_str(),
                );
            }
            println!("\nTotal: {} list(s)", lists.len());
        }

        EditorCommands::AssetGroups => {
            let customer_id = resolve_customer_id(cli, config)?;
            let db = EditorDatabase::new(customer_id)?;
            let groups = db.list_asset_groups()?;

            if groups.is_empty() {
                println!("No asset groups found.");
                return Ok(());
            }

            println!(
                "{:<10} {:<12} {:<40} {:<10}",
                "LocalID", "RemoteID", "Name", "State"
            );
            println!("{}", "-".repeat(72));

            for g in &groups {
                println!(
                    "{:<10} {:<12} {:<40} {:<10}",
                    g.local_id,
                    opt_id(g.remote_id),
                    truncate(&g.name, 38),
                    g.state_str(),
                );
            }
            println!("\nTotal: {} asset group(s)", groups.len());
        }

        // --- Binary operations ---

        EditorCommands::Download { user_email, campaign_names, campaign_remote_ids, download_type } => {
            let customer_id = resolve_customer_id(cli, config)?;
            println!("Downloading account data for customer {}...", customer_id);
            let output = binary::download(
                customer_id,
                user_email,
                campaign_names,
                campaign_remote_ids,
                download_type.as_deref(),
                None,
                ecfg,
            )?;
            print_editor_output(&output);
        }

        EditorCommands::Import { file } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let path = Path::new(file);
            println!("Importing CSV file: {}", path.display());
            let output = binary::import_csv(customer_id, path, None, ecfg)?;
            print_editor_output(&output);
        }

        EditorCommands::Post { user_email } => {
            let customer_id = resolve_customer_id(cli, config)?;
            println!("Posting pending changes for customer {}...", customer_id);
            let output = binary::post(customer_id, user_email, None, ecfg)?;
            print_editor_output(&output);
        }

        EditorCommands::Validate => {
            let customer_id = resolve_customer_id(cli, config)?;
            println!("Validating pending changes for customer {}...", customer_id);
            let output = binary::validate(customer_id, None, ecfg)?;
            print_editor_output(&output);
        }

        EditorCommands::ExportXml { output, format } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let xml_format = match format.to_lowercase().as_str() {
                "share" => XmlExportFormat::Share,
                "upgrade" => XmlExportFormat::Upgrade,
                _ => XmlExportFormat::Standard,
            };
            let path = Path::new(output);
            println!("Exporting XML to {}...", path.display());
            let result = binary::export_xml(customer_id, path, &xml_format, ecfg)?;
            print_editor_output(&result);
        }

        EditorCommands::ImportXml { file } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let path = Path::new(file);
            println!("Importing XML from {}...", path.display());
            let output = binary::import_xml(customer_id, path, None, ecfg)?;
            print_editor_output(&output);
        }

        EditorCommands::AcceptProposals => {
            let customer_id = resolve_customer_id(cli, config)?;
            println!("Accepting proposals for customer {}...", customer_id);
            let output = binary::accept_proposals(customer_id, None, ecfg)?;
            print_editor_output(&output);
        }

        EditorCommands::ExportHtml { output } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let path = Path::new(output);
            println!("Exporting HTML to {}...", path.display());
            let result = binary::export_html(customer_id, path, ecfg)?;
            print_editor_output(&result);
        }

        // --- Direct DB writes ---

        EditorCommands::AddKeywords {
            campaign,
            ad_group,
            keywords,
            match_type,
            bid,
        } => {
            let customer_id = resolve_customer_id(cli, config)?;
            if keywords.is_empty() {
                println!("No keywords specified.");
                return Ok(());
            }

            let writer = EditorDatabaseWriter::new(customer_id)?;
            let ag_local_id = writer
                .find_ad_group(campaign, ad_group)?
                .ok_or_else(|| {
                    GadsError::Validation(format!(
                        "Ad group '{}' not found in campaign '{}'",
                        ad_group, campaign
                    ))
                })?;

            let criterion_type = match match_type.to_lowercase().as_str() {
                "exact" => 1,
                "phrase" => 2,
                _ => 0,
            };

            let max_cpc_micros = bid.map(|b| (b * 1_000_000.0) as i64).unwrap_or(0);

            let mut added = 0;
            for kw in keywords {
                let local_id =
                    writer.add_keyword(ag_local_id, kw, criterion_type, max_cpc_micros)?;
                println!("  Added keyword '{}' (localId: {})", kw, local_id);
                added += 1;
            }

            println!("\nAdded {} keyword(s). Run 'editor post' to push to Google.", added);
        }

        EditorCommands::PauseKeyword { local_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let writer = EditorDatabaseWriter::new(customer_id)?;
            writer.pause_keyword(*local_id)?;
            println!("Keyword {} paused. Run 'editor post' to push to Google.", local_id);
        }

        EditorCommands::EnableKeyword { local_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let writer = EditorDatabaseWriter::new(customer_id)?;
            writer.enable_keyword(*local_id)?;
            println!("Keyword {} enabled. Run 'editor post' to push to Google.", local_id);
        }

        EditorCommands::RemoveKeyword { local_id } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let writer = EditorDatabaseWriter::new(customer_id)?;
            writer.remove_keyword(*local_id)?;
            println!("Keyword {} marked for removal. Run 'editor post' to push to Google.", local_id);
        }

        EditorCommands::SetCampaignStatus { local_id, status } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let status_code = match status.to_lowercase().as_str() {
                "enabled" => 2,
                "paused" => 3,
                "removed" => 4,
                _ => {
                    return Err(GadsError::Validation(format!(
                        "Invalid status '{}'. Use: enabled, paused, removed",
                        status
                    )));
                }
            };
            let writer = EditorDatabaseWriter::new(customer_id)?;
            writer.set_campaign_status(*local_id, status_code)?;
            println!(
                "Campaign {} set to {}. Run 'editor post' to push to Google.",
                local_id, status
            );
        }

        EditorCommands::SetCampaignBudget { local_id, amount } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let micros = (*amount * 1_000_000.0) as i64;
            let writer = EditorDatabaseWriter::new(customer_id)?;
            writer.set_campaign_budget(*local_id, micros)?;
            println!(
                "Campaign {} budget set to ${:.2}. Run 'editor post' to push to Google.",
                local_id, amount
            );
        }

        // --- CSV import commands ---

        EditorCommands::AddAdGroups {
            campaign,
            ad_groups,
            bid,
        } => {
            let customer_id = resolve_customer_id(cli, config)?;
            if ad_groups.is_empty() {
                println!("No ad groups specified.");
                return Ok(());
            }

            let entries: Vec<AdGroupEntry> = ad_groups
                .iter()
                .map(|ag| AdGroupEntry {
                    campaign: campaign.clone(),
                    ad_group: ag.clone(),
                    max_cpc: *bid,
                    status: "Enabled".to_string(),
                })
                .collect();

            let tmp = tempfile::NamedTempFile::with_suffix(".csv")
                .map_err(|e| GadsError::Other(format!("Failed to create temp file: {}", e)))?;
            csv_writer::write_ad_group_csv(tmp.path(), &entries)?;
            let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
            print_editor_output(&output);
            println!("Added {} ad group(s) via CSV import.", entries.len());
        }

        EditorCommands::AddNegativeKeywords {
            campaign,
            ad_group,
            keywords,
            match_type,
        } => {
            let customer_id = resolve_customer_id(cli, config)?;
            if keywords.is_empty() {
                println!("No negative keywords specified.");
                return Ok(());
            }

            let entries: Vec<NegativeKeywordEntry> = keywords
                .iter()
                .map(|kw| NegativeKeywordEntry {
                    campaign: campaign.clone(),
                    ad_group: ad_group.clone(),
                    keyword: kw.clone(),
                    match_type: match_type.clone(),
                })
                .collect();

            let tmp = tempfile::NamedTempFile::with_suffix(".csv")
                .map_err(|e| GadsError::Other(format!("Failed to create temp file: {}", e)))?;
            csv_writer::write_negative_keyword_csv(tmp.path(), &entries)?;
            let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
            print_editor_output(&output);
            println!("Added {} negative keyword(s) via CSV import.", entries.len());
        }

        EditorCommands::AddSitelinks {
            campaign,
            texts,
            urls,
        } => {
            let customer_id = resolve_customer_id(cli, config)?;
            if texts.is_empty() || urls.is_empty() {
                println!("Both --texts and --urls are required.");
                return Ok(());
            }
            if texts.len() != urls.len() {
                return Err(GadsError::Validation(
                    "Number of --texts must match number of --urls".to_string(),
                ));
            }

            let entries: Vec<SitelinkEntry> = texts
                .iter()
                .zip(urls.iter())
                .map(|(text, url)| SitelinkEntry {
                    campaign: campaign.clone(),
                    ad_group: None,
                    sitelink_text: text.clone(),
                    final_url: url.clone(),
                    description1: None,
                    description2: None,
                })
                .collect();

            let tmp = tempfile::NamedTempFile::with_suffix(".csv")
                .map_err(|e| GadsError::Other(format!("Failed to create temp file: {}", e)))?;
            csv_writer::write_sitelink_csv(tmp.path(), &entries)?;
            let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
            print_editor_output(&output);
            println!("Added {} sitelink(s) via CSV import.", entries.len());
        }

        EditorCommands::AddCallouts { campaign, texts } => {
            let customer_id = resolve_customer_id(cli, config)?;
            if texts.is_empty() {
                println!("No callout texts specified.");
                return Ok(());
            }

            let entries: Vec<CalloutEntry> = texts
                .iter()
                .map(|t| CalloutEntry {
                    campaign: campaign.clone(),
                    ad_group: None,
                    callout_text: t.clone(),
                })
                .collect();

            let tmp = tempfile::NamedTempFile::with_suffix(".csv")
                .map_err(|e| GadsError::Other(format!("Failed to create temp file: {}", e)))?;
            csv_writer::write_callout_csv(tmp.path(), &entries)?;
            let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
            print_editor_output(&output);
            println!("Added {} callout(s) via CSV import.", entries.len());
        }

        EditorCommands::AddLabels { names } => {
            let customer_id = resolve_customer_id(cli, config)?;
            if names.is_empty() {
                println!("No label names specified.");
                return Ok(());
            }

            let entries: Vec<LabelEntry> = names
                .iter()
                .map(|n| LabelEntry {
                    label_name: n.clone(),
                    description: None,
                    color: None,
                })
                .collect();

            let tmp = tempfile::NamedTempFile::with_suffix(".csv")
                .map_err(|e| GadsError::Other(format!("Failed to create temp file: {}", e)))?;
            csv_writer::write_label_csv(tmp.path(), &entries)?;
            let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
            print_editor_output(&output);
            println!("Added {} label(s) via CSV import.", entries.len());
        }

        EditorCommands::UpdateBudgets { campaign, amount } => {
            let customer_id = resolve_customer_id(cli, config)?;
            let entries = vec![BudgetEntry {
                budget_name: campaign.clone(),
                amount: *amount,
                status: "Enabled".to_string(),
            }];

            let tmp = tempfile::NamedTempFile::with_suffix(".csv")
                .map_err(|e| GadsError::Other(format!("Failed to create temp file: {}", e)))?;
            csv_writer::write_budget_csv(tmp.path(), &entries)?;
            let output = binary::import_csv(customer_id, tmp.path(), None, ecfg)?;
            print_editor_output(&output);
            println!("Updated budget to ${:.2} via CSV import.", amount);
        }
    }

    Ok(())
}

fn print_editor_output(output: &EditorOutput) {
    if !output.stdout.is_empty() {
        println!("{}", output.stdout);
    }
    if !output.stderr.is_empty() {
        eprintln!("{}", output.stderr);
    }
    if output.success {
        println!("Completed successfully.");
    } else {
        eprintln!(
            "Warning: Editor exited with code {:?}. Check output above.",
            output.exit_code
        );
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

fn opt_id(id: Option<i64>) -> String {
    id.map(|i| i.to_string()).unwrap_or_else(|| "-".to_string())
}

fn campaign_type_str(ct: Option<i32>) -> &'static str {
    match ct {
        Some(0) => "Search",
        Some(1) => "Display",
        Some(2) => "Shopping",
        Some(3) => "Video",
        Some(6) => "PMax",
        Some(7) => "Local",
        Some(8) => "Smart",
        Some(9) => "DemandGen",
        _ => "-",
    }
}
