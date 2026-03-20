use gadscli::editor::csv_writer;
use gadscli::editor::types::*;
use tempfile::NamedTempFile;

// --- Keyword CSV ---

#[test]
fn test_write_keyword_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![
        KeywordEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: "Ad Group 1".to_string(),
            keyword: "buy shoes".to_string(),
            match_type: "Broad".to_string(),
            max_cpc: Some(1.50),
            status: "Enabled".to_string(),
        },
        KeywordEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: "Ad Group 1".to_string(),
            keyword: "[exact match]".to_string(),
            match_type: "Exact".to_string(),
            max_cpc: None,
            status: "Paused".to_string(),
        },
    ];

    csv_writer::write_keyword_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines[0], "Campaign,Ad group,Keyword,Match type,Max CPC,Status");
    assert_eq!(lines[1], "Campaign 1,Ad Group 1,buy shoes,Broad,1.50,Enabled");
    assert_eq!(lines[2], "Campaign 1,Ad Group 1,[exact match],Exact,,Paused");
}

#[test]
fn test_write_keyword_csv_special_chars() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![KeywordEntry {
        campaign: "Campaign, With Comma".to_string(),
        ad_group: "Ad Group \"Quoted\"".to_string(),
        keyword: "keyword with, comma".to_string(),
        match_type: "Broad".to_string(),
        max_cpc: Some(2.00),
        status: "Enabled".to_string(),
    }];

    csv_writer::write_keyword_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    // CSV should properly quote fields with commas
    assert!(content.contains("\"Campaign, With Comma\""));
    assert!(content.contains("\"keyword with, comma\""));
}

#[test]
fn test_write_keyword_csv_empty() {
    let tmp = NamedTempFile::new().unwrap();
    csv_writer::write_keyword_csv(tmp.path(), &[]).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 1); // Header only
}

// --- Campaign CSV ---

#[test]
fn test_write_campaign_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![CampaignEntry {
        campaign: "My Campaign".to_string(),
        budget: 50.0,
        status: "Enabled".to_string(),
        bidding_strategy: "Manual CPC".to_string(),
    }];

    csv_writer::write_campaign_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines[0], "Campaign,Budget,Campaign status,Bid strategy type");
    assert_eq!(lines[1], "My Campaign,50.00,Enabled,Manual CPC");
}

// --- Ad CSV ---

#[test]
fn test_write_ad_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![AdEntry {
        campaign: "Campaign 1".to_string(),
        ad_group: "Ad Group 1".to_string(),
        headlines: vec!["H1".to_string(), "H2".to_string(), "H3".to_string()],
        descriptions: vec!["D1".to_string(), "D2".to_string()],
        final_url: "https://example.com".to_string(),
        path1: Some("shoes".to_string()),
        path2: Some("buy".to_string()),
        status: "Enabled".to_string(),
    }];

    csv_writer::write_ad_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert!(lines[0].starts_with("Campaign,Ad group,Headline 1"));
    assert!(lines[1].contains("H1,H2,H3,D1,D2,https://example.com,shoes,buy,Enabled"));
}

// --- Ad Group CSV ---

#[test]
fn test_write_ad_group_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![
        AdGroupEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: "New Ad Group".to_string(),
            max_cpc: Some(1.50),
            status: "Enabled".to_string(),
        },
        AdGroupEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: "Another Group".to_string(),
            max_cpc: None,
            status: "Paused".to_string(),
        },
    ];

    csv_writer::write_ad_group_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines[0], "Campaign,Ad group,Max CPC,Ad group status");
    assert_eq!(lines[1], "Campaign 1,New Ad Group,1.50,Enabled");
    assert_eq!(lines[2], "Campaign 1,Another Group,,Paused");
}

#[test]
fn test_write_ad_group_csv_empty() {
    let tmp = NamedTempFile::new().unwrap();
    csv_writer::write_ad_group_csv(tmp.path(), &[]).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines.len(), 1);
}

// --- Negative Keyword CSV ---

