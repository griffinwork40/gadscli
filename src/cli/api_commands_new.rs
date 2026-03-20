#![allow(dead_code)]

use clap::Subcommand;

// Device
#[derive(Subcommand)]
pub enum DeviceCommands {
    /// List device bid adjustments for a campaign
    List {
        #[arg(long)]
        campaign_id: String,
    },
    /// Set device bid adjustment
    Set {
        #[arg(long)]
        campaign_id: String,
        /// Device type: MOBILE, DESKTOP, TABLET, CONNECTED_TV
        #[arg(long)]
        device: String,
        /// Bid modifier (e.g., 1.2 for +20%, 0.0 to exclude)
        #[arg(long)]
        bid_modifier: f64,
    },
    /// Remove a device bid adjustment
    Remove {
        /// Criterion ID or resource name
        id: String,
    },
}

// Schedule
#[derive(Subcommand)]
pub enum ScheduleCommands {
    /// List ad schedules for a campaign
    List {
        #[arg(long)]
        campaign_id: String,
    },
    /// Add an ad schedule
    Add {
        #[arg(long)]
        campaign_id: String,
        /// Day of week: MONDAY, TUESDAY, etc.
        #[arg(long)]
        day: String,
        #[arg(long)]
        start_hour: i32,
        #[arg(long)]
        end_hour: i32,
        #[arg(long)]
        bid_modifier: Option<f64>,
    },
    /// Remove an ad schedule
    Remove {
        id: String,
    },
}

// Location
#[derive(Subcommand)]
pub enum LocationCommands {
    /// List location targets for a campaign
    List {
        #[arg(long)]
        campaign_id: String,
    },
    /// Add a location target
    Add {
        #[arg(long)]
        campaign_id: String,
        /// Geo target constant ID (e.g., 2840 for United States)
        #[arg(long)]
        geo_id: String,
        /// Add as negative (exclude) location
        #[arg(long)]
        negative: bool,
        #[arg(long)]
        bid_modifier: Option<f64>,
    },
    /// Remove a location target
    Remove {
        id: String,
    },
    /// Search for geo target constants by name
    Search {
        /// Search text
        query: String,
    },
}

// Audience
#[derive(Subcommand)]
pub enum AudienceCommands {
    /// List audience targets
    List {
        #[arg(long)]
        campaign_id: Option<String>,
        #[arg(long)]
        ad_group_id: Option<String>,
    },
    /// Add an audience target
    Add {
        #[arg(long)]
        campaign_id: Option<String>,
        #[arg(long)]
        ad_group_id: Option<String>,
        /// User list resource name
        #[arg(long)]
        audience_id: String,
        #[arg(long)]
        bid_modifier: Option<f64>,
    },
    /// Remove an audience target
    Remove {
        id: String,
    },
}

// Shared negative keyword lists
#[derive(Subcommand)]
pub enum NegativeListCommands {
    /// List all shared negative keyword lists
    List,
    /// Create a new shared negative keyword list
    Create {
        #[arg(long)]
        name: String,
    },
    /// Remove a shared negative keyword list
    Remove {
        /// Shared set ID
        id: String,
    },
    /// Add a keyword to a shared negative keyword list
    AddKeyword {
        #[arg(long)]
        list_id: String,
        #[arg(long)]
        text: String,
        #[arg(long, default_value = "BROAD")]
        match_type: String,
    },
    /// Remove a keyword from a shared negative keyword list
    RemoveKeyword {
        /// Shared criterion ID
        id: String,
    },
    /// List keywords in a shared negative keyword list
    ListKeywords {
        #[arg(long)]
        list_id: String,
    },
    /// Apply a shared list to a campaign
    Apply {
        #[arg(long)]
        list_id: String,
        #[arg(long)]
        campaign_id: String,
    },
    /// Remove a shared list from a campaign
    Unapply {
        /// Campaign shared set ID
        id: String,
    },
    /// List campaigns using a shared list
    ListCampaigns {
        #[arg(long)]
        list_id: String,
    },
}
