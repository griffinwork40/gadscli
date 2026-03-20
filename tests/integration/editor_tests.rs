use gadscli::editor::database::{EditorDatabase, EditorDatabaseWriter};
use std::path::PathBuf;

fn fixture_db_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/editor_test.db")
}

fn open_fixture_db() -> EditorDatabase {
    EditorDatabase::open(&fixture_db_path()).expect("Failed to open test fixture DB")
}

fn open_writable_copy() -> (tempfile::TempDir, EditorDatabaseWriter) {
    let tmp = tempfile::TempDir::new().unwrap();
    let dest = tmp.path().join("test.db");
    std::fs::copy(fixture_db_path(), &dest).unwrap();
    let writer = EditorDatabaseWriter::open(&dest).expect("Failed to open writable copy");
    (tmp, writer)
}

// --- Reader tests ---

#[test]
fn test_list_campaigns() {
    let db = open_fixture_db();
    let campaigns = db.list_campaigns().unwrap();
    // 3 campaigns total, but list_campaigns filters status != 4 (Removed)
    assert!(campaigns.len() >= 2);
    let alpha = campaigns.iter().find(|c| c.name == "Search Campaign Alpha");
    assert!(alpha.is_some());
    let alpha = alpha.unwrap();
    assert_eq!(alpha.remote_id, Some(100001));
    assert_eq!(alpha.status_str(), "Enabled");
    assert_eq!(alpha.state_str(), "Normal");
    assert!((alpha.budget_dollars() - 50.0).abs() < 0.01);
}

#[test]
fn test_list_campaigns_includes_new() {
    let db = open_fixture_db();
    let campaigns = db.list_campaigns().unwrap();
    let pmax = campaigns.iter().find(|c| c.name == "New PMax Campaign");
    assert!(pmax.is_some());
    assert_eq!(pmax.unwrap().state_str(), "New");
}

#[test]
fn test_list_ad_groups() {
    let db = open_fixture_db();
    let ad_groups = db.list_ad_groups(None).unwrap();
    assert!(ad_groups.len() >= 3);

    let (brand, campaign_name) = ad_groups
        .iter()
        .find(|(ag, _)| ag.name == "Brand Keywords")
        .unwrap();
    assert_eq!(campaign_name, "Search Campaign Alpha");
    assert_eq!(brand.status_str(), "Enabled");
    assert!((brand.bid_dollars().unwrap() - 1.5).abs() < 0.01);
}

#[test]
fn test_list_ad_groups_filtered() {
    let db = open_fixture_db();
    // campaign localId=1 should have 2 ad groups
    let ad_groups = db.list_ad_groups(Some(1)).unwrap();
    assert_eq!(ad_groups.len(), 2);
}

#[test]
fn test_list_keywords() {
    let db = open_fixture_db();
    let keywords = db.list_keywords(None).unwrap();
    assert!(keywords.len() >= 3);

    let (kw, ag_name, c_name) = keywords
        .iter()
        .find(|(kw, _, _)| kw.text == "buy shoes online")
        .unwrap();
    assert_eq!(ag_name, "Brand Keywords");
    assert_eq!(c_name, "Search Campaign Alpha");
    assert_eq!(kw.match_type_str(), "Broad");
    assert_eq!(kw.quality_score, Some(7));
}

#[test]
fn test_list_keywords_filtered() {
    let db = open_fixture_db();
    // ad group localId=1 should have 2 keywords
    let keywords = db.list_keywords(Some(1)).unwrap();
    assert_eq!(keywords.len(), 2);
}

#[test]
fn test_list_ads() {
    let db = open_fixture_db();
    let ads = db.list_ads(None).unwrap();
    assert!(ads.len() >= 2);

    let (ad, _, _) = ads
        .iter()
        .find(|(ad, _, _)| ad.headline1.as_deref() == Some("Buy Shoes Now"))
        .unwrap();
    assert_eq!(ad.headline2.as_deref(), Some("Free Shipping"));
    assert_eq!(ad.description1.as_deref(), Some("Shop the best shoes online."));
    assert_eq!(ad.status_str(), "Enabled");
}

#[test]
fn test_list_budgets() {
    let db = open_fixture_db();
    let budgets = db.list_budgets().unwrap();
    assert_eq!(budgets.len(), 2);

    let search_budget = budgets
        .iter()
        .find(|b| b.name.as_deref() == Some("Daily Budget - Search"))
        .unwrap();
    assert!((search_budget.budget_dollars() - 50.0).abs() < 0.01);
}

