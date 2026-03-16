#![allow(dead_code)]

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(name = "gadscli", about = "Google Ads CLI", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Customer ID (without hyphens)
    #[arg(long, global = true, env = "GADS_CUSTOMER_ID")]
    pub customer_id: Option<String>,

    /// Login customer ID for MCC accounts
    #[arg(long, global = true, env = "GADS_LOGIN_CUSTOMER_ID")]
    pub login_customer_id: Option<String>,

    /// Output format
    #[arg(long, global = true, default_value = "table", value_enum)]
    pub format: FormatOption,

    /// Validate only, don't execute mutations
    #[arg(long, global = true)]
    pub dry_run: bool,

    /// Verbose output
    #[arg(long, short, global = true)]
    pub verbose: bool,

    /// Suppress non-essential output
    #[arg(long, short, global = true)]
    pub quiet: bool,

    /// API version
    #[arg(long, global = true)]
    pub api_version: Option<String>,

    /// Page size for list operations
    #[arg(long, global = true)]
    pub page_size: Option<i32>,

    /// Fetch all pages automatically
    #[arg(long, global = true)]
    pub page_all: bool,

    /// Config profile to use
    #[arg(long, global = true)]
    pub profile: Option<String>,
}

#[derive(Clone, ValueEnum)]
pub enum FormatOption {
    Json,
    Table,
    Csv,
    Yaml,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Authentication management
    Auth {
        #[command(subcommand)]
        command: AuthCommands,
    },
    /// Configuration management
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
    /// Account management
    Account {
        #[command(subcommand)]
        command: AccountCommands,
    },
    /// Campaign management
    Campaign {
        #[command(subcommand)]
        command: CampaignCommands,
    },
    /// Ad group management
    #[command(name = "ad-group")]
    AdGroup {
        #[command(subcommand)]
        command: AdGroupCommands,
    },
    /// Ad management
    Ad {
        #[command(subcommand)]
        command: AdCommands,
    },
    /// Keyword management
    Keyword {
        #[command(subcommand)]
        command: KeywordCommands,
    },
    /// Budget management
    Budget {
        #[command(subcommand)]
        command: BudgetCommands,
    },
    /// Bidding strategy management
    Bidding {
        #[command(subcommand)]
        command: BiddingCommands,
    },
    /// Run GAQL reports
    Report {
        #[command(subcommand)]
        command: ReportCommands,
    },
    /// Asset management
    Asset {
        #[command(subcommand)]
        command: AssetCommands,
    },
    /// Conversion action management
    Conversion {
        #[command(subcommand)]
        command: ConversionCommands,
    },
    /// Label management
    Label {
        #[command(subcommand)]
        command: LabelCommands,
    },
    /// Recommendation management
    Recommendation {
        #[command(subcommand)]
        command: RecommendationCommands,
    },
    /// Batch job management
    Batch {
        #[command(subcommand)]
        command: BatchCommands,
    },
    /// Field metadata queries
    Field {
        #[command(subcommand)]
        command: FieldCommands,
    },
}

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

// Bidding
#[derive(Subcommand)]
pub enum BiddingCommands {
    List,
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        strategy_type: String,
        #[arg(long)]
        target_cpa_micros: Option<i64>,
        #[arg(long)]
        target_roas: Option<f64>,
    },
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        target_cpa_micros: Option<i64>,
        #[arg(long)]
        target_roas: Option<f64>,
    },
    Remove {
        id: String,
    },
}

// Report
#[derive(Subcommand)]
pub enum ReportCommands {
    /// Execute a raw GAQL query
    Query {
        /// GAQL query string
        query: String,
        #[arg(long)]
        date_range: Option<String>,
        #[arg(long)]
        start_date: Option<String>,
        #[arg(long)]
        end_date: Option<String>,
    },
    /// Run a pre-built report template
    Run {
        /// Template name
        template: String,
        #[arg(long)]
        date_range: Option<String>,
    },
    /// List available report templates
    Templates,
}

// Asset
#[derive(Subcommand)]
pub enum AssetCommands {
    List {
        #[arg(long)]
        asset_type: Option<String>,
    },
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        asset_type: String,
        #[arg(long)]
        text_content: Option<String>,
    },
    Remove {
        id: String,
    },
}

// Conversion
#[derive(Subcommand)]
pub enum ConversionCommands {
    List,
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long, default_value = "WEBPAGE")]
        action_type: String,
        #[arg(long)]
        category: Option<String>,
    },
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        status: Option<String>,
    },
    Upload {
        #[arg(long)]
        conversion_action_id: String,
        #[arg(long)]
        gclid: String,
        #[arg(long)]
        conversion_date_time: String,
        #[arg(long)]
        conversion_value: Option<f64>,
    },
}

// Label
#[derive(Subcommand)]
pub enum LabelCommands {
    List,
    Get {
        id: String,
    },
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        color: Option<String>,
    },
    Update {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        description: Option<String>,
    },
    Remove {
        id: String,
    },
    /// Assign a label to a resource
    Assign {
        #[arg(long)]
        label_id: String,
        #[arg(long)]
        resource_type: String,
        #[arg(long)]
        resource_id: String,
    },
}

// Recommendation
#[derive(Subcommand)]
pub enum RecommendationCommands {
    List {
        #[arg(long)]
        recommendation_type: Option<String>,
    },
    Apply {
        id: String,
    },
    Dismiss {
        id: String,
    },
}

// Batch
#[derive(Subcommand)]
pub enum BatchCommands {
    Create,
    Run {
        id: String,
    },
    Status {
        id: String,
    },
    Results {
        id: String,
    },
}

// Field
#[derive(Subcommand)]
pub enum FieldCommands {
    /// Search for field metadata
    Search { resource: String },
    /// Show fields for a resource type
    List { resource: String },
}
