#![allow(dead_code)]

pub mod http;
pub mod rate_limiter;
pub mod request;

pub use http::HttpClient;
pub use rate_limiter::RateLimiter;
pub use request::ApiRequestBuilder;

use crate::auth::AuthProvider;

/// Main client for interacting with the Google Ads API
pub struct GoogleAdsClient {
    http_client: HttpClient,
    rate_limiter: RateLimiter,
    api_version: String,
    customer_id: Option<String>,
}

impl GoogleAdsClient {
    pub fn new(
        auth: Box<dyn AuthProvider>,
        api_version: String,
        customer_id: Option<String>,
    ) -> Self {
        Self {
            http_client: HttpClient::new(auth),
            rate_limiter: RateLimiter::new(100, std::time::Duration::from_secs(60)),
            api_version,
            customer_id,
        }
    }

    /// Get effective customer ID (from arg or stored)
    pub fn customer_id(&self, override_id: Option<&str>) -> crate::error::Result<String> {
        override_id
            .map(String::from)
            .or_else(|| self.customer_id.clone())
            .ok_or_else(|| {
                crate::error::GadsError::Config(
                    "No customer ID provided. Use --customer-id or set in config.".into(),
                )
            })
    }

    pub fn base_url(&self) -> String {
        format!("https://googleads.googleapis.com/v{}", self.api_version)
    }

    /// Execute a GAQL search query
    pub async fn search(
        &self,
        customer_id: &str,
        query: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> crate::error::Result<crate::types::responses::SearchResponse> {
        self.rate_limiter.acquire().await;

        let url = format!(
            "{}/customers/{}/googleAds:search",
            self.base_url(),
            customer_id
        );
        let body = ApiRequestBuilder::search_body(query, page_size, page_token);

        let response = self
            .http_client
            .execute(reqwest::Method::POST, &url, Some(body))
            .await?;

        serde_json::from_value(response)
            .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))
    }

    /// Execute a GAQL search query with automatic pagination (returns all results)
    pub async fn search_all(
        &self,
        customer_id: &str,
        query: &str,
        page_size: Option<i32>,
    ) -> crate::error::Result<Vec<crate::types::responses::SearchRow>> {
        let mut all_results = Vec::new();
        let mut page_token: Option<String> = None;

        loop {
            let response = self
                .search(
                    customer_id,
                    query,
                    page_size,
                    page_token.as_deref(),
                )
                .await?;

            all_results.extend(response.results);

            match response.next_page_token {
                Some(token) if !token.is_empty() => {
                    page_token = Some(token);
                }
                _ => break,
            }
        }

        Ok(all_results)
    }

    /// Execute a mutate operation
    pub async fn mutate<T: serde::Serialize>(
        &self,
        customer_id: &str,
        resource_type: &str,
        operations: Vec<crate::types::operations::MutateOperation<T>>,
        partial_failure: bool,
        validate_only: bool,
    ) -> crate::error::Result<crate::types::responses::MutateResponse> {
        self.rate_limiter.acquire().await;

        let url = format!(
            "{}/customers/{}/{}:mutate",
            self.base_url(),
            customer_id,
            resource_type
        );
        let body = ApiRequestBuilder::mutate_body(&operations, partial_failure, validate_only)?;

        let response = self
            .http_client
            .execute(reqwest::Method::POST, &url, Some(body))
            .await?;

        serde_json::from_value(response)
            .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))
    }

    /// Get a single resource by resource name
    pub async fn get_resource(
        &self,
        resource_name: &str,
    ) -> crate::error::Result<serde_json::Value> {
        self.rate_limiter.acquire().await;

        let url = format!("{}/{}", self.base_url(), resource_name);

        self.http_client
            .execute(reqwest::Method::GET, &url, None)
            .await
    }

    pub fn http(&self) -> &HttpClient {
        &self.http_client
    }

    pub fn api_version(&self) -> &str {
        &self.api_version
    }
}
