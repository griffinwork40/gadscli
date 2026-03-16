#![allow(dead_code)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GadsError {
    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        errors: Vec<ApiErrorDetail>,
    },

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Rate limit exceeded: retry after {retry_after_seconds}s")]
    RateLimit { retry_after_seconds: u64 },

    #[error("Policy violation: {0}")]
    PolicyViolation(String),

    #[error("Partial failure: {succeeded} succeeded, {failed} failed")]
    PartialFailure {
        succeeded: usize,
        failed: usize,
        errors: Vec<ApiErrorDetail>,
    },

    #[error("HTTP error: {0}")]
    Http(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("{0}")]
    Other(String),
}

impl From<reqwest::Error> for GadsError {
    fn from(e: reqwest::Error) -> Self {
        GadsError::Http(e.to_string())
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ApiErrorDetail {
    pub error_code: String,
    pub message: String,
    pub trigger: Option<String>,
    pub location: Option<String>,
    pub field_path: Option<String>,
}

impl GadsError {
    pub fn exit_code(&self) -> i32 {
        match self {
            GadsError::Api { .. } => 1,
            GadsError::Auth(_) => 2,
            GadsError::Config(_) => 3,
            GadsError::Validation(_) => 4,
            GadsError::RateLimit { .. } => 5,
            GadsError::PolicyViolation(_) => 6,
            GadsError::PartialFailure { .. } => 7,
            _ => 1,
        }
    }
}

pub type Result<T> = std::result::Result<T, GadsError>;
