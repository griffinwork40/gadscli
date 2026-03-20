#![allow(dead_code)]

use crate::cli::{Cli, EditorCommands};
use crate::config::Config;
use crate::editor::types::*;
use crate::error::{GadsError, Result};

use super::editor_ops;
use super::editor_read;
use super::editor_read_ext;
use super::editor_write;

pub(crate) fn resolve_customer_id(cli: &Cli, config: &Config) -> Result<u64> {
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
        // Read commands
        EditorCommands::Status => editor_read::handle_status(cli, config, ecfg),
        EditorCommands::Campaigns { status } => editor_read::handle_campaigns(cli, config, status.as_deref()),
        EditorCommands::AdGroups { campaign_id } => editor_read::handle_ad_groups(cli, config, *campaign_id),
        EditorCommands::Keywords { ad_group_id, campaign_id: _ } => editor_read::handle_keywords(cli, config, *ad_group_id),
        EditorCommands::Ads { ad_group_id } => editor_read::handle_ads(cli, config, *ad_group_id),
        EditorCommands::Budgets => editor_read::handle_budgets(cli, config),
        EditorCommands::Labels => editor_read::handle_labels(cli, config),
        EditorCommands::Account => editor_read::handle_account(cli, config),
        EditorCommands::Pending => editor_read::handle_pending(cli, config),
        EditorCommands::NegativeKeywords { campaign_id } => editor_read_ext::handle_negative_keywords(cli, config, *campaign_id),
        EditorCommands::BiddingStrategies => editor_read_ext::handle_bidding_strategies(cli, config),
        EditorCommands::Sitelinks => editor_read_ext::handle_sitelinks(cli, config),
        EditorCommands::Callouts => editor_read_ext::handle_callouts(cli, config),
        EditorCommands::StructuredSnippets => editor_read_ext::handle_structured_snippets(cli, config),
        EditorCommands::GeoTargets { campaign_id } => editor_read_ext::handle_geo_targets(cli, config, *campaign_id),
        EditorCommands::Audiences { campaign_id } => editor_read_ext::handle_audiences(cli, config, *campaign_id),
        EditorCommands::Placements => editor_read_ext::handle_placements(cli, config),
        EditorCommands::SearchTerms { ad_group_id } => editor_read_ext::handle_search_terms(cli, config, *ad_group_id),
        EditorCommands::NegativeKeywordLists => editor_read_ext::handle_negative_keyword_lists(cli, config),
        EditorCommands::AssetGroups => editor_read_ext::handle_asset_groups(cli, config),

        // Binary operations
        EditorCommands::Download { user_email, campaign_names, campaign_remote_ids, download_type } => {
            editor_ops::handle_download(cli, config, ecfg, user_email, campaign_names, campaign_remote_ids, download_type.as_deref())
        }
        EditorCommands::Import { file } => editor_ops::handle_import(cli, config, ecfg, file),
        EditorCommands::Post { user_email } => editor_ops::handle_post(cli, config, ecfg, user_email),
        EditorCommands::Validate => editor_ops::handle_validate(cli, config, ecfg),
        EditorCommands::ExportXml { output, format } => editor_ops::handle_export_xml(cli, config, ecfg, output, format),
        EditorCommands::ImportXml { file } => editor_ops::handle_import_xml(cli, config, ecfg, file),
        EditorCommands::AcceptProposals => editor_ops::handle_accept_proposals(cli, config, ecfg),
        EditorCommands::ExportHtml { output } => editor_ops::handle_export_html(cli, config, ecfg, output),

        // Direct DB writes
        EditorCommands::AddKeywords { campaign, ad_group, keywords, match_type, bid } => {
            editor_write::handle_add_keywords(cli, config, campaign, ad_group, keywords, match_type, *bid)
        }
        EditorCommands::PauseKeyword { local_id } => editor_write::handle_pause_keyword(cli, config, *local_id),
        EditorCommands::EnableKeyword { local_id } => editor_write::handle_enable_keyword(cli, config, *local_id),
        EditorCommands::RemoveKeyword { local_id } => editor_write::handle_remove_keyword(cli, config, *local_id),
        EditorCommands::SetCampaignStatus { local_id, status } => {
            editor_write::handle_set_campaign_status(cli, config, *local_id, status)
        }
        EditorCommands::SetCampaignBudget { local_id, amount } => {
            editor_write::handle_set_campaign_budget(cli, config, *local_id, *amount)
        }

        // CSV import commands
        EditorCommands::AddAdGroups { campaign, ad_groups, bid } => {
            editor_write::handle_add_ad_groups(cli, config, ecfg, campaign, ad_groups, *bid)
        }
        EditorCommands::AddNegativeKeywords { campaign, ad_group, keywords, match_type } => {
            editor_write::handle_add_negative_keywords(cli, config, ecfg, campaign, ad_group, keywords, match_type)
        }
        EditorCommands::AddSitelinks { campaign, texts, urls } => {
            editor_write::handle_add_sitelinks(cli, config, ecfg, campaign, texts, urls)
        }
        EditorCommands::AddCallouts { campaign, texts } => {
            editor_write::handle_add_callouts(cli, config, ecfg, campaign, texts)
        }
        EditorCommands::AddLabels { names } => editor_write::handle_add_labels(cli, config, ecfg, names),
        EditorCommands::UpdateBudgets { campaign, amount } => {
            editor_write::handle_update_budgets(cli, config, ecfg, campaign, *amount)
        }
    }
}

pub(crate) fn print_editor_output(output: &EditorOutput) {
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

pub(crate) fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}...", &s[..max.saturating_sub(3)])
    } else {
        s.to_string()
    }
}

pub(crate) fn opt_id(id: Option<i64>) -> String {
    id.map(|i| i.to_string()).unwrap_or_else(|| "-".to_string())
}

pub(crate) fn campaign_type_str(ct: Option<i32>) -> &'static str {
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
