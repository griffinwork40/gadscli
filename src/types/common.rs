#![allow(dead_code)]

use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CampaignStatus {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "REMOVED")]
    Removed,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for CampaignStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CampaignStatus::Enabled => write!(f, "Enabled"),
            CampaignStatus::Paused => write!(f, "Paused"),
            CampaignStatus::Removed => write!(f, "Removed"),
            CampaignStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AdGroupStatus {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "REMOVED")]
    Removed,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for AdGroupStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdGroupStatus::Enabled => write!(f, "Enabled"),
            AdGroupStatus::Paused => write!(f, "Paused"),
            AdGroupStatus::Removed => write!(f, "Removed"),
            AdGroupStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AdStatus {
    #[serde(rename = "ENABLED")]
    Enabled,
    #[serde(rename = "PAUSED")]
    Paused,
    #[serde(rename = "REMOVED")]
    Removed,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for AdStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdStatus::Enabled => write!(f, "Enabled"),
            AdStatus::Paused => write!(f, "Paused"),
            AdStatus::Removed => write!(f, "Removed"),
            AdStatus::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum KeywordMatchType {
    #[serde(rename = "EXACT")]
    Exact,
    #[serde(rename = "PHRASE")]
    Phrase,
    #[serde(rename = "BROAD")]
    Broad,
}

impl fmt::Display for KeywordMatchType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeywordMatchType::Exact => write!(f, "Exact"),
            KeywordMatchType::Phrase => write!(f, "Phrase"),
            KeywordMatchType::Broad => write!(f, "Broad"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CampaignType {
    #[serde(rename = "SEARCH")]
    Search,
    #[serde(rename = "DISPLAY")]
    Display,
    #[serde(rename = "SHOPPING")]
    Shopping,
    #[serde(rename = "VIDEO")]
    Video,
    #[serde(rename = "PERFORMANCE_MAX")]
    PerformanceMax,
    #[serde(rename = "DEMAND_GEN")]
    DemandGen,
    #[serde(rename = "APP")]
    App,
    #[serde(rename = "LOCAL")]
    Local,
    #[serde(rename = "SMART")]
    Smart,
    #[serde(rename = "HOTEL")]
    Hotel,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for CampaignType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CampaignType::Search => write!(f, "Search"),
            CampaignType::Display => write!(f, "Display"),
            CampaignType::Shopping => write!(f, "Shopping"),
            CampaignType::Video => write!(f, "Video"),
            CampaignType::PerformanceMax => write!(f, "Performance Max"),
            CampaignType::DemandGen => write!(f, "Demand Gen"),
            CampaignType::App => write!(f, "App"),
            CampaignType::Local => write!(f, "Local"),
            CampaignType::Smart => write!(f, "Smart"),
            CampaignType::Hotel => write!(f, "Hotel"),
            CampaignType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum BiddingStrategyType {
    #[serde(rename = "MANUAL_CPC")]
    ManualCpc,
    #[serde(rename = "MANUAL_CPM")]
    ManualCpm,
    #[serde(rename = "MAXIMIZE_CLICKS")]
    MaximizeClicks,
    #[serde(rename = "MAXIMIZE_CONVERSIONS")]
    MaximizeConversions,
    #[serde(rename = "MAXIMIZE_CONVERSION_VALUE")]
    MaximizeConversionValue,
    #[serde(rename = "TARGET_CPA")]
    TargetCpa,
    #[serde(rename = "TARGET_ROAS")]
    TargetRoas,
    #[serde(rename = "TARGET_IMPRESSION_SHARE")]
    TargetImpressionShare,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for BiddingStrategyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BiddingStrategyType::ManualCpc => write!(f, "Manual CPC"),
            BiddingStrategyType::ManualCpm => write!(f, "Manual CPM"),
            BiddingStrategyType::MaximizeClicks => write!(f, "Maximize Clicks"),
            BiddingStrategyType::MaximizeConversions => write!(f, "Maximize Conversions"),
            BiddingStrategyType::MaximizeConversionValue => write!(f, "Maximize Conversion Value"),
            BiddingStrategyType::TargetCpa => write!(f, "Target CPA"),
            BiddingStrategyType::TargetRoas => write!(f, "Target ROAS"),
            BiddingStrategyType::TargetImpressionShare => write!(f, "Target Impression Share"),
            BiddingStrategyType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AdType {
    #[serde(rename = "RESPONSIVE_SEARCH_AD")]
    ResponsiveSearchAd,
    #[serde(rename = "RESPONSIVE_DISPLAY_AD")]
    ResponsiveDisplayAd,
    #[serde(rename = "EXPANDED_TEXT_AD")]
    ExpandedTextAd,
    #[serde(rename = "CALL_AD")]
    CallAd,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for AdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdType::ResponsiveSearchAd => write!(f, "Responsive Search Ad"),
            AdType::ResponsiveDisplayAd => write!(f, "Responsive Display Ad"),
            AdType::ExpandedTextAd => write!(f, "Expanded Text Ad"),
            AdType::CallAd => write!(f, "Call Ad"),
            AdType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AssetType {
    #[serde(rename = "TEXT")]
    Text,
    #[serde(rename = "IMAGE")]
    Image,
    #[serde(rename = "YOUTUBE_VIDEO")]
    YoutubeVideo,
    #[serde(rename = "MEDIA_BUNDLE")]
    MediaBundle,
    #[serde(rename = "LEAD_FORM")]
    LeadForm,
    #[serde(rename = "CALL_TO_ACTION")]
    CallToAction,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for AssetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssetType::Text => write!(f, "Text"),
            AssetType::Image => write!(f, "Image"),
            AssetType::YoutubeVideo => write!(f, "YouTube Video"),
            AssetType::MediaBundle => write!(f, "Media Bundle"),
            AssetType::LeadForm => write!(f, "Lead Form"),
            AssetType::CallToAction => write!(f, "Call to Action"),
            AssetType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ConversionActionType {
    #[serde(rename = "AD_CALL")]
    AdCall,
    #[serde(rename = "CLICK_TO_CALL")]
    ClickToCall,
    #[serde(rename = "GOOGLE_PLAY_DOWNLOAD")]
    GooglePlayDownload,
    #[serde(rename = "GOOGLE_PLAY_IN_APP_PURCHASE")]
    GooglePlayInAppPurchase,
    #[serde(rename = "UPLOAD_CALLS")]
    UploadCalls,
    #[serde(rename = "UPLOAD_CLICKS")]
    UploadClicks,
    #[serde(rename = "WEBPAGE")]
    Webpage,
    #[serde(rename = "WEBSITE_CALL")]
    WebsiteCall,
    #[serde(other)]
    Unknown,
}

impl fmt::Display for ConversionActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConversionActionType::AdCall => write!(f, "Ad Call"),
            ConversionActionType::ClickToCall => write!(f, "Click to Call"),
            ConversionActionType::GooglePlayDownload => write!(f, "Google Play Download"),
            ConversionActionType::GooglePlayInAppPurchase => {
                write!(f, "Google Play In-App Purchase")
            }
            ConversionActionType::UploadCalls => write!(f, "Upload Calls"),
            ConversionActionType::UploadClicks => write!(f, "Upload Clicks"),
            ConversionActionType::Webpage => write!(f, "Webpage"),
            ConversionActionType::WebsiteCall => write!(f, "Website Call"),
            ConversionActionType::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DeviceType {
    #[serde(rename = "MOBILE")]
    Mobile,
    #[serde(rename = "DESKTOP")]
    Desktop,
    #[serde(rename = "TABLET")]
    Tablet,
    #[serde(rename = "CONNECTED_TV")]
    ConnectedTv,
    #[serde(rename = "OTHER")]
    Other,
}

impl fmt::Display for DeviceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DeviceType::Mobile => write!(f, "Mobile"),
            DeviceType::Desktop => write!(f, "Desktop"),
            DeviceType::Tablet => write!(f, "Tablet"),
            DeviceType::ConnectedTv => write!(f, "Connected TV"),
            DeviceType::Other => write!(f, "Other"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DayOfWeek {
    #[serde(rename = "MONDAY")]
    Monday,
    #[serde(rename = "TUESDAY")]
    Tuesday,
    #[serde(rename = "WEDNESDAY")]
    Wednesday,
    #[serde(rename = "THURSDAY")]
    Thursday,
    #[serde(rename = "FRIDAY")]
    Friday,
    #[serde(rename = "SATURDAY")]
    Saturday,
    #[serde(rename = "SUNDAY")]
    Sunday,
}

impl fmt::Display for DayOfWeek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DayOfWeek::Monday => write!(f, "Monday"),
            DayOfWeek::Tuesday => write!(f, "Tuesday"),
            DayOfWeek::Wednesday => write!(f, "Wednesday"),
            DayOfWeek::Thursday => write!(f, "Thursday"),
            DayOfWeek::Friday => write!(f, "Friday"),
            DayOfWeek::Saturday => write!(f, "Saturday"),
            DayOfWeek::Sunday => write!(f, "Sunday"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
#[serde(rename_all = "lowercase")]
pub enum OutputFormat {
    Json,
    Table,
    Csv,
    Yaml,
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OutputFormat::Json => write!(f, "json"),
            OutputFormat::Table => write!(f, "table"),
            OutputFormat::Csv => write!(f, "csv"),
            OutputFormat::Yaml => write!(f, "yaml"),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "table" => Ok(OutputFormat::Table),
            "csv" => Ok(OutputFormat::Csv),
            "yaml" => Ok(OutputFormat::Yaml),
            other => Err(format!("Unknown output format: {other}")),
        }
    }
}
