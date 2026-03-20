use gadscli::types::common::*;
use gadscli::types::operations::*;
use gadscli::types::query::*;
use gadscli::types::resources::*;
use gadscli::types::responses::*;

#[test]
fn test_campaign_status_serde() {
    let json = "\"ENABLED\"";
    let status: CampaignStatus = serde_json::from_str(json).unwrap();
    assert_eq!(status, CampaignStatus::Enabled);

    let serialized = serde_json::to_string(&status).unwrap();
    assert_eq!(serialized, "\"ENABLED\"");
}

#[test]
fn test_campaign_status_unknown() {
    let json = "\"SOMETHING_NEW\"";
    let status: CampaignStatus = serde_json::from_str(json).unwrap();
    assert_eq!(status, CampaignStatus::Unknown);
}

#[test]
fn test_campaign_status_display() {
    assert_eq!(CampaignStatus::Enabled.to_string(), "Enabled");
    assert_eq!(CampaignStatus::Paused.to_string(), "Paused");
    assert_eq!(CampaignStatus::Removed.to_string(), "Removed");
}

#[test]
fn test_output_format_from_str() {
    assert_eq!("json".parse::<OutputFormat>().unwrap(), OutputFormat::Json);
    assert_eq!(
        "TABLE".parse::<OutputFormat>().unwrap(),
        OutputFormat::Table
    );
    assert_eq!("CSV".parse::<OutputFormat>().unwrap(), OutputFormat::Csv);
    assert!("xml".parse::<OutputFormat>().is_err());
}

#[test]
fn test_campaign_deserialize() {
    let json = r#"{
        "resourceName": "customers/123/campaigns/456",
        "id": "456",
        "name": "Test Campaign",
        "status": "ENABLED",
        "advertisingChannelType": "SEARCH"
    }"#;
    let campaign: Campaign = serde_json::from_str(json).unwrap();
    assert_eq!(campaign.resource_name, "customers/123/campaigns/456");
    assert_eq!(campaign.id, Some("456".to_string()));
    assert_eq!(campaign.name, Some("Test Campaign".to_string()));
    assert_eq!(campaign.status, Some(CampaignStatus::Enabled));
    assert_eq!(campaign.campaign_type, Some(CampaignType::Search));
}

#[test]
fn test_campaign_serialize_skips_none() {
    let campaign = Campaign {
        resource_name: "customers/123/campaigns/456".to_string(),
        id: Some("456".to_string()),
        name: None,
        status: None,
        campaign_type: None,
        bidding_strategy_type: None,
        budget: None,
        start_date: None,
        end_date: None,
    };
    let json = serde_json::to_string(&campaign).unwrap();
    assert!(!json.contains("name"));
    assert!(json.contains("resourceName"));
    assert!(json.contains("456"));
}

#[test]
fn test_search_response_deserialize() {
    let json = r#"{
        "results": [
            {
                "campaign": {
                    "resourceName": "customers/123/campaigns/456",
                    "id": "456",
                    "name": "Campaign A"
                },
                "metrics": {
                    "impressions": "1000",
                    "clicks": "50",
                    "costMicros": "25000000"
                }
            }
        ],
        "totalResultsCount": "1"
    }"#;
    let response: SearchResponse = serde_json::from_str(json).unwrap();
    assert_eq!(response.results.len(), 1);
    assert_eq!(response.total_results_count, Some(1));

    let row = &response.results[0];
    assert!(row.campaign.is_some());
    assert!(row.metrics.is_some());
    let campaign = row.campaign.as_ref().unwrap();
    assert_eq!(campaign.name, Some("Campaign A".to_string()));
    let metrics = row.metrics.as_ref().unwrap();
    assert_eq!(metrics.clicks, Some(50));
    assert_eq!(metrics.cost_micros, Some(25000000));
}

#[test]
fn test_mutate_operation_serialize() {
    let op = MutateOperation::<serde_json::Value> {
        create: Some(serde_json::json!({"name": "New Campaign"})),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("create"));
    assert!(!json.contains("update"));
    assert!(!json.contains("remove"));
}

#[test]
fn test_date_range_today() {
    let dr = DateRange::today();
    assert_eq!(dr.start_date, dr.end_date);
    assert!(dr.start_date.contains('-')); // YYYY-MM-DD format
}

#[test]
fn test_date_range_last_7_days() {
    let dr = DateRange::last_7_days();
    assert_ne!(dr.start_date, dr.end_date);
}

#[test]
fn test_metrics_deserialize() {
    let json = r#"{
        "impressions": 5000,
        "clicks": 250,
        "costMicros": 50000000,
        "ctr": 0.05,
        "averageCpc": 200000
    }"#;
    let metrics: Metrics = serde_json::from_str(json).unwrap();
    assert_eq!(metrics.impressions, Some(5000));
    assert_eq!(metrics.clicks, Some(250));
    assert_eq!(metrics.cost_micros, Some(50000000));
    assert_eq!(metrics.ctr, Some(0.05));
}

#[test]
fn test_keyword_match_type_serde() {
    assert_eq!(
        serde_json::to_string(&KeywordMatchType::Exact).unwrap(),
        "\"EXACT\""
    );
    assert_eq!(
        serde_json::to_string(&KeywordMatchType::Phrase).unwrap(),
        "\"PHRASE\""
    );
    assert_eq!(
        serde_json::to_string(&KeywordMatchType::Broad).unwrap(),
        "\"BROAD\""
    );
}