#[test]
fn test_list_labels() {
    let db = open_fixture_db();
    let labels = db.list_labels().unwrap();
    assert!(labels.len() >= 3);

    let new_label = labels.iter().find(|l| l.name == "New Label").unwrap();
    assert_eq!(new_label.state_str(), "New");
    assert_eq!(new_label.remote_id, None);
}

#[test]
fn test_get_account_settings() {
    let db = open_fixture_db();
    let settings = db.get_account_settings().unwrap();
    assert_eq!(settings.name.as_deref(), Some("Test Account"));
    assert_eq!(settings.currency_code.as_deref(), Some("USD"));
    assert_eq!(settings.time_zone.as_deref(), Some("America/New_York"));
    assert!((settings.optimization_score.unwrap() - 0.85).abs() < 0.01);
}

#[test]
fn test_pending_changes() {
    let db = open_fixture_db();
    let changes = db.pending_changes().unwrap();
    // At least: New PMax Campaign (state=2), cheap sneakers keyword (state=1),
    // New RSA Ad (state=2), New Label (state=2)
    assert!(changes.len() >= 3);

    let pmax = changes
        .iter()
        .find(|c| c.entity_type == "Campaign" && c.name == "New PMax Campaign");
    assert!(pmax.is_some());
    assert_eq!(pmax.unwrap().state_str(), "New");
}

#[test]
fn test_list_negative_keywords() {
    let db = open_fixture_db();
    let nkws = db.list_negative_keywords(None).unwrap();
    assert_eq!(nkws.len(), 2);

    let free = nkws.iter().find(|nk| nk.text == "free shoes").unwrap();
    assert_eq!(free.match_type_str(), "Exact");
    assert_eq!(free.status_str(), "Enabled");
}

#[test]
fn test_list_negative_keywords_filtered() {
    let db = open_fixture_db();
    // campaign localId=1 should have 2 negative keywords
    let nkws = db.list_negative_keywords(Some(1)).unwrap();
    assert_eq!(nkws.len(), 2);
}

#[test]
fn test_list_bidding_strategies() {
    let db = open_fixture_db();
    let strategies = db.list_bidding_strategies().unwrap();
    assert_eq!(strategies.len(), 2);

    let target_cpa = strategies
        .iter()
        .find(|s| s.name == "Target CPA Strategy")
        .unwrap();
    assert_eq!(target_cpa.strategy_type_str(), "Target CPA");
}

#[test]
fn test_list_sitelinks() {
    let db = open_fixture_db();
    let sitelinks = db.list_sitelinks().unwrap();
    assert_eq!(sitelinks.len(), 2);

    let shop = sitelinks
        .iter()
        .find(|sl| sl.link_text == "Shop Now")
        .unwrap();
    assert_eq!(shop.final_urls.as_deref(), Some("https://example.com/shop"));
    assert_eq!(shop.description1.as_deref(), Some("Browse our collection"));
}

#[test]
fn test_list_callouts() {
    let db = open_fixture_db();
    let callouts = db.list_callouts().unwrap();
    assert_eq!(callouts.len(), 2);
    assert!(callouts.iter().any(|c| c.text == "Free Shipping"));
    assert!(callouts.iter().any(|c| c.text == "24/7 Support"));
}

#[test]
fn test_list_structured_snippets() {
    let db = open_fixture_db();
    let snippets = db.list_structured_snippets().unwrap();
    assert_eq!(snippets.len(), 2);

    let brands = snippets.iter().find(|s| s.header == "Brands").unwrap();
    assert_eq!(brands.values.as_deref(), Some("Nike, Adidas, Puma"));
}

#[test]
fn test_list_geo_targets() {
    let db = open_fixture_db();
    let targets = db.list_geo_targets(None).unwrap();
    assert_eq!(targets.len(), 2);

    let ny = targets
        .iter()
        .find(|t| t.location_name.as_deref() == Some("New York, NY"))
        .unwrap();
    assert_eq!(ny.location_id, Some(1014221));
}

#[test]
fn test_list_audiences() {
    let db = open_fixture_db();
    let audiences = db.list_audiences(None).unwrap();
    assert_eq!(audiences.len(), 2);

    let shoes = audiences
        .iter()
        .find(|a| a.audience_name.as_deref() == Some("In-Market: Shoes"))
        .unwrap();
    assert_eq!(shoes.audience_id, Some(80432));
}

