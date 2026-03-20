use serde::Serialize;

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
