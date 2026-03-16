#![allow(dead_code)]

use crate::auth::{AuthProvider, token::TokenCache};
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};

const ADWORDS_SCOPE: &str = "https://www.googleapis.com/auth/adwords";

#[derive(Debug, Clone)]
pub struct ServiceAccountProvider {
    key_path: String,
    subject: Option<String>,
    developer_token: String,
    login_customer_id: Option<String>,
    token_cache: TokenCache,
}

#[derive(Deserialize)]
struct ServiceAccountKey {
    client_email: String,
    private_key: String,
    token_uri: String,
}

#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    scope: String,
    aud: String,
    iat: i64,
    exp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    sub: Option<String>,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
}

impl ServiceAccountProvider {
    pub fn new(
        key_path: String,
        subject: Option<String>,
        developer_token: String,
        login_customer_id: Option<String>,
    ) -> Self {
        Self {
            key_path,
            subject,
            developer_token,
            login_customer_id,
            token_cache: TokenCache::new(),
        }
    }

    async fn fetch_token(&self) -> crate::error::Result<String> {
        // Read the service account JSON key file
        let key_data = tokio::fs::read_to_string(&self.key_path)
            .await
            .map_err(|e| crate::error::GadsError::Auth(format!("Failed to read service account key file '{}': {e}", self.key_path)))?;

        let key: ServiceAccountKey = serde_json::from_str(&key_data)
            .map_err(|e| crate::error::GadsError::Auth(format!("Failed to parse service account key: {e}")))?;

        let now = chrono::Utc::now().timestamp();
        let claims = JwtClaims {
            iss: key.client_email.clone(),
            scope: ADWORDS_SCOPE.to_string(),
            aud: key.token_uri.clone(),
            iat: now,
            exp: now + 3600,
            sub: self.subject.clone(),
        };

        let encoding_key = EncodingKey::from_rsa_pem(key.private_key.as_bytes())
            .map_err(|e| crate::error::GadsError::Auth(format!("Failed to load RSA private key: {e}")))?;

        let header = Header::new(Algorithm::RS256);
        let jwt = encode(&header, &claims, &encoding_key)
            .map_err(|e| crate::error::GadsError::Auth(format!("Failed to sign JWT: {e}")))?;

        // Exchange the JWT for an access token
        let client = reqwest::Client::new();
        let params = [
            ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
            ("assertion", &jwt),
        ];

        let response = client
            .post(&key.token_uri)
            .form(&params)
            .send()
            .await
            .map_err(|e| crate::error::GadsError::Auth(format!("HTTP error during service account token fetch: {e}")))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            return Err(crate::error::GadsError::Auth(format!(
                "Service account token fetch failed ({}): {}",
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

impl AuthProvider for ServiceAccountProvider {
    fn access_token(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = crate::error::Result<String>> + Send + '_>> {
        Box::pin(async move {
            if let Some(token) = self.token_cache.get().await {
                return Ok(token);
            }
            self.fetch_token().await
        })
    }

    fn developer_token(&self) -> crate::error::Result<String> {
        Ok(self.developer_token.clone())
    }

    fn login_customer_id(&self) -> Option<String> {
        self.login_customer_id.clone()
    }
}

// ---------------------------------------------------------------------------
// StaticTokenProvider — used when the user supplies an access token directly
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct StaticTokenProvider {
    pub access_token: String,
    pub developer_token: String,
    pub login_customer_id: Option<String>,
}

impl StaticTokenProvider {
    pub fn new(
        access_token: String,
        developer_token: String,
        login_customer_id: Option<String>,
    ) -> Self {
        Self {
            access_token,
            developer_token,
            login_customer_id,
        }
    }
}

impl AuthProvider for StaticTokenProvider {
    fn access_token(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = crate::error::Result<String>> + Send + '_>> {
        let token = self.access_token.clone();
        Box::pin(async move { Ok(token) })
    }

    fn developer_token(&self) -> crate::error::Result<String> {
        Ok(self.developer_token.clone())
    }

    fn login_customer_id(&self) -> Option<String> {
        self.login_customer_id.clone()
    }
}
