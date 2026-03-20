#![allow(dead_code)]

use reqwest::{Client, Method};

use crate::auth::AuthProvider;
use crate::error::{GadsError, Result};

pub struct HttpClient {
    client: Client,
    auth: Box<dyn AuthProvider>,
}

impl HttpClient {
    pub fn new(auth: Box<dyn AuthProvider>) -> Self {
        let client = Client::builder()
            .user_agent("gadscli/0.1.0")
            .timeout(std::time::Duration::from_secs(120))
            .build()
            .expect("Failed to create HTTP client");
        Self { client, auth }
    }

    /// Execute a request with auth headers and retry logic
    pub async fn execute(
        &self,
        method: Method,
        url: &str,
        body: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        self.execute_with_retry(method, url, body, 3).await
    }

    async fn execute_with_retry(
        &self,
        method: Method,
        url: &str,
        body: Option<serde_json::Value>,
        max_retries: u32,
    ) -> Result<serde_json::Value> {
        let mut attempt = 0u32;

        loop {
            let req = self.build_request(method.clone(), url, body.as_ref()).await?;
            let response = req
                .send()
                .await
                .map_err(|e| GadsError::Http(e.to_string()))?;

            let status = response.status().as_u16();

            if (status == 429 || status == 503)
                && attempt < max_retries {
                    let backoff = std::time::Duration::from_secs(1u64 << attempt);
                    tokio::time::sleep(backoff).await;
                    attempt += 1;
                    continue;
                }

            if status == 401 {
                return Err(GadsError::Auth(
                    "Unauthorized: invalid or expired access token".into(),
                ));
            }

            let response_text = response
                .text()
                .await
                .map_err(|e| GadsError::Http(format!("Failed to read response body: {}", e)))?;

            if std::env::var("GADS_DEBUG").is_ok() {
                eprintln!("[DEBUG] Status: {}", status);
                eprintln!("[DEBUG] Response ({} bytes): {}", response_text.len(), &response_text[..response_text.len().min(2000)]);
            }

            let response_body: serde_json::Value = serde_json::from_str(&response_text)
                .map_err(|e| {
                    if response_text.starts_with('<') {
                        GadsError::Http(format!("Server returned HTML instead of JSON (status {}). Check your API version.", status))
                    } else {
                        GadsError::Http(format!("Failed to parse response body: {}", e))
                    }
                })?;

            if status >= 400 {
                return Err(Self::parse_error(status, &response_body));
            }

            return Ok(response_body);
        }
    }

    /// Build request with auth headers
    async fn build_request(
        &self,
        method: Method,
        url: &str,
        body: Option<&serde_json::Value>,
    ) -> Result<reqwest::RequestBuilder> {
        let access_token = self.auth.access_token().await?;
        let developer_token = self.auth.developer_token()?;

        let mut req = self
            .client
            .request(method, url)
            .header("Authorization", format!("Bearer {}", access_token))
            .header("developer-token", &developer_token)
            .header("Content-Type", "application/json");

        if let Some(login_id) = self.auth.login_customer_id() {
            req = req.header("login-customer-id", login_id);
        }

        if let Some(body) = body {
            req = req.json(body);
        }

        Ok(req)
    }

    /// Parse API error response into GadsError
    fn parse_error(status: u16, body: &serde_json::Value) -> GadsError {
        let mut details = Vec::new();

        if let Some(error_details) = body
            .get("error")
            .and_then(|e| e.get("details"))
            .and_then(|d| d.as_array())
        {
            for detail in error_details {
                let error_code = detail
                    .get("errorCode")
                    .and_then(|c| c.as_str())
                    .unwrap_or("UNKNOWN")
                    .to_string();
                let msg = detail
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error")
                    .to_string();

                details.push(crate::error::ApiErrorDetail {
                    error_code,
                    message: msg,
                    trigger: detail
                        .get("trigger")
                        .and_then(|t| t.as_str())
                        .map(String::from),
                    location: detail
                        .get("location")
                        .and_then(|l| l.as_str())
                        .map(String::from),
                    field_path: detail
                        .get("fieldPath")
                        .and_then(|f| f.as_str())
                        .map(String::from),
                });
            }
        }

        let message = body
            .get("error")
            .and_then(|e| e.get("message"))
            .and_then(|m| m.as_str())
            .unwrap_or("Unknown API error")
            .to_string();

        if details.is_empty() {
            details.push(crate::error::ApiErrorDetail {
                error_code: format!("HTTP_{}", status),
                message: message.clone(),
                trigger: None,
                location: None,
                field_path: None,
            });
        }

        GadsError::Api {
            status,
            message,
            errors: details,
        }
    }
}
