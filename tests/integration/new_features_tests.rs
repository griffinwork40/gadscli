use gadscli::types::responses::*;
use gadscli::types::responses_ext::*;
use gadscli::types::operations::*;

#[test]
fn test_location_info_deserialize() {
    let json = r#"{"geoTargetConstant": "geoTargetConstants/2840"}"#;
    let info: LocationInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.geo_target_constant, Some("geoTargetConstants/2840".to_string()));
}

#[test]
fn test_device_info_deserialize() {
    let json = r#"{"type": "MOBILE"}"#;
    let info: DeviceInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.device_type, Some("MOBILE".to_string()));
}

#[test]
fn test_ad_schedule_info_deserialize() {
    let json = r#"{"dayOfWeek": "MONDAY", "startHour": 9, "endHour": 17, "startMinute": "ZERO", "endMinute": "ZERO"}"#;
    let info: AdScheduleInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.day_of_week, Some("MONDAY".to_string()));
    assert_eq!(info.start_hour, Some(9));
    assert_eq!(info.end_hour, Some(17));
}

#[test]
fn test_user_list_info_deserialize() {
    let json = r#"{"userList": "customers/123/userLists/456"}"#;
    let info: UserListInfo = serde_json::from_str(json).unwrap();
    assert_eq!(info.user_list, Some("customers/123/userLists/456".to_string()));
}

#[test]
fn test_campaign_criterion_row_extended() {
    let json = r#"{
        "resourceName": "customers/123/campaignCriteria/456~789",
        "criterionId": "789",
        "type": "LOCATION",
        "bidModifier": "1.2",
        "location": {"geoTargetConstant": "geoTargetConstants/2840"},
        "negative": false,
        "campaign": "customers/123/campaigns/456"
    }"#;
    let row: CampaignCriterionRow = serde_json::from_str(json).unwrap();
    assert_eq!(row.criterion_type, Some("LOCATION".to_string()));
    assert!((row.bid_modifier.unwrap() - 1.2).abs() < f64::EPSILON);
    assert!(row.location.is_some());
}

#[test]
fn test_campaign_asset_row_deserialize() {
    let json = r#"{
        "resourceName": "customers/123/campaignAssets/456~789",
        "campaign": "customers/123/campaigns/456",
        "asset": "customers/123/assets/789",
        "fieldType": "SITELINK",
        "status": "ENABLED"
    }"#;
    let row: CampaignAssetRow = serde_json::from_str(json).unwrap();
    assert_eq!(row.field_type, Some("SITELINK".to_string()));
}

#[test]
fn test_shared_set_deserialize() {
    let json = r#"{
        "resourceName": "customers/123/sharedSets/456",
        "id": "456",
        "name": "My Negative List",
        "type": "NEGATIVE_KEYWORDS",
        "status": "ENABLED",
        "memberCount": "10"
    }"#;
    let ss: SharedSet = serde_json::from_str(json).unwrap();
    assert_eq!(ss.name, Some("My Negative List".to_string()));
    assert_eq!(ss.member_count, Some(10));
}

#[test]
fn test_shared_criterion_row_deserialize() {
    let json = r#"{
        "resourceName": "customers/123/sharedCriteria/456~789",
        "sharedSet": "customers/123/sharedSets/456",
        "keyword": {"text": "bad keyword", "matchType": "BROAD"},
        "criterionId": "789"
    }"#;
    let sc: SharedCriterionRow = serde_json::from_str(json).unwrap();
    assert_eq!(sc.keyword.as_ref().unwrap().text, Some("bad keyword".to_string()));
}

#[test]
fn test_campaign_shared_set_row_deserialize() {
    let json = r#"{
        "resourceName": "customers/123/campaignSharedSets/456~789",
        "campaign": "customers/123/campaigns/456",
        "sharedSet": "customers/123/sharedSets/789",
        "status": "ENABLED"
    }"#;
    let css: CampaignSharedSetRow = serde_json::from_str(json).unwrap();
    assert_eq!(css.status, Some("ENABLED".to_string()));
}

#[test]
fn test_search_term_view_row_deserialize() {
    let json = r#"{
        "resourceName": "customers/123/searchTermViews/456~789",
        "searchTerm": "buy shoes online",
        "adGroup": "customers/123/adGroups/456",
        "status": "ADDED"
    }"#;
    let stv: SearchTermViewRow = serde_json::from_str(json).unwrap();
    assert_eq!(stv.search_term, Some("buy shoes online".to_string()));
}

