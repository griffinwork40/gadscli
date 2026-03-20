#![allow(dead_code)]

use clap::Subcommand;

// Auth
#[derive(Subcommand)]
pub enum AuthCommands {
    /// Login with OAuth2
    Login,
    /// Logout and clear credentials
    Logout,
    /// Show authentication status
    Status,
    /// Show current authenticated user info
    Whoami,
}

// Config
#[derive(Subcommand)]
pub enum ConfigCommands {
    /// Set a config value
    Set { key: String, value: String },
    /// Get a config value
    Get { key: String },
    /// List all config values
    List,
    /// Initialize config interactively
    Init,
}

// Account
#[derive(Subcommand)]
pub enum AccountCommands {
    /// List accessible accounts
    List,
    /// Show account info
    Info,
    /// Show account hierarchy
    Hierarchy,
}

// Campaign
#[derive(Subcommand)]
pub enum CampaignCommands {
    List {
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        limit: Option<i32>,
    },
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        budget_id: String,
        #[arg(long, default_value = "SEARCH")]
        campaign_type: String,
        #[arg(long, default_value = "MANUAL_CPC")]
        bidding_strategy: String,
    },
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    Pause {
        id: String,
    },
    Enable {
        id: String,
    },
    Remove {
        id: String,
    },
}

// AdGroup
#[derive(Subcommand)]
pub enum AdGroupCommands {
    List {
        #[arg(long)]
        campaign_id: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        campaign_id: String,
        #[arg(long)]
        name: String,
        #[arg(long)]
        cpc_bid_micros: Option<i64>,
    },
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        cpc_bid_micros: Option<i64>,
    },
    Pause {
        id: String,
    },
    Enable {
        id: String,
    },
    Remove {
        id: String,
    },
}

// Ad
#[derive(Subcommand)]
pub enum AdCommands {
    List {
        #[arg(long)]
        ad_group_id: Option<String>,
        #[arg(long)]
        campaign_id: Option<String>,
    },
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        ad_group_id: String,
        #[arg(long, num_args = 1..)]
        headlines: Vec<String>,
        #[arg(long, num_args = 1..)]
        descriptions: Vec<String>,
        #[arg(long)]
        final_url: String,
        /// Pin headlines: "Text:1" pins to HEADLINE_1
        #[arg(long, num_args = 0..)]
        pin_headline: Vec<String>,
        /// Pin descriptions: "Text:1" pins to DESCRIPTION_1
        #[arg(long, num_args = 0..)]
        pin_description: Vec<String>,
    },
    Pause {
        id: String,
    },
    Enable {
        id: String,
    },
    Remove {
        id: String,
    },
}

// Keyword
#[derive(Subcommand)]
pub enum KeywordCommands {
    List {
        #[arg(long)]
        ad_group_id: Option<String>,
        #[arg(long)]
        campaign_id: Option<String>,
    },
    Add {
        #[arg(long)]
        ad_group_id: String,
        #[arg(long)]
        text: String,
        #[arg(long, default_value = "BROAD")]
        match_type: String,
        #[arg(long)]
        cpc_bid_micros: Option<i64>,
        /// Add as a negative keyword
        #[arg(long)]
        negative: bool,
    },
    Remove {
        id: String,
    },
    Update {
        id: String,
        #[arg(long)]
        status: Option<String>,
        #[arg(long)]
        cpc_bid_micros: Option<i64>,
    },
    /// Add a campaign-level negative keyword
    AddNegative {
        #[arg(long)]
        campaign_id: String,
        #[arg(long)]
        text: String,
        #[arg(long, default_value = "BROAD")]
        match_type: String,
    },
    /// List negative keywords
    ListNegatives {
        #[arg(long)]
        ad_group_id: Option<String>,
        #[arg(long)]
        campaign_id: Option<String>,
    },
    /// Remove a negative keyword
    RemoveNegative {
        id: String,
    },
    /// Add multiple keywords at once
    AddBulk {
        #[arg(long)]
        ad_group_id: String,
        /// Keywords to add (space-separated)
        #[arg(long, num_args = 1..)]
        keywords: Vec<String>,
        #[arg(long, default_value = "BROAD")]
        match_type: String,
        #[arg(long)]
        cpc_bid_micros: Option<i64>,
    },
    /// Exclude search terms as campaign negatives based on cost/conversion filters
    ExcludeTerms {
        #[arg(long)]
        campaign_id: String,
        /// Minimum cost in micros to include
        #[arg(long)]
        min_cost_micros: Option<i64>,
        /// Maximum conversions to include (terms with fewer conversions are excluded)
        #[arg(long)]
        max_conversions: Option<f64>,
    },
    /// Generate keyword ideas
    Ideas {
        /// Seed keywords
        #[arg(long, num_args = 0..)]
        text: Vec<String>,
        /// Seed URL
        #[arg(long)]
        url: Option<String>,
        /// Language resource (e.g., "languageConstants/1000" for English)
        #[arg(long)]
        language: Option<String>,
        /// Geo target IDs
        #[arg(long, num_args = 0..)]
        geo_ids: Vec<String>,
    },
}

// Budget
#[derive(Subcommand)]
pub enum BudgetCommands {
    List,
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        amount_micros: i64,
    },
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        amount_micros: Option<i64>,
    },
    Remove {
        id: String,
    },
}



