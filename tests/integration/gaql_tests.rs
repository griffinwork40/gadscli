use gadscli::gaql::builder::QueryBuilder;
use gadscli::gaql::parser::{extract_resource, validate_query};
use gadscli::gaql::templates;

#[test]
fn test_query_builder_basic() {
    let query = QueryBuilder::new()
        .select(&["campaign.id", "campaign.name"])
        .from("campaign")
        .build()
        .unwrap();
    assert_eq!(query, "SELECT campaign.id, campaign.name FROM campaign");
}

#[test]
fn test_query_builder_with_where() {
    let query = QueryBuilder::new()
        .select(&["campaign.id", "campaign.name", "campaign.status"])
        .from("campaign")
        .where_clause("campaign.status = 'ENABLED'")
        .build()
        .unwrap();
    assert!(query.contains("WHERE campaign.status = 'ENABLED'"));
}

#[test]
fn test_query_builder_with_where_if_some() {
    let status = Some("ENABLED");
    let query = QueryBuilder::new()
        .select(&["campaign.id"])
        .from("campaign")
        .where_if("campaign.status", "=", status)
        .build()
        .unwrap();
    assert!(query.contains("WHERE campaign.status = 'ENABLED'"));
}

#[test]
fn test_query_builder_with_where_if_none() {
    let status: Option<&str> = None;
    let query = QueryBuilder::new()
        .select(&["campaign.id"])
        .from("campaign")
        .where_if("campaign.status", "=", status)
        .build()
        .unwrap();
    assert!(!query.contains("WHERE"));
}

#[test]
fn test_query_builder_with_order_and_limit() {
    let query = QueryBuilder::new()
        .select(&["campaign.id", "metrics.clicks"])
        .from("campaign")
        .order_by("metrics.clicks", true)
        .limit(10)
        .build()
        .unwrap();
    assert!(query.contains("ORDER BY metrics.clicks DESC"));
    assert!(query.contains("LIMIT 10"));
}

#[test]
fn test_query_builder_full() {
    let query = QueryBuilder::new()
        .select(&["campaign.id", "campaign.name", "metrics.clicks"])
        .from("campaign")
        .where_not("campaign.status", "REMOVED")
        .order_by("campaign.name", false)
        .limit(50)
        .build()
        .unwrap();
    assert!(query.starts_with("SELECT"));
    assert!(query.contains("FROM campaign"));
    assert!(query.contains("WHERE campaign.status != 'REMOVED'"));
    assert!(query.contains("ORDER BY campaign.name"));
    assert!(query.contains("LIMIT 50"));
}

#[test]
fn test_query_builder_no_select_fails() {
    let result = QueryBuilder::new().from("campaign").build();
    assert!(result.is_err());
}

#[test]
fn test_query_builder_no_from_fails() {
    let result = QueryBuilder::new().select(&["campaign.id"]).build();
    assert!(result.is_err());
}

#[test]
fn test_validate_valid_queries() {
    assert!(validate_query("SELECT campaign.id FROM campaign").is_ok());
    assert!(validate_query(
        "SELECT a.id, a.name FROM ad_group WHERE a.status = 'ENABLED'"
    )
    .is_ok());
    assert!(
        validate_query("SELECT c.id FROM campaign ORDER BY c.name LIMIT 10").is_ok()
    );
}

#[test]
fn test_validate_invalid_queries() {
    assert!(validate_query("FROM campaign SELECT id").is_err()); // wrong order
    assert!(validate_query("campaign.id").is_err()); // no SELECT
    assert!(validate_query("SELECT campaign.id;").is_err()); // semicolon
}

#[test]
fn test_extract_resource() {
    assert_eq!(
        extract_resource("SELECT c.id FROM campaign WHERE c.status = 'ENABLED'"),
        Some("campaign".into())
    );
    assert_eq!(
        extract_resource("SELECT a.id FROM ad_group"),
        Some("ad_group".into())
    );
    assert_eq!(
        extract_resource("SELECT k.text FROM keyword_view ORDER BY k.text"),
        Some("keyword_view".into())
    );
}

#[test]
fn test_templates_exist() {
    let all = templates::get_all_templates();
    assert!(!all.is_empty());
    assert!(all.len() >= 6);

    for t in &all {
        assert!(!t.name.is_empty());
        assert!(!t.description.is_empty());
        assert!(!t.query.is_empty());
        assert!(
            validate_query(&t.query).is_ok(),
            "Template '{}' has invalid GAQL",
            t.name
        );
    }
}

#[test]
fn test_get_template_by_name() {
    let t = templates::get_template("campaign-performance");
    assert!(t.is_some());
    let t = t.unwrap();
    assert_eq!(t.name, "campaign-performance");
    assert!(t.query.contains("campaign"));
}

#[test]
fn test_get_template_unknown() {
    assert!(templates::get_template("nonexistent-report").is_none());
}
