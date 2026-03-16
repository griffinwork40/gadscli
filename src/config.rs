#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Default API version
const DEFAULT_API_VERSION: &str = "18";
/// Default page size for search queries
const DEFAULT_PAGE_SIZE: i32 = 1000;
/// Default output format
const DEFAULT_OUTPUT_FORMAT: &str = "table";

/// Application configuration loaded from ~/.config/gadscli/config.toml
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    /// Default customer ID (without hyphens)
    pub customer_id: Option<String>,
    /// Login customer ID for MCC accounts
    pub login_customer_id: Option<String>,
    /// Developer token for API access
    pub developer_token: Option<String>,
    /// OAuth2 client ID
    pub client_id: Option<String>,
    /// OAuth2 client secret
    pub client_secret: Option<String>,
    /// OAuth2 refresh token
    pub refresh_token: Option<String>,
    /// Direct access token (overrides OAuth flow)
    pub access_token: Option<String>,
    /// Service account key file path
    pub service_account_key_path: Option<String>,
    /// Service account impersonation subject
    pub service_account_subject: Option<String>,
    /// Default output format (json, table, csv, yaml)
    pub output_format: String,
    /// Default page size for list operations
    pub page_size: i32,
    /// Google Ads API version number
    pub api_version: String,
    /// Named profiles
    pub profiles: HashMap<String, Profile>,
}

/// A named profile that overrides specific config values
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Profile {
    pub customer_id: Option<String>,
    pub login_customer_id: Option<String>,
    pub developer_token: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub service_account_key_path: Option<String>,
    pub service_account_subject: Option<String>,
    pub output_format: Option<String>,
    pub page_size: Option<i32>,
    pub api_version: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            customer_id: None,
            login_customer_id: None,
            developer_token: None,
            client_id: None,
            client_secret: None,
            refresh_token: None,
            access_token: None,
            service_account_key_path: None,
            service_account_subject: None,
            output_format: DEFAULT_OUTPUT_FORMAT.to_string(),
            page_size: DEFAULT_PAGE_SIZE,
            api_version: DEFAULT_API_VERSION.to_string(),
            profiles: HashMap::new(),
        }
    }
}

impl Config {
    /// Load config from the default path (~/.config/gadscli/config.toml)
    /// Falls back to defaults if file doesn't exist
    pub fn load() -> crate::error::Result<Self> {
        let path = Self::config_path()?;
        Self::load_from(&path)
    }

