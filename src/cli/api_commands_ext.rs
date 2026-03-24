#![allow(dead_code)]

use clap::Subcommand;

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
    /// Link an asset to a campaign
    Link {
        #[arg(long)]
        campaign_id: String,
        #[arg(long)]
        asset_id: String,
        /// Field type: SITELINK, CALLOUT, STRUCTURED_SNIPPET, etc.
        #[arg(long)]
        field_type: String,
    },
    /// Unlink an asset from a campaign
    Unlink {
        /// Campaign asset resource name or ID
        id: String,
    },
    /// List assets linked to a campaign
    ListLinked {
        #[arg(long)]
        campaign_id: String,
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
    /// Create a new batch job
    Create,
    /// Add operations to a batch job
    AddOperations {
        #[arg(long)]
        job_id: String,
        /// JSON file containing operations array
        #[arg(long)]
        file: Option<String>,
        /// Inline JSON operations
        #[arg(long)]
        json: Option<String>,
    },
    /// Run a batch job
    Run { id: String },
    /// Check batch job status
    Status { id: String },
    /// Get batch job results
    Results { id: String },
    /// Wait for batch job completion with polling
    Wait {
        id: String,
        /// Timeout in seconds
        #[arg(long, default_value = "300")]
        timeout_secs: u64,
        /// Poll interval in seconds
        #[arg(long, default_value = "5")]
        poll_interval_secs: u64,
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

