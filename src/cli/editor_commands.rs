#![allow(dead_code)]

use clap::Subcommand;

// Editor
#[derive(Subcommand)]
pub enum EditorCommands {
    /// Show Editor status: binary, version, database, pending changes
    Status,
    /// List campaigns from Editor's local database
    Campaigns {
        #[arg(long)]
        status: Option<String>,
    },
    /// List ad groups from Editor's local database
    AdGroups {
        #[arg(long)]
        campaign_id: Option<i64>,
    },
    /// List keywords from Editor's local database
    Keywords {
        #[arg(long)]
        ad_group_id: Option<i64>,
        #[arg(long)]
        campaign_id: Option<i64>,
    },
    /// List ads (RSAs) from Editor's local database
    Ads {
        #[arg(long)]
        ad_group_id: Option<i64>,
    },
    /// List budgets from Editor's local database
    Budgets,
    /// List labels from Editor's local database
    Labels,
    /// Show account settings from Editor's local database
    Account,
    /// Show pending (unposted) changes
    Pending,
    /// List negative keywords from Editor's local database
    NegativeKeywords {
        #[arg(long)]
        campaign_id: Option<i64>,
    },
    /// List bidding strategies from Editor's local database
    BiddingStrategies,
    /// List sitelinks from Editor's local database
    Sitelinks,
    /// List callouts from Editor's local database
    Callouts,
    /// List structured snippets from Editor's local database
    StructuredSnippets,
    /// List geo targets from Editor's local database
    GeoTargets {
        #[arg(long)]
        campaign_id: Option<i64>,
    },
    /// List audiences from Editor's local database
    Audiences {
        #[arg(long)]
        campaign_id: Option<i64>,
    },
    /// List placements from Editor's local database
    Placements,
    /// List search terms from Editor's local database
    SearchTerms {
        #[arg(long)]
        ad_group_id: Option<i64>,
    },
    /// List negative keyword lists from Editor's local database
    NegativeKeywordLists,
    /// List asset groups from Editor's local database
    AssetGroups,
    /// Download fresh account data from Google
    Download {
        /// Email address for authentication
        #[arg(long)]
        user_email: String,
        /// Only download these campaign names (space-separated)
        #[arg(long, num_args = 1..)]
        campaign_names: Vec<String>,
        /// Only download these campaign remote IDs (space-separated)
        #[arg(long, num_args = 1..)]
        campaign_remote_ids: Vec<String>,
        /// Download type: full, fullmerge, or grc (default: full)
        #[arg(long)]
        download_type: Option<String>,
    },
    /// Import a CSV file into Editor
    Import {
        /// Path to the CSV file
        file: String,
    },
    /// Post pending changes to Google
    Post {
        /// Email address for authentication
        #[arg(long)]
        user_email: String,
    },
    /// Validate pending changes before posting
    Validate,
    /// Export account data as XML
    ExportXml {
        /// Output file path
        #[arg(long)]
        output: String,
        /// XML format: standard, share, or upgrade
        #[arg(long, default_value = "standard")]
        format: String,
    },
    /// Import changes from an XML file
    ImportXml {
        /// Path to the XML file
        file: String,
    },
    /// Accept pending proposals/recommendations
    AcceptProposals,
    /// Export account data as HTML report
    ExportHtml {
        /// Output file path
        #[arg(long)]
        output: String,
    },
    /// Add keywords via direct database write
    AddKeywords {
        #[arg(long)]
        campaign: String,
        #[arg(long)]
        ad_group: String,
        #[arg(long, num_args = 1..)]
        keywords: Vec<String>,
        #[arg(long, default_value = "Broad")]
        match_type: String,
        #[arg(long)]
        bid: Option<f64>,
    },
    /// Pause a keyword by local ID
    PauseKeyword {
        /// Local ID of the keyword
        local_id: i64,
    },
    /// Enable a keyword by local ID
    EnableKeyword {
        /// Local ID of the keyword
        local_id: i64,
    },
    /// Remove a keyword by local ID
    RemoveKeyword {
        /// Local ID of the keyword
        local_id: i64,
    },
    /// Set a campaign's status by local ID
    SetCampaignStatus {
        /// Local ID of the campaign
        local_id: i64,
        /// Status: enabled, paused, or removed
        #[arg(long)]
        status: String,
    },
    /// Set a campaign's budget by local ID
    SetCampaignBudget {
        /// Local ID of the campaign
        local_id: i64,
        /// Budget amount in dollars
        #[arg(long)]
        amount: f64,
    },
    /// Add ad groups via CSV import
    AddAdGroups {
        #[arg(long)]
        campaign: String,
        #[arg(long, num_args = 1..)]
        ad_groups: Vec<String>,
        #[arg(long)]
        bid: Option<f64>,
    },
    /// Add negative keywords via CSV import
    AddNegativeKeywords {
        #[arg(long)]
        campaign: String,
        #[arg(long)]
        ad_group: Option<String>,
        #[arg(long, num_args = 1..)]
        keywords: Vec<String>,
        #[arg(long, default_value = "Broad")]
        match_type: String,
    },
    /// Add sitelinks via CSV import
    AddSitelinks {
        #[arg(long)]
        campaign: String,
        #[arg(long, num_args = 1..)]
        texts: Vec<String>,
        #[arg(long, num_args = 1..)]
        urls: Vec<String>,
    },
    /// Add callouts via CSV import
    AddCallouts {
        #[arg(long)]
        campaign: String,
        #[arg(long, num_args = 1..)]
        texts: Vec<String>,
    },
    /// Add labels via CSV import
    AddLabels {
        #[arg(long, num_args = 1..)]
        names: Vec<String>,
    },
    /// Update budgets via CSV import
    UpdateBudgets {
        #[arg(long)]
        campaign: String,
        #[arg(long)]
        amount: f64,
    },
}
