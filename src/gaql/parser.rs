#![allow(dead_code)]

/// Validate a GAQL query string for basic syntax
pub fn validate_query(query: &str) -> crate::error::Result<()> {
    let query_upper = query.trim().to_uppercase();

    if !query_upper.starts_with("SELECT ") {
        return Err(crate::error::GadsError::Validation(
            "GAQL query must start with SELECT".into(),
        ));
    }

    if !query_upper.contains(" FROM ") {
        return Err(crate::error::GadsError::Validation(
            "GAQL query must contain FROM clause".into(),
        ));
    }

    // Check for common syntax errors
    if query.contains(';') {
        return Err(crate::error::GadsError::Validation(
            "GAQL queries should not end with semicolons".into(),
        ));
    }

    // Validate known clauses appear in correct order
    let clauses = ["SELECT", "FROM", "WHERE", "ORDER BY", "LIMIT", "PARAMETERS"];
    let mut last_pos = 0;
    for clause in &clauses {
        if let Some(pos) = query_upper.find(clause) {
            if pos < last_pos {
                return Err(crate::error::GadsError::Validation(format!(
                    "{} clause appears out of order",
                    clause
                )));
            }
            last_pos = pos;
        }
    }

    Ok(())
}

/// Extract the resource name from a GAQL query (the FROM clause value)
pub fn extract_resource(query: &str) -> Option<String> {
    let upper = query.to_uppercase();
    let from_idx = upper.find(" FROM ")?;
    let rest = &query[from_idx + 6..].trim();
    let end = rest.find(|c: char| c.is_whitespace()).unwrap_or(rest.len());
    Some(rest[..end].to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_query() {
        assert!(validate_query("SELECT campaign.id FROM campaign").is_ok());
        assert!(validate_query("SELECT campaign.id, campaign.name FROM campaign WHERE campaign.status = 'ENABLED' ORDER BY campaign.name LIMIT 10").is_ok());
    }

    #[test]
    fn test_validate_invalid_query() {
        assert!(validate_query("FROM campaign").is_err());
        assert!(validate_query("SELECT campaign.id").is_err());
        assert!(validate_query("SELECT campaign.id FROM campaign;").is_err());
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
    }
}