#[test]
fn test_list_placements() {
    let db = open_fixture_db();
    let placements = db.list_placements().unwrap();
    assert_eq!(placements.len(), 2);
    assert!(placements.iter().any(|p| p.url == "shoes.example.com"));
}

#[test]
fn test_list_search_terms() {
    let db = open_fixture_db();
    let terms = db.list_search_terms(None).unwrap();
    assert_eq!(terms.len(), 2);

    let term = terms
        .iter()
        .find(|t| t.search_term == "buy running shoes online")
        .unwrap();
    assert_eq!(term.keyword_text.as_deref(), Some("buy shoes online"));
}

#[test]
fn test_list_negative_keyword_lists() {
    let db = open_fixture_db();
    let lists = db.list_negative_keyword_lists().unwrap();
    assert_eq!(lists.len(), 2);
    assert!(lists.iter().any(|l| l.name == "Brand Exclusions"));
    assert!(lists.iter().any(|l| l.name == "Competitor Terms"));
}

#[test]
fn test_list_asset_groups() {
    let db = open_fixture_db();
    let groups = db.list_asset_groups().unwrap();
    assert_eq!(groups.len(), 2);
    assert!(groups.iter().any(|g| g.name == "Main Asset Group"));
}

// --- Writer tests ---

#[test]
fn test_writer_pause_keyword() {
    let (_tmp, writer) = open_writable_copy();
    writer.pause_keyword(1).unwrap();
    // Verify via read: need to open the same DB as reader
    let reader = EditorDatabase::open(&_tmp.path().join("test.db")).unwrap();
    let keywords = reader.list_keywords(None).unwrap();
    let kw = keywords.iter().find(|(kw, _, _)| kw.local_id == 1).unwrap();
    assert_eq!(kw.0.status_str(), "Paused");
    assert_eq!(kw.0.state_str(), "Edited");
}

#[test]
fn test_writer_enable_keyword() {
    let (_tmp, writer) = open_writable_copy();
    // First pause, then enable
    writer.pause_keyword(1).unwrap();
    writer.enable_keyword(1).unwrap();
    let reader = EditorDatabase::open(&_tmp.path().join("test.db")).unwrap();
    let keywords = reader.list_keywords(None).unwrap();
    let kw = keywords.iter().find(|(kw, _, _)| kw.local_id == 1).unwrap();
    assert_eq!(kw.0.status_str(), "Enabled");
}

#[test]
fn test_writer_remove_keyword() {
    let (_tmp, writer) = open_writable_copy();
    writer.remove_keyword(1).unwrap();
    // Removed keywords (status=4) should still be readable
    let reader = EditorDatabase::open(&_tmp.path().join("test.db")).unwrap();
    let keywords = reader.list_keywords(None).unwrap();
    let kw = keywords.iter().find(|(kw, _, _)| kw.local_id == 1).unwrap();
    assert_eq!(kw.0.status_str(), "Removed");
}

#[test]
fn test_writer_set_campaign_status() {
    let (_tmp, writer) = open_writable_copy();
    writer.set_campaign_status(1, 3).unwrap(); // Pause
    let reader = EditorDatabase::open(&_tmp.path().join("test.db")).unwrap();
    let campaigns = reader.list_campaigns().unwrap();
    let c = campaigns.iter().find(|c| c.local_id == 1).unwrap();
    assert_eq!(c.status_str(), "Paused");
    assert_eq!(c.state_str(), "Edited");
}

#[test]
fn test_writer_set_campaign_budget() {
    let (_tmp, writer) = open_writable_copy();
    writer.set_campaign_budget(1, 100_000_000).unwrap();
    let reader = EditorDatabase::open(&_tmp.path().join("test.db")).unwrap();
    let campaigns = reader.list_campaigns().unwrap();
    let c = campaigns.iter().find(|c| c.local_id == 1).unwrap();
    assert!((c.budget_dollars() - 100.0).abs() < 0.01);
}

#[test]
fn test_writer_add_keyword() {
    let (_tmp, writer) = open_writable_copy();
    let local_id = writer.add_keyword(1, "new keyword test", 1, 2_000_000).unwrap();
    assert!(local_id > 0);

    let reader = EditorDatabase::open(&_tmp.path().join("test.db")).unwrap();
    let keywords = reader.list_keywords(None).unwrap();
    let (kw, _, _) = keywords
        .iter()
        .find(|(kw, _, _)| kw.text == "new keyword test")
        .unwrap();
    assert_eq!(kw.match_type_str(), "Exact");
    assert_eq!(kw.state_str(), "New");
}
