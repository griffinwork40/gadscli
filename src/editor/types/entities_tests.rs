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
