#![allow(dead_code)]

pub mod developer_token;
pub mod oauth;
pub mod service_account;
pub mod token;

pub use developer_token::DeveloperTokenProvider;
pub use oauth::OAuthProvider;
pub use service_account::{ServiceAccountProvider, StaticTokenProvider};
pub use token::TokenCache;

/// Trait for providing authentication credentials to the Google Ads API.
pub trait AuthProvider: Send + Sync {
    /// Get a valid access token, refreshing if needed.
    fn access_token(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = crate::error::Result<String>> + Send + '_>>;

    /// Get the Google Ads developer token.
    fn developer_token(&self) -> crate::error::Result<String>;

    /// Get the login customer ID (for MCC/manager accounts).
    fn login_customer_id(&self) -> Option<String>;
}

/// All credentials needed to authenticate with the Google Ads API.
#[derive(Debug, Clone)]
pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub refresh_token: Option<String>,
    pub developer_token: String,
    pub login_customer_id: Option<String>,
    /// Direct access token override — skips OAuth flow entirely.
    pub access_token: Option<String>,
    pub service_account_key_path: Option<String>,
    pub service_account_subject: Option<String>,
}

impl Credentials {
    /// Build credentials by overlaying environment variables on top of provided config values.
    ///
    /// Environment variables take precedence:
    /// - `GADS_CLIENT_ID`, `GADS_CLIENT_SECRET`, `GADS_REFRESH_TOKEN`
    /// - `GADS_DEVELOPER_TOKEN`, `GADS_LOGIN_CUSTOMER_ID`
    /// - `GADS_ACCESS_TOKEN`
    /// - `GADS_SERVICE_ACCOUNT_KEY`, `GADS_SERVICE_ACCOUNT_SUBJECT`
    pub fn from_env_and_config(
        client_id: String,
        client_secret: String,
        refresh_token: Option<String>,
        developer_token: String,
        login_customer_id: Option<String>,
        access_token: Option<String>,
        service_account_key_path: Option<String>,
        service_account_subject: Option<String>,
    ) -> Self {
        fn env(key: &str) -> Option<String> {
            std::env::var(key).ok().filter(|v| !v.is_empty())
        }

        Self {
            client_id: env("GADS_CLIENT_ID").unwrap_or(client_id),
            client_secret: env("GADS_CLIENT_SECRET").unwrap_or(client_secret),
            refresh_token: env("GADS_REFRESH_TOKEN").or(refresh_token),
            developer_token: env("GADS_DEVELOPER_TOKEN").unwrap_or(developer_token),
            login_customer_id: env("GADS_LOGIN_CUSTOMER_ID").or(login_customer_id),
            access_token: env("GADS_ACCESS_TOKEN").or(access_token),
            service_account_key_path: env("GADS_SERVICE_ACCOUNT_KEY").or(service_account_key_path),
            service_account_subject: env("GADS_SERVICE_ACCOUNT_SUBJECT").or(service_account_subject),
        }
    }

    /// Choose the appropriate `AuthProvider` based on available credentials.
    ///
    /// Priority:
    /// 1. Static access token (if `access_token` is set)
    /// 2. Service account (if `service_account_key_path` is set)
    /// 3. OAuth refresh token flow (default)
    pub fn into_provider(self) -> Box<dyn AuthProvider> {
        if let Some(token) = self.access_token {
            return Box::new(StaticTokenProvider::new(
                token,
                self.developer_token,
                self.login_customer_id,
            ));
        }

        if let Some(key_path) = self.service_account_key_path {
            return Box::new(ServiceAccountProvider::new(
                key_path,
                self.service_account_subject,
                self.developer_token,
                self.login_customer_id,
            ));
        }

        Box::new(OAuthProvider::new(
            self.client_id,
            self.client_secret,
            self.refresh_token.unwrap_or_default(),
            self.developer_token,
            self.login_customer_id,
        ))
    }
}
