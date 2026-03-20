use serde::Serialize;

use super::{state_to_str, status_to_str};

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
