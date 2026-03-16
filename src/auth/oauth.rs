#![allow(dead_code)]

use crate::auth::{AuthProvider, token::TokenCache};

const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

#[derive(Debug, Clone)]
pub struct OAuthProvider {
    client_id: String,
    client_secret: String,
    refresh_token: String,
    developer_token: String,
    login_customer_id: Option<String>,
    token_cache: TokenCache,
}

#[derive(serde::Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

impl OAuthProvider {
    pub fn new(
        client_id: String,
        client_secret: String,
        refresh_token: String,
        developer_token: String,
        login_customer_id: Option<String>,
    ) -> Self {
        Self {
            client_id,
            client_secret,
            refresh_token,
            developer_token,
            login_customer_id,
            token_cache: TokenCache::new(),
        }
    }

    async fn refresh_access_token(&self) -> crate::error::Result<String> {
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "refresh_token"),
            ("client_id", &self.client_id),
            ("client_secret", &self.client_secret),
            ("refresh_token", &self.refresh_token),
        ];

        let response = client
            .post(GOOGLE_TOKEN_URL)
            .form(&params)
            .send()
            .await
            .map_err(|e| crate::error::GadsError::Auth(format!("HTTP error during token refresh: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::GadsError::Auth(format!(
                "Token refresh failed ({}): {}",
                status, body
            )));
        }

        let token_response: TokenResponse = response
            .json()
            .await
            .map_err(|e| crate::error::GadsError::Auth(format!("Failed to parse token response: {e}")))?;

        self.token_cache
            .set(token_response.access_token.clone(), token_response.expires_in)
            .await;

        Ok(token_response.access_token)
    }
}

impl AuthProvider for OAuthProvider {
    fn access_token(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = crate::error::Result<String>> + Send + '_>> {
        Box::pin(async move {
            if let Some(token) = self.token_cache.get().await {
                return Ok(token);
            }
            self.refresh_access_token().await
        })
    }

    fn developer_token(&self) -> crate::error::Result<String> {
        Ok(self.developer_token.clone())
    }

    fn login_customer_id(&self) -> Option<String> {
        self.login_customer_id.clone()
    }
}
