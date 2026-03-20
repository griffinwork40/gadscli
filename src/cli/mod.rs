#![allow(dead_code)]

mod api_commands;
mod api_commands_ext;
mod api_commands_new;
mod editor_commands;

pub use api_commands::*;
pub use api_commands_ext::*;
pub use api_commands_new::*;
pub use editor_commands::*;

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
    /// Shared negative keyword list management
    #[command(name = "negative-list")]
    NegativeList {
        #[command(subcommand)]
        command: NegativeListCommands,
    },
    /// Device bid adjustment management
    Device {
        #[command(subcommand)]
        command: DeviceCommands,
    },
    /// Ad scheduling management
    Schedule {
        #[command(subcommand)]
        command: ScheduleCommands,
    },
    /// Location targeting management
    Location {
        #[command(subcommand)]
        command: LocationCommands,
    },
    /// Audience targeting management
    Audience {
        #[command(subcommand)]
        command: AudienceCommands,
    },
    /// Google Ads Editor automation (reads local SQLite, writes via CSV import)
    Editor {
        #[command(subcommand)]
        command: EditorCommands,
    },
}
