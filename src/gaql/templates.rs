#![allow(dead_code)]

use crate::types::query::ReportTemplate;

pub fn get_all_templates() -> Vec<ReportTemplate> {
    vec![
        campaign_performance(),
        ad_group_performance(),
        keyword_performance(),
        search_terms(),
        quality_score(),
        account_summary(),
        geographic_performance(),
        device_performance(),
        hourly_performance(),
    ]
}

pub fn get_template(name: &str) -> Option<ReportTemplate> {
    get_all_templates().into_iter().find(|t| t.name == name)
}

pub fn campaign_performance() -> ReportTemplate {
    ReportTemplate {
        name: "campaign-performance".to_string(),
        description: "Campaign performance metrics with spend, clicks, conversions".to_string(),
        query: "SELECT campaign.id, campaign.name, campaign.status, \
                metrics.impressions, metrics.clicks, metrics.ctr, \
                metrics.cost_micros, metrics.conversions, metrics.conversions_value, \
                metrics.average_cpc \
                FROM campaign \
                WHERE campaign.status != 'REMOVED' \
                ORDER BY metrics.cost_micros DESC"
            .to_string(),
        default_date_range: Some("LAST_30_DAYS".to_string()),
    }
}

pub fn ad_group_performance() -> ReportTemplate {
    ReportTemplate {
        name: "ad-group-performance".to_string(),
        description: "Ad group performance metrics".to_string(),
        query: "SELECT ad_group.id, ad_group.name, ad_group.status, ad_group.campaign, \
                metrics.impressions, metrics.clicks, metrics.ctr, \
                metrics.cost_micros, metrics.conversions, metrics.average_cpc \
                FROM ad_group \
                WHERE ad_group.status != 'REMOVED' \
                ORDER BY metrics.cost_micros DESC"
            .to_string(),
        default_date_range: Some("LAST_30_DAYS".to_string()),
    }
}

pub fn keyword_performance() -> ReportTemplate {
    ReportTemplate {
        name: "keyword-performance".to_string(),
        description: "Keyword performance metrics (impressions, clicks, cost, conversions)".to_string(),
        query: "SELECT ad_group_criterion.criterion_id, ad_group_criterion.keyword.text, \
                ad_group_criterion.keyword.match_type, ad_group_criterion.status, \
                metrics.impressions, metrics.clicks, metrics.ctr, \
                metrics.cost_micros, metrics.conversions, metrics.average_cpc \
                FROM keyword_view \
                WHERE ad_group_criterion.status != 'REMOVED' \
                ORDER BY metrics.impressions DESC"
            .to_string(),
        default_date_range: Some("LAST_30_DAYS".to_string()),
    }
}

pub fn search_terms() -> ReportTemplate {
    ReportTemplate {
        name: "search-terms".to_string(),
        description: "Search terms report showing actual queries".to_string(),
        query: "SELECT search_term_view.search_term, search_term_view.status, \
                campaign.name, ad_group.name, \
                metrics.impressions, metrics.clicks, metrics.cost_micros, metrics.conversions \
                FROM search_term_view \
                ORDER BY metrics.impressions DESC"
            .to_string(),
        default_date_range: Some("LAST_30_DAYS".to_string()),
    }
}

pub fn quality_score() -> ReportTemplate {
    ReportTemplate {
        name: "quality-score".to_string(),
        description: "Quality score breakdown by keyword".to_string(),
        query: "SELECT ad_group_criterion.criterion_id, ad_group_criterion.keyword.text, \
                ad_group_criterion.quality_info.quality_score, \
                ad_group_criterion.quality_info.creative_quality_score, \
                ad_group_criterion.quality_info.post_click_quality_score, \
                ad_group_criterion.quality_info.search_predicted_ctr \
                FROM keyword_view \
                WHERE ad_group_criterion.status = 'ENABLED' \
                ORDER BY ad_group_criterion.quality_info.quality_score ASC"
            .to_string(),
        default_date_range: None,
    }
}

pub fn account_summary() -> ReportTemplate {
    ReportTemplate {
        name: "account-summary".to_string(),
        description: "Overall account performance summary".to_string(),
        query: "SELECT metrics.impressions, metrics.clicks, metrics.ctr, \
                metrics.cost_micros, metrics.conversions, metrics.conversions_value, \
                metrics.all_conversions, metrics.average_cpc \
                FROM customer"
            .to_string(),
        default_date_range: Some("LAST_30_DAYS".to_string()),
    }
}

pub fn geographic_performance() -> ReportTemplate {
    ReportTemplate {
        name: "geographic-performance".to_string(),
        description: "Performance by geographic location".to_string(),
        query: "SELECT geographic_view.country_criterion_id, \
                geographic_view.location_type, \
                metrics.impressions, metrics.clicks, metrics.cost_micros, metrics.conversions \
                FROM geographic_view \
                ORDER BY metrics.cost_micros DESC"
            .to_string(),
        default_date_range: Some("LAST_30_DAYS".to_string()),
    }
}

pub fn device_performance() -> ReportTemplate {
    ReportTemplate {
        name: "device-performance".to_string(),
        description: "Performance by device type".to_string(),
        query: "SELECT segments.device, \
                metrics.impressions, metrics.clicks, metrics.ctr, \
                metrics.cost_micros, metrics.conversions \
                FROM campaign \
                WHERE campaign.status != 'REMOVED'"
            .to_string(),
        default_date_range: Some("LAST_30_DAYS".to_string()),
    }
}

pub fn hourly_performance() -> ReportTemplate {
    ReportTemplate {
        name: "hourly-performance".to_string(),
        description: "Performance by hour of day".to_string(),
        query: "SELECT segments.hour, \
                metrics.impressions, metrics.clicks, metrics.cost_micros, metrics.conversions \
                FROM campaign \
                WHERE campaign.status != 'REMOVED'"
            .to_string(),
        default_date_range: Some("LAST_7_DAYS".to_string()),
    }
}
