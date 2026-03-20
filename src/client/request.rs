#![allow(dead_code)]

/// Helper for building API request payloads
pub struct ApiRequestBuilder {
    customer_id: String,
    base_url: String,
}

impl ApiRequestBuilder {
    pub fn new(customer_id: String, base_url: String) -> Self {
        Self { customer_id, base_url }
    }

    /// Build URL for search endpoint
    pub fn search_url(&self) -> String {
        format!("{}/customers/{}/googleAds:search", self.base_url, self.customer_id)
    }

    /// Build URL for searchStream endpoint
    pub fn search_stream_url(&self) -> String {
        format!("{}/customers/{}/googleAds:searchStream", self.base_url, self.customer_id)
    }

    /// Build URL for mutate endpoint
    pub fn mutate_url(&self, resource_type: &str) -> String {
        format!("{}/customers/{}/{}:mutate", self.base_url, self.customer_id, resource_type)
    }

    /// Build search request body
    pub fn search_body(
        query: &str,
        _page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> serde_json::Value {
        let mut body = serde_json::json!({
            "query": query,
        });

        // Note: pageSize is not supported in Google Ads API v20+.
        // The search endpoint uses a fixed page size of 10,000 rows.

        if let Some(token) = page_token {
            body["pageToken"] = serde_json::json!(token);
        }

        body
    }

    /// Build mutate request body
    pub fn mutate_body<T: serde::Serialize>(
        operations: &[crate::types::operations::MutateOperation<T>],
        partial_failure: bool,
        validate_only: bool,
    ) -> crate::error::Result<serde_json::Value> {
        let ops_value = serde_json::to_value(operations)
            .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;

        Ok(serde_json::json!({
            "operations": ops_value,
            "partialFailure": partial_failure,
            "validateOnly": validate_only,
        }))
    }
}
