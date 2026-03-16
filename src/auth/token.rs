#![allow(dead_code)]

use std::sync::Arc;
use tokio::sync::RwLock;
use chrono::{DateTime, Utc};

pub struct TokenCache {
    inner: Arc<RwLock<CachedToken>>,
}

struct CachedToken {
    access_token: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

impl Default for TokenCache {
    fn default() -> Self {
        Self::new()
    }
}

impl TokenCache {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(CachedToken {
                access_token: None,
                expires_at: None,
            })),
        }
    }

    /// Returns the cached token if it's still valid (with a 60s buffer before expiry).
    pub async fn get(&self) -> Option<String> {
        let guard = self.inner.read().await;
        match (&guard.access_token, &guard.expires_at) {
            (Some(token), Some(expires_at)) => {
                let now = Utc::now();
                let buffer = chrono::Duration::seconds(60);
                if now < *expires_at - buffer {
                    Some(token.clone())
                } else {
                    None
                }
            }
            (Some(token), None) => Some(token.clone()),
            _ => None,
        }
    }

    /// Store a token with a computed expiry based on `expires_in_secs`.
    pub async fn set(&self, token: String, expires_in_secs: u64) {
        let mut guard = self.inner.write().await;
        let expires_at = Utc::now() + chrono::Duration::seconds(expires_in_secs as i64);
        guard.access_token = Some(token);
        guard.expires_at = Some(expires_at);
    }

    /// Invalidate the cached token.
    pub async fn clear(&self) {
        let mut guard = self.inner.write().await;
        guard.access_token = None;
        guard.expires_at = None;
    }
}

impl Clone for TokenCache {
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
        }
    }
}

impl std::fmt::Debug for TokenCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenCache").finish_non_exhaustive()
    }
}
