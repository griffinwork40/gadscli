#![allow(dead_code)]

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct EditorCampaign {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub name: String,
    pub status: i32,
    pub campaign_type: Option<i32>,
    pub budget_amount: Option<i64>,
    pub bidding_strategy_type: Option<i32>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub state: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorAdGroup {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub name: String,
    pub status: i32,
    pub max_cpc: Option<i64>,
    pub state: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorKeyword {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub text: String,
    pub criterion_type: i32,
    pub status: i32,
    pub max_cpc: Option<i64>,
    pub quality_score: Option<i32>,
    pub state: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorAd {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub status: i32,
    pub headline1: Option<String>,
    pub headline2: Option<String>,
    pub headline3: Option<String>,
    pub headline4: Option<String>,
    pub headline5: Option<String>,
    pub headline6: Option<String>,
    pub headline7: Option<String>,
    pub headline8: Option<String>,
    pub headline9: Option<String>,
    pub headline10: Option<String>,
    pub headline11: Option<String>,
    pub headline12: Option<String>,
    pub headline13: Option<String>,
    pub headline14: Option<String>,
    pub headline15: Option<String>,
    pub description1: Option<String>,
    pub description2: Option<String>,
    pub description3: Option<String>,
    pub description4: Option<String>,
    pub path1: Option<String>,
    pub path2: Option<String>,
    pub final_urls: Option<Vec<u8>>,
    pub state: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorBudget {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub name: Option<String>,
    pub budget_amount: Option<i64>,
    pub status: i32,
    pub state: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorLabel {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
    pub state: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorAccountSetting {
    pub name: Option<String>,
    pub currency_code: Option<String>,
    pub time_zone: Option<String>,
    pub optimization_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PendingChange {
    pub entity_type: String,
    pub local_id: i64,
    pub name: String,
    pub state: i32,
}

impl EditorCampaign {
    pub fn status_str(&self) -> &str {
        status_to_str(self.status)
    }

    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }

    pub fn budget_dollars(&self) -> f64 {
        micros_to_dollars(self.budget_amount)
    }
}

impl EditorAdGroup {
    pub fn status_str(&self) -> &str {
        status_to_str(self.status)
    }

    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }

    pub fn bid_dollars(&self) -> Option<f64> {
        self.max_cpc.map(|m| m as f64 / 1_000_000.0)
    }
}

impl EditorKeyword {
    pub fn status_str(&self) -> &str {
        status_to_str(self.status)
    }

    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }

    pub fn bid_dollars(&self) -> Option<f64> {
        self.max_cpc.map(|m| m as f64 / 1_000_000.0)
    }

    pub fn match_type_str(&self) -> &str {
        match self.criterion_type {
            0 => "Broad",
            1 => "Exact",
            2 => "Phrase",
            3 => "Broad",
            _ => "Unknown",
        }
    }
}

impl EditorAd {
    pub fn status_str(&self) -> &str {
        status_to_str(self.status)
    }

    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }

    pub fn headlines(&self) -> Vec<&str> {
        [
            self.headline1.as_deref(),
            self.headline2.as_deref(),
            self.headline3.as_deref(),
            self.headline4.as_deref(),
            self.headline5.as_deref(),
            self.headline6.as_deref(),
            self.headline7.as_deref(),
            self.headline8.as_deref(),
            self.headline9.as_deref(),
            self.headline10.as_deref(),
            self.headline11.as_deref(),
            self.headline12.as_deref(),
            self.headline13.as_deref(),
            self.headline14.as_deref(),
            self.headline15.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    pub fn descriptions(&self) -> Vec<&str> {
        [
            self.description1.as_deref(),
            self.description2.as_deref(),
            self.description3.as_deref(),
            self.description4.as_deref(),
        ]
        .into_iter()
        .flatten()
        .collect()
    }
}

impl EditorBudget {
    pub fn status_str(&self) -> &str {
        status_to_str(self.status)
    }

    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }

    pub fn budget_dollars(&self) -> f64 {
        micros_to_dollars(self.budget_amount)
    }
}

impl EditorLabel {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

impl PendingChange {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

fn status_to_str(status: i32) -> &'static str {
    match status {
        0 => "Enabled",
        2 => "Enabled",
        3 => "Paused",
        4 => "Removed",
        _ => "Unknown",
    }
}

fn state_to_str(state: i32) -> &'static str {
    match state {
        0 => "Normal",
        1 => "Edited",
        2 => "New",
        _ => "Unknown",
    }
}

fn micros_to_dollars(micros: Option<i64>) -> f64 {
    micros.map(|m| m as f64 / 1_000_000.0).unwrap_or(0.0)
}

// CSV import types for csv_writer
#[derive(Debug, Clone)]
pub struct KeywordEntry {
    pub campaign: String,
    pub ad_group: String,
    pub keyword: String,
    pub match_type: String,
    pub max_cpc: Option<f64>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct CampaignEntry {
    pub campaign: String,
    pub budget: f64,
    pub status: String,
    pub bidding_strategy: String,
}

#[derive(Debug, Clone)]
pub struct AdEntry {
    pub campaign: String,
    pub ad_group: String,
    pub headlines: Vec<String>,
    pub descriptions: Vec<String>,
    pub final_url: String,
    pub path1: Option<String>,
    pub path2: Option<String>,
    pub status: String,
}

// --- New entity types for expanded database readers ---

#[derive(Debug, Clone, Serialize)]
pub struct EditorNegativeKeyword {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub text: String,
    pub criterion_type: i32,
    pub status: i32,
    pub state: i32,
}

impl EditorNegativeKeyword {
    pub fn status_str(&self) -> &str {
        status_to_str(self.status)
    }
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
    pub fn match_type_str(&self) -> &str {
        match self.criterion_type {
            0 => "Broad",
            1 => "Exact",
            2 => "Phrase",
            3 => "Broad",
            _ => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorBiddingStrategy {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub name: String,
    pub strategy_type: Option<i32>,
    pub state: i32,
}

impl EditorBiddingStrategy {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
    pub fn strategy_type_str(&self) -> &str {
        match self.strategy_type {
            Some(0) => "Manual CPC",
            Some(1) => "Manual CPM",
            Some(2) => "Target CPA",
            Some(3) => "Max Conversions",
            Some(4) => "Max Clicks",
            Some(5) => "Target ROAS",
            Some(6) => "Max Conv Value",
            _ => "Unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorSitelink {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub link_text: String,
    pub final_urls: Option<String>,
    pub description1: Option<String>,
    pub description2: Option<String>,
    pub state: i32,
}

impl EditorSitelink {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorCallout {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub text: String,
    pub state: i32,
}

impl EditorCallout {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorStructuredSnippet {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub header: String,
    pub values: Option<String>,
    pub state: i32,
}

impl EditorStructuredSnippet {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorGeoTarget {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub location_id: Option<i64>,
    pub location_name: Option<String>,
    pub state: i32,
}

impl EditorGeoTarget {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorAudience {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub audience_id: Option<i64>,
    pub audience_name: Option<String>,
    pub state: i32,
}

impl EditorAudience {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorPlacement {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub url: String,
    pub state: i32,
}

impl EditorPlacement {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorSearchTerm {
    pub local_id: i64,
    pub parent_id: i64,
    pub search_term: String,
    pub keyword_text: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorNegativeKeywordList {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub name: String,
    pub state: i32,
}

impl EditorNegativeKeywordList {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct EditorAssetGroup {
    pub local_id: i64,
    pub remote_id: Option<i64>,
    pub parent_id: i64,
    pub name: String,
    pub state: i32,
}

impl EditorAssetGroup {
    pub fn state_str(&self) -> &str {
        state_to_str(self.state)
    }
}

// --- New CSV import entry types ---

#[derive(Debug, Clone)]
pub struct AdGroupEntry {
    pub campaign: String,
    pub ad_group: String,
    pub max_cpc: Option<f64>,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct NegativeKeywordEntry {
    pub campaign: String,
    pub ad_group: Option<String>,
    pub keyword: String,
    pub match_type: String,
}

#[derive(Debug, Clone)]
pub struct BudgetEntry {
    pub budget_name: String,
    pub amount: f64,
    pub status: String,
}

#[derive(Debug, Clone)]
pub struct SitelinkEntry {
    pub campaign: String,
    pub ad_group: Option<String>,
    pub sitelink_text: String,
    pub final_url: String,
    pub description1: Option<String>,
    pub description2: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CalloutEntry {
    pub campaign: String,
    pub ad_group: Option<String>,
    pub callout_text: String,
}

#[derive(Debug, Clone)]
pub struct LabelEntry {
    pub label_name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

/// Output from an Editor binary command
#[derive(Debug, Clone, Serialize)]
pub struct EditorOutput {
    pub exit_code: Option<i32>,
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
}

/// XML export format variants
#[derive(Debug, Clone)]
pub enum XmlExportFormat {
    Standard,
    Share,
    Upgrade,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_str() {
        let campaign = EditorCampaign {
            local_id: 1,
            remote_id: Some(100),
            name: "Test".to_string(),
            status: 0,
            campaign_type: None,
            budget_amount: None,
            bidding_strategy_type: None,
            start_date: None,
            end_date: None,
            state: 0,
        };
        assert_eq!(campaign.status_str(), "Enabled");

        let also_enabled = EditorCampaign { status: 2, ..campaign.clone() };
        assert_eq!(also_enabled.status_str(), "Enabled");

        let paused = EditorCampaign { status: 3, ..campaign.clone() };
        assert_eq!(paused.status_str(), "Paused");

        let removed = EditorCampaign { status: 4, ..campaign };
        assert_eq!(removed.status_str(), "Removed");
    }

    #[test]
    fn test_state_str() {
        let campaign = EditorCampaign {
            local_id: 1,
            remote_id: None,
            name: "Test".to_string(),
            status: 2,
            campaign_type: None,
            budget_amount: None,
            bidding_strategy_type: None,
            start_date: None,
            end_date: None,
            state: 0,
        };
        assert_eq!(campaign.state_str(), "Normal");

        let edited = EditorCampaign { state: 1, ..campaign.clone() };
        assert_eq!(edited.state_str(), "Edited");

        let new = EditorCampaign { state: 2, ..campaign };
        assert_eq!(new.state_str(), "New");
    }

    #[test]
    fn test_budget_dollars() {
        let campaign = EditorCampaign {
            local_id: 1,
            remote_id: None,
            name: "Test".to_string(),
            status: 2,
            campaign_type: None,
            budget_amount: Some(50_000_000),
            bidding_strategy_type: None,
            start_date: None,
            end_date: None,
            state: 0,
        };
        assert!((campaign.budget_dollars() - 50.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_budget_dollars_none() {
        let campaign = EditorCampaign {
            local_id: 1,
            remote_id: None,
            name: "Test".to_string(),
            status: 2,
            campaign_type: None,
            budget_amount: None,
            bidding_strategy_type: None,
            start_date: None,
            end_date: None,
            state: 0,
        };
        assert!((campaign.budget_dollars() - 0.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_bid_dollars() {
        let ag = EditorAdGroup {
            local_id: 1,
            remote_id: None,
            parent_id: 1,
            name: "Test AG".to_string(),
            status: 2,
            max_cpc: Some(1_500_000),
            state: 0,
        };
        assert!((ag.bid_dollars().unwrap() - 1.5).abs() < f64::EPSILON);
    }

    #[test]
    fn test_keyword_match_type() {
        let kw = EditorKeyword {
            local_id: 1,
            remote_id: None,
            parent_id: 1,
            text: "test keyword".to_string(),
            criterion_type: 1,
            status: 2,
            max_cpc: None,
            quality_score: None,
            state: 0,
        };
        assert_eq!(kw.match_type_str(), "Exact");
    }

    #[test]
    fn test_ad_headlines() {
        let ad = EditorAd {
            local_id: 1,
            remote_id: None,
            parent_id: 1,
            status: 2,
            headline1: Some("H1".to_string()),
            headline2: Some("H2".to_string()),
            headline3: None,
            headline4: None,
            headline5: None,
            headline6: None,
            headline7: None,
            headline8: None,
            headline9: None,
            headline10: None,
            headline11: None,
            headline12: None,
            headline13: None,
            headline14: None,
            headline15: None,
            description1: Some("D1".to_string()),
            description2: None,
            description3: None,
            description4: None,
            path1: None,
            path2: None,
            final_urls: None,
            state: 0,
        };
        assert_eq!(ad.headlines(), vec!["H1", "H2"]);
        assert_eq!(ad.descriptions(), vec!["D1"]);
    }
}