#[test]
fn test_geo_target_constant_row_deserialize() {
    let json = r#"{
        "resourceName": "geoTargetConstants/2840",
        "id": "2840",
        "name": "United States",
        "canonicalName": "United States",
        "countryCode": "US",
        "targetType": "Country",
        "status": "ENABLED"
    }"#;
    let gtc: GeoTargetConstantRow = serde_json::from_str(json).unwrap();
    assert_eq!(gtc.name, Some("United States".to_string()));
    assert_eq!(gtc.country_code, Some("US".to_string()));
}

#[test]
fn test_keyword_idea_result_deserialize() {
    let json = r#"{
        "text": "ppf film",
        "keywordIdeaMetrics": {
            "avgMonthlySearches": "1000",
            "competition": "MEDIUM",
            "lowTopOfPageBidMicros": "500000",
            "highTopOfPageBidMicros": "2000000"
        }
    }"#;
    let result: KeywordIdeaResult = serde_json::from_str(json).unwrap();
    assert_eq!(result.text, Some("ppf film".to_string()));
    let metrics = result.keyword_idea_metrics.unwrap();
    assert_eq!(metrics.avg_monthly_searches, Some(1000));
    assert_eq!(metrics.competition, Some("MEDIUM".to_string()));
}

#[test]
fn test_search_row_with_new_fields() {
    let json = r#"{
        "campaign": {"resourceName": "customers/123/campaigns/456"},
        "sharedSet": {"resourceName": "customers/123/sharedSets/789", "name": "Test List"},
        "geoTargetConstant": {"resourceName": "geoTargetConstants/2840", "name": "United States"}
    }"#;
    let row: SearchRow = serde_json::from_str(json).unwrap();
    assert!(row.campaign.is_some());
    assert!(row.shared_set.is_some());
    assert!(row.geo_target_constant.is_some());
}

#[test]
fn test_mutate_operation_campaign_criteria() {
    let op = MutateOperation::<serde_json::Value> {
        create: Some(serde_json::json!({
            "campaign": "customers/123/campaigns/456",
            "criterion": {
                "device": {"type": "MOBILE"}
            },
            "bidModifier": 1.2
        })),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("MOBILE"));
    assert!(json.contains("bidModifier"));
}

#[test]
fn test_campaign_asset_mutate_serialize() {
    let body = serde_json::json!({
        "campaign": "customers/123/campaigns/456",
        "asset": "customers/123/assets/789",
        "fieldType": "SITELINK"
    });
    let op = MutateOperation::<serde_json::Value> {
        create: Some(body),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("SITELINK"));
    assert!(json.contains("assets/789"));
}

#[test]
fn test_shared_set_create_mutate_serialize() {
    let body = serde_json::json!({
        "name": "My Negative List",
        "type": "NEGATIVE_KEYWORDS"
    });
    let op = MutateOperation::<serde_json::Value> {
        create: Some(body),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("NEGATIVE_KEYWORDS"));
}

#[test]
fn test_shared_criterion_create_mutate_serialize() {
    let body = serde_json::json!({
        "sharedSet": "customers/123/sharedSets/456",
        "keyword": { "text": "bad keyword", "matchType": "BROAD" }
    });
    let op = MutateOperation::<serde_json::Value> {
        create: Some(body),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("bad keyword"));
}

#[test]
fn test_campaign_shared_set_create_serialize() {
    let body = serde_json::json!({
        "campaign": "customers/123/campaigns/456",
        "sharedSet": "customers/123/sharedSets/789"
    });
    let op = MutateOperation::<serde_json::Value> {
        create: Some(body),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("sharedSets/789"));
}

#[test]
fn test_parse_pin_headline() {
    let result = gadscli::commands::ad::parse_pin("My Headline:1", "HEADLINE");
    assert_eq!(result, Some(("My Headline".to_string(), "HEADLINE_1".to_string())));
}

#[test]
fn test_parse_pin_description() {
    let result = gadscli::commands::ad::parse_pin("Description text:2", "DESCRIPTION");
    assert_eq!(result, Some(("Description text".to_string(), "DESCRIPTION_2".to_string())));
}

#[test]
fn test_parse_pin_no_colon() {
    let result = gadscli::commands::ad::parse_pin("no pin here", "HEADLINE");
    assert!(result.is_none());
}

