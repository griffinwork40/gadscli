#![allow(dead_code)]

/// Manages the Google Ads developer token.
#[derive(Debug, Clone)]
pub struct DeveloperTokenProvider {
    token: String,
}

impl DeveloperTokenProvider {
    pub fn new(token: String) -> Self {
        Self { token }
    }

    pub fn token(&self) -> &str {
        &self.token
    }

    /// Reads the developer token from the `GADS_DEVELOPER_TOKEN` environment variable.
    pub fn from_env() -> Option<Self> {
        std::env::var("GADS_DEVELOPER_TOKEN").ok().map(Self::new)
    }
}
