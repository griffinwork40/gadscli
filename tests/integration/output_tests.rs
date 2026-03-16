use gadscli::output;

#[test]
fn test_json_to_string() {
    let data = serde_json::json!({"name": "test", "value": 42});
    let result = output::json::to_string(&data).unwrap();
    assert!(result.contains("\"name\""));
    assert!(result.contains("\"test\""));
    assert!(result.contains("42"));
}

#[test]
fn test_yaml_to_string() {
    let data = serde_json::json!({"name": "test", "value": 42});
    let result = output::yaml::to_string(&data).unwrap();
    assert!(result.contains("name"));
    assert!(result.contains("test"));
}