#[test]
fn test_bulk_keyword_ops_construction() {
    use gadscli::commands::keyword::{AdGroupCriterionMutate, KeywordInfo};
    let keywords = vec!["term1".to_string(), "term2".to_string(), "term3".to_string()];
    let ops: Vec<MutateOperation<AdGroupCriterionMutate>> = keywords.iter()
        .map(|kw| MutateOperation {
            create: Some(AdGroupCriterionMutate {
                ad_group: Some("customers/123/adGroups/456".to_string()),
                status: Some("ENABLED".to_string()),
                keyword: Some(KeywordInfo {
                    text: kw.clone(),
                    match_type: "BROAD".to_string(),
                }),
                ..Default::default()
            }),
            update: None,
            remove: None,
            update_mask: None,
        })
        .collect();
    assert_eq!(ops.len(), 3);
    let json = serde_json::to_string(&ops).unwrap();
    assert!(json.contains("term1"));
    assert!(json.contains("term2"));
    assert!(json.contains("term3"));
}

#[test]
fn test_keyword_ideas_request_body_text_only() {
    let body = serde_json::json!({
        "keywordSeed": { "keywords": ["ppf film", "paint protection"] }
    });
    let s = serde_json::to_string(&body).unwrap();
    assert!(s.contains("ppf film"));
}

#[test]
fn test_keyword_ideas_request_body_url_only() {
    let body = serde_json::json!({
        "urlSeed": { "url": "https://example.com" }
    });
    let s = serde_json::to_string(&body).unwrap();
    assert!(s.contains("example.com"));
}

#[test]
fn test_keyword_ideas_request_body_both() {
    let body = serde_json::json!({
        "keywordSeed": { "keywords": ["ppf"] },
        "urlSeed": { "url": "https://example.com" }
    });
    let s = serde_json::to_string(&body).unwrap();
    assert!(s.contains("ppf"));
    assert!(s.contains("example.com"));
}

#[test]
fn test_search_term_exclusion_query_construction() {
    let cid = "123456";
    let campaign_id = "789";
    let min_cost = Some(10_000_000i64);
    let max_conv = Some(0.0f64);

    let mut query = format!(
        "SELECT search_term_view.search_term, search_term_view.status, \
         metrics.cost_micros, metrics.conversions \
         FROM search_term_view \
         WHERE campaign.resource_name = 'customers/{}/campaigns/{}' \
         AND search_term_view.status = 'NONE'",
        cid, campaign_id
    );

    if let Some(mc) = min_cost {
        query.push_str(&format!(" AND metrics.cost_micros >= {}", mc));
    }
    if let Some(mc) = max_conv {
        query.push_str(&format!(" AND metrics.conversions <= {}", mc));
    }

    assert!(query.contains("cost_micros >= 10000000"));
    assert!(query.contains("conversions <= 0"));
    assert!(query.contains("campaigns/789"));
}

#[test]
fn test_device_criterion_mutate_serialize() {
    let body = serde_json::json!({
        "campaign": "customers/123/campaigns/456",
        "device": { "type": "MOBILE" },
        "bidModifier": 1.2
    });
    let op = MutateOperation::<serde_json::Value> {
        create: Some(body),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("MOBILE"));
    assert!(json.contains("1.2"));
}

#[test]
fn test_schedule_criterion_mutate_serialize() {
    let body = serde_json::json!({
        "campaign": "customers/123/campaigns/456",
        "adSchedule": {
            "dayOfWeek": "MONDAY",
            "startHour": 9,
            "endHour": 17,
            "startMinute": "ZERO",
            "endMinute": "ZERO"
        },
        "bidModifier": 1.5
    });
    let op = MutateOperation::<serde_json::Value> {
        create: Some(body),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("MONDAY"));
    assert!(json.contains("1.5"));
}

#[test]
fn test_location_criterion_mutate_serialize() {
    let body = serde_json::json!({
        "campaign": "customers/123/campaigns/456",
        "location": { "geoTargetConstant": "geoTargetConstants/2840" },
        "negative": false,
        "bidModifier": 1.1
    });
    let op = MutateOperation::<serde_json::Value> {
        create: Some(body),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("2840"));
}

#[test]
fn test_audience_criterion_mutate_serialize() {
    let body = serde_json::json!({
        "campaign": "customers/123/campaigns/456",
        "userList": { "userList": "customers/123/userLists/789" },
        "bidModifier": 1.3
    });
    let op = MutateOperation::<serde_json::Value> {
        create: Some(body),
        update: None,
        remove: None,
        update_mask: None,
    };
    let json = serde_json::to_string(&op).unwrap();
    assert!(json.contains("userLists"));
}