#[test]
fn test_write_negative_keyword_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![
        NegativeKeywordEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: None,
            keyword: "free stuff".to_string(),
            match_type: "Exact".to_string(),
        },
        NegativeKeywordEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: Some("Ad Group 1".to_string()),
            keyword: "cheap".to_string(),
            match_type: "Phrase".to_string(),
        },
    ];

    csv_writer::write_negative_keyword_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines[0], "Campaign,Ad group,Negative keyword,Match type");
    assert_eq!(lines[1], "Campaign 1,,free stuff,Exact");
    assert_eq!(lines[2], "Campaign 1,Ad Group 1,cheap,Phrase");
}

#[test]
fn test_write_negative_keyword_csv_empty() {
    let tmp = NamedTempFile::new().unwrap();
    csv_writer::write_negative_keyword_csv(tmp.path(), &[]).unwrap();
    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert_eq!(content.lines().count(), 1);
}

// --- Budget CSV ---

#[test]
fn test_write_budget_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![BudgetEntry {
        budget_name: "Daily Budget".to_string(),
        amount: 75.50,
        status: "Enabled".to_string(),
    }];

    csv_writer::write_budget_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines[0], "Budget name,Budget,Budget status");
    assert_eq!(lines[1], "Daily Budget,75.50,Enabled");
}

#[test]
fn test_write_budget_csv_empty() {
    let tmp = NamedTempFile::new().unwrap();
    csv_writer::write_budget_csv(tmp.path(), &[]).unwrap();
    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert_eq!(content.lines().count(), 1);
}

// --- Sitelink CSV ---

#[test]
fn test_write_sitelink_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![SitelinkEntry {
        campaign: "Campaign 1".to_string(),
        ad_group: None,
        sitelink_text: "Shop Now".to_string(),
        final_url: "https://example.com/shop".to_string(),
        description1: Some("Browse collection".to_string()),
        description2: Some("Free shipping".to_string()),
    }];

    csv_writer::write_sitelink_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert!(lines[0].starts_with("Campaign,Ad group,Sitelink text"));
    assert!(lines[1].contains("Shop Now"));
    assert!(lines[1].contains("https://example.com/shop"));
    assert!(lines[1].contains("Browse collection"));
}

#[test]
fn test_write_sitelink_csv_empty() {
    let tmp = NamedTempFile::new().unwrap();
    csv_writer::write_sitelink_csv(tmp.path(), &[]).unwrap();
    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert_eq!(content.lines().count(), 1);
}

// --- Callout CSV ---

#[test]
fn test_write_callout_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![
        CalloutEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: None,
            callout_text: "Free Shipping".to_string(),
        },
        CalloutEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: Some("Ad Group 1".to_string()),
            callout_text: "24/7 Support".to_string(),
        },
    ];

    csv_writer::write_callout_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines[0], "Campaign,Ad group,Callout text");
    assert_eq!(lines[1], "Campaign 1,,Free Shipping");
    assert_eq!(lines[2], "Campaign 1,Ad Group 1,24/7 Support");
}

#[test]
fn test_write_callout_csv_empty() {
    let tmp = NamedTempFile::new().unwrap();
    csv_writer::write_callout_csv(tmp.path(), &[]).unwrap();
    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert_eq!(content.lines().count(), 1);
}

// --- Label CSV ---

#[test]
fn test_write_label_csv_roundtrip() {
    let tmp = NamedTempFile::new().unwrap();
    let entries = vec![
        LabelEntry {
            label_name: "High Priority".to_string(),
            description: Some("Important campaigns".to_string()),
            color: Some("#FF0000".to_string()),
        },
        LabelEntry {
            label_name: "Seasonal".to_string(),
            description: None,
            color: None,
        },
    ];

    csv_writer::write_label_csv(tmp.path(), &entries).unwrap();

    let content = std::fs::read_to_string(tmp.path()).unwrap();
    let lines: Vec<&str> = content.lines().collect();
    assert_eq!(lines[0], "Label name,Label description,Label color");
    assert_eq!(lines[1], "High Priority,Important campaigns,#FF0000");
    assert_eq!(lines[2], "Seasonal,,");
}

#[test]
fn test_write_label_csv_empty() {
    let tmp = NamedTempFile::new().unwrap();
    csv_writer::write_label_csv(tmp.path(), &[]).unwrap();
    let content = std::fs::read_to_string(tmp.path()).unwrap();
    assert_eq!(content.lines().count(), 1);
}
