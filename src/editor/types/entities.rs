use serde::Serialize;

use super::{micros_to_dollars, state_to_str, status_to_str};

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

#[cfg(test)]
#[path = "entities_tests.rs"]
mod tests;
