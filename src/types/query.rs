#![allow(dead_code)]

use chrono::{Datelike, Duration, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchRequest {
    pub customer_id: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_total_results_count: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DateRange {
    pub start_date: String,
    pub end_date: String,
}

impl DateRange {
    pub fn today() -> Self {
        let today = Local::now().date_naive();
        let date_str = today.format("%Y-%m-%d").to_string();
        Self {
            start_date: date_str.clone(),
            end_date: date_str,
        }
    }

    pub fn yesterday() -> Self {
        let yesterday = Local::now().date_naive() - Duration::days(1);
        let date_str = yesterday.format("%Y-%m-%d").to_string();
        Self {
            start_date: date_str.clone(),
            end_date: date_str,
        }
    }

    pub fn last_7_days() -> Self {
        let today = Local::now().date_naive();
        let start = today - Duration::days(7);
        Self {
            start_date: start.format("%Y-%m-%d").to_string(),
            end_date: today.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn last_30_days() -> Self {
        let today = Local::now().date_naive();
        let start = today - Duration::days(30);
        Self {
            start_date: start.format("%Y-%m-%d").to_string(),
            end_date: today.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn this_month() -> Self {
        let today = Local::now().date_naive();
        let start = today
            .with_day(1)
            .expect("day 1 always valid");
        Self {
            start_date: start.format("%Y-%m-%d").to_string(),
            end_date: today.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn last_month() -> Self {
        let today = Local::now().date_naive();
        // Go to first day of this month, then subtract one day to get last day of last month
        let first_this_month = today.with_day(1).expect("day 1 always valid");
        let last_last_month = first_this_month - Duration::days(1);
        let first_last_month = last_last_month
            .with_day(1)
            .expect("day 1 always valid");
        Self {
            start_date: first_last_month.format("%Y-%m-%d").to_string(),
            end_date: last_last_month.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn last_14_days() -> Self {
        let today = Local::now().date_naive();
        let start = today - Duration::days(14);
        Self {
            start_date: start.format("%Y-%m-%d").to_string(),
            end_date: today.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn this_week_sun_today() -> Self {
        let today = Local::now().date_naive();
        let days_since_sunday = today.weekday().num_days_from_sunday();
        let sunday = today - Duration::days(days_since_sunday as i64);
        Self {
            start_date: sunday.format("%Y-%m-%d").to_string(),
            end_date: today.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn this_week_mon_today() -> Self {
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday();
        let monday = today - Duration::days(days_since_monday as i64);
        Self {
            start_date: monday.format("%Y-%m-%d").to_string(),
            end_date: today.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn last_week_sun_sat() -> Self {
        let today = Local::now().date_naive();
        let days_since_sunday = today.weekday().num_days_from_sunday();
        let this_sunday = today - Duration::days(days_since_sunday as i64);
        let last_sunday = this_sunday - Duration::days(7);
        let last_saturday = last_sunday + Duration::days(6);
        Self {
            start_date: last_sunday.format("%Y-%m-%d").to_string(),
            end_date: last_saturday.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn last_week_mon_sun() -> Self {
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday();
        let this_monday = today - Duration::days(days_since_monday as i64);
        let last_monday = this_monday - Duration::days(7);
        let last_sunday = last_monday + Duration::days(6);
        Self {
            start_date: last_monday.format("%Y-%m-%d").to_string(),
            end_date: last_sunday.format("%Y-%m-%d").to_string(),
        }
    }

    pub fn last_business_week() -> Self {
        let today = Local::now().date_naive();
        let days_since_monday = today.weekday().num_days_from_monday();
        let this_monday = today - Duration::days(days_since_monday as i64);
        let last_monday = this_monday - Duration::days(7);
        let last_friday = last_monday + Duration::days(4);
        Self {
            start_date: last_monday.format("%Y-%m-%d").to_string(),
            end_date: last_friday.format("%Y-%m-%d").to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReportTemplate {
    pub name: String,
    pub description: String,
    pub query: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_date_range: Option<String>,
}
