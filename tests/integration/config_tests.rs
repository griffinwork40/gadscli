use gadscli::config::{Config, Profile};
use tempfile::TempDir;

#[test]
fn test_config_default_values() {
    let config = Config::default();
    assert_eq!(config.api_version, "20");
    assert_eq!(config.page_size, 1000);
    assert_eq!(config.output_format, "table");
    assert!(config.customer_id.is_none());
}

#[test]
fn test_config_save_and_load() {
    let tmp = TempDir::new().unwrap();
    let path = tmp.path().join("config.toml");

    let mut config = Config::default();
    config.customer_id = Some("1234567890".to_string());
    config.api_version = "17".to_string();
    config.save_to(&path).unwrap();

    let loaded = Config::load_from(&path).unwrap();
    assert_eq!(loaded.customer_id, Some("1234567890".to_string()));
    assert_eq!(loaded.api_version, "17");
}

#[test]
fn test_config_load_nonexistent() {
    let path = std::path::PathBuf::from("/tmp/nonexistent_gadscli_config.toml");
    let config = Config::load_from(&path).unwrap();
    assert_eq!(config.api_version, "20"); // defaults
}

#[test]
fn test_config_set_get() {
    let mut config = Config::default();

    config.set_value("customer_id", "123-456-7890").unwrap();
    assert_eq!(
        config.get_value("customer_id"),
        Some("1234567890".to_string())
    );

    config.set_value("output_format", "json").unwrap();
    assert_eq!(
        config.get_value("output-format"),
        Some("json".to_string())
    );

    config.set_value("page_size", "500").unwrap();
    assert_eq!(config.get_value("page-size"), Some("500".to_string()));
}

#[test]
fn test_config_set_invalid_format() {
    let mut config = Config::default();
    assert!(config.set_value("output_format", "xml").is_err());
}

#[test]
fn test_config_set_invalid_key() {
    let mut config = Config::default();
    assert!(config.set_value("unknown_key", "value").is_err());
}

#[test]
fn test_config_list_values() {
    let mut config = Config::default();
    config.customer_id = Some("123".to_string());
    let values = config.list_values();
    assert!(!values.is_empty());
    assert!(values
        .iter()
        .any(|(k, v)| k == "customer_id" && v == "123"));
}

#[test]
fn test_config_profile() {
    let mut config = Config::default();
    config.profiles.insert(
        "test".to_string(),
        Profile {
            customer_id: Some("9876543210".to_string()),
            api_version: Some("17".to_string()),
            ..Default::default()
        },
    );

    config.apply_profile("test").unwrap();
    assert_eq!(config.customer_id, Some("9876543210".to_string()));
    assert_eq!(config.api_version, "17");
}

#[test]
fn test_config_profile_not_found() {
    let mut config = Config::default();
    assert!(config.apply_profile("nonexistent").is_err());
}

#[test]
fn test_normalize_customer_id() {
    assert_eq!(Config::normalize_customer_id("123-456-7890"), "1234567890");
    assert_eq!(Config::normalize_customer_id("1234567890"), "1234567890");
    assert_eq!(Config::normalize_customer_id(""), "");
}