    /// Load config from a specific path
    pub fn load_from(path: &PathBuf) -> crate::error::Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = std::fs::read_to_string(path)
            .map_err(|e| crate::error::GadsError::Config(format!("Failed to read config: {}", e)))?;
        let config: Config = toml::from_str(&content)
            .map_err(|e| crate::error::GadsError::Config(format!("Failed to parse config: {}", e)))?;
        Ok(config)
    }

    /// Save config to the default path
    pub fn save(&self) -> crate::error::Result<()> {
        let path = Self::config_path()?;
        self.save_to(&path)
    }

    /// Save config to a specific path
    pub fn save_to(&self, path: &PathBuf) -> crate::error::Result<()> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| crate::error::GadsError::Config(format!("Failed to create config directory: {}", e)))?;
        }
        let content = toml::to_string_pretty(self)
            .map_err(|e| crate::error::GadsError::Config(format!("Failed to serialize config: {}", e)))?;
        std::fs::write(path, content)
            .map_err(|e| crate::error::GadsError::Config(format!("Failed to write config: {}", e)))?;
        Ok(())
    }

    /// Get the config directory path, respecting GADS_CONFIG_DIR env var
    pub fn config_dir() -> crate::error::Result<PathBuf> {
        if let Ok(dir) = std::env::var("GADS_CONFIG_DIR") {
            return Ok(PathBuf::from(dir));
        }
        dirs::config_dir()
            .map(|d| d.join("gadscli"))
            .ok_or_else(|| crate::error::GadsError::Config("Could not determine config directory".into()))
    }

    /// Get the config file path
    pub fn config_path() -> crate::error::Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Get the credentials file path
    pub fn credentials_path() -> crate::error::Result<PathBuf> {
        Ok(Self::config_dir()?.join("credentials.enc"))
    }

    /// Apply environment variable overrides
    pub fn apply_env_overrides(&mut self) {
        if let Ok(val) = std::env::var("GADS_CUSTOMER_ID") {
            self.customer_id = Some(val);
        }
        if let Ok(val) = std::env::var("GADS_LOGIN_CUSTOMER_ID") {
            self.login_customer_id = Some(val);
        }
        if let Ok(val) = std::env::var("GADS_DEVELOPER_TOKEN") {
            self.developer_token = Some(val);
        }
        if let Ok(val) = std::env::var("GADS_ACCESS_TOKEN") {
            self.access_token = Some(val);
        }
        if let Ok(val) = std::env::var("GADS_CLIENT_ID") {
            self.client_id = Some(val);
        }
        if let Ok(val) = std::env::var("GADS_CLIENT_SECRET") {
            self.client_secret = Some(val);
        }
        if let Ok(val) = std::env::var("GADS_REFRESH_TOKEN") {
            self.refresh_token = Some(val);
        }
        if let Ok(val) = std::env::var("GADS_SERVICE_ACCOUNT_KEY") {
            self.service_account_key_path = Some(val);
        }
        if let Ok(val) = std::env::var("GADS_SERVICE_ACCOUNT_SUBJECT") {
            self.service_account_subject = Some(val);
        }
    }

    /// Apply a named profile's overrides
    pub fn apply_profile(&mut self, profile_name: &str) -> crate::error::Result<()> {
        let profile = self.profiles.get(profile_name)
            .ok_or_else(|| crate::error::GadsError::Config(format!("Profile '{}' not found", profile_name)))?
            .clone();

        if let Some(v) = profile.customer_id { self.customer_id = Some(v); }
        if let Some(v) = profile.login_customer_id { self.login_customer_id = Some(v); }
        if let Some(v) = profile.developer_token { self.developer_token = Some(v); }
        if let Some(v) = profile.client_id { self.client_id = Some(v); }
        if let Some(v) = profile.client_secret { self.client_secret = Some(v); }
        if let Some(v) = profile.refresh_token { self.refresh_token = Some(v); }
        if let Some(v) = profile.access_token { self.access_token = Some(v); }
        if let Some(v) = profile.service_account_key_path { self.service_account_key_path = Some(v); }
        if let Some(v) = profile.service_account_subject { self.service_account_subject = Some(v); }
        if let Some(v) = profile.output_format { self.output_format = v; }
        if let Some(v) = profile.page_size { self.page_size = v; }
        if let Some(v) = profile.api_version { self.api_version = v; }

        Ok(())
    }

    /// Get a config value by key name (for `gadscli config get <key>`)
    pub fn get_value(&self, key: &str) -> Option<String> {
        match key {
            "customer_id" | "customer-id" => self.customer_id.clone(),
            "login_customer_id" | "login-customer-id" => self.login_customer_id.clone(),
            "developer_token" | "developer-token" => self.developer_token.clone(),
            "client_id" | "client-id" => self.client_id.clone(),
            "client_secret" | "client-secret" => Some("[hidden]".to_string()),
            "refresh_token" | "refresh-token" => self.refresh_token.as_ref().map(|_| "[hidden]".to_string()),
            "output_format" | "output-format" => Some(self.output_format.clone()),
            "page_size" | "page-size" => Some(self.page_size.to_string()),
            "api_version" | "api-version" => Some(self.api_version.clone()),
            _ => None,
        }
    }

    /// Set a config value by key name (for `gadscli config set <key> <value>`)
    pub fn set_value(&mut self, key: &str, value: &str) -> crate::error::Result<()> {
        match key {
            "customer_id" | "customer-id" => self.customer_id = Some(Self::normalize_customer_id(value)),
            "login_customer_id" | "login-customer-id" => self.login_customer_id = Some(Self::normalize_customer_id(value)),
            "developer_token" | "developer-token" => self.developer_token = Some(value.to_string()),
            "client_id" | "client-id" => self.client_id = Some(value.to_string()),
            "client_secret" | "client-secret" => self.client_secret = Some(value.to_string()),
            "refresh_token" | "refresh-token" => self.refresh_token = Some(value.to_string()),
            "output_format" | "output-format" => {
                match value {
                    "json" | "table" | "csv" | "yaml" => self.output_format = value.to_string(),
                    _ => return Err(crate::error::GadsError::Validation(
                        format!("Invalid output format '{}'. Must be one of: json, table, csv, yaml", value)
                    )),
                }
            }
            "page_size" | "page-size" => {
                self.page_size = value.parse::<i32>()
                    .map_err(|_| crate::error::GadsError::Validation(format!("Invalid page size: {}", value)))?;
            }
            "api_version" | "api-version" => self.api_version = value.to_string(),
            _ => return Err(crate::error::GadsError::Validation(format!("Unknown config key: {}", key))),
        }
        Ok(())
    }

    /// List all config keys and their values (for `gadscli config list`)
    pub fn list_values(&self) -> Vec<(String, String)> {
        vec![
            ("customer_id".into(), self.customer_id.clone().unwrap_or_default()),
            ("login_customer_id".into(), self.login_customer_id.clone().unwrap_or_default()),
            ("developer_token".into(), self.developer_token.as_ref().map(|_| "[set]".to_string()).unwrap_or_default()),
            ("client_id".into(), self.client_id.clone().unwrap_or_default()),
            ("client_secret".into(), self.client_secret.as_ref().map(|_| "[set]".to_string()).unwrap_or_default()),
            ("refresh_token".into(), self.refresh_token.as_ref().map(|_| "[set]".to_string()).unwrap_or_default()),
            ("output_format".into(), self.output_format.clone()),
            ("page_size".into(), self.page_size.to_string()),
            ("api_version".into(), self.api_version.clone()),
        ]
    }

    /// Remove hyphens from customer ID (e.g., "123-456-7890" -> "1234567890")
    pub fn normalize_customer_id(id: &str) -> String {
        id.replace('-', "")
    }

    /// Initialize a new config file with interactive prompts
    pub fn init_interactive() -> crate::error::Result<Self> {
        // This will be implemented by the config command handler
        // For now just create a default config
        let config = Self::default();
        config.save()?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.api_version, "18");
        assert_eq!(config.page_size, 1000);
        assert_eq!(config.output_format, "table");
    }

    #[test]
    fn test_normalize_customer_id() {
        assert_eq!(Config::normalize_customer_id("123-456-7890"), "1234567890");
        assert_eq!(Config::normalize_customer_id("1234567890"), "1234567890");
    }

    #[test]
    fn test_set_invalid_output_format() {
        let mut config = Config::default();
        assert!(config.set_value("output_format", "xml").is_err());
        assert!(config.set_value("output_format", "json").is_ok());
    }

    #[test]
    fn test_get_set_value() {
        let mut config = Config::default();
        config.set_value("customer_id", "123-456-7890").unwrap();
        assert_eq!(config.get_value("customer_id"), Some("1234567890".to_string()));
    }

    #[test]
    fn test_config_roundtrip() {
        let config = Config::default();
        let toml_str = toml::to_string_pretty(&config).unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();
        assert_eq!(parsed.api_version, config.api_version);
        assert_eq!(parsed.page_size, config.page_size);
    }
}
