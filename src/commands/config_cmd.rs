use crate::cli::ConfigCommands;
use crate::config::Config;
use crate::error::{GadsError, Result};
use dialoguer::{Input, Password};

pub fn handle(command: &ConfigCommands, config: &mut Config) -> Result<()> {
    match command {
        ConfigCommands::Set { key, value } => set(key, value, config),
        ConfigCommands::Get { key } => get(key, config),
        ConfigCommands::List => list(config),
        ConfigCommands::Init => init(config),
    }
}

fn set(key: &str, value: &str, config: &mut Config) -> Result<()> {
    config.set_value(key, value)?;
    config.save()?;
    println!("Set {} = {}", key, display_value(key, value));
    Ok(())
}

fn get(key: &str, config: &Config) -> Result<()> {
    match config.get_value(key) {
        Some(value) => {
            println!("{}", value);
            Ok(())
        }
        None => Err(GadsError::Config(format!("Unknown config key: '{}'", key))),
    }
}

fn list(config: &Config) -> Result<()> {
    let values = config.list_values();
    println!("{:<25} {}", "KEY", "VALUE");
    println!("{}", "-".repeat(50));
    for (key, value) in &values {
        let display = if value.is_empty() { "(not set)" } else { value.as_str() };
        println!("{:<25} {}", key, display);
    }
    Ok(())
}

fn init(config: &mut Config) -> Result<()> {
    println!("Google Ads CLI - Configuration Setup");
    println!("-------------------------------------");
    println!("Press Enter to keep existing values.\n");

    let client_id: String = Input::new()
        .with_prompt("OAuth2 Client ID")
        .with_initial_text(config.client_id.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()
        .map_err(|e| GadsError::Config(format!("Input error: {e}")))?;

    let client_secret: String = Password::new()
        .with_prompt("OAuth2 Client Secret (leave blank to keep existing)")
        .allow_empty_password(true)
        .interact()
        .map_err(|e| GadsError::Config(format!("Input error: {e}")))?;

    let developer_token: String = Password::new()
        .with_prompt("Developer Token (leave blank to keep existing)")
        .allow_empty_password(true)
        .interact()
        .map_err(|e| GadsError::Config(format!("Input error: {e}")))?;

    let customer_id: String = Input::new()
        .with_prompt("Default Customer ID (without hyphens, optional)")
        .with_initial_text(config.customer_id.clone().unwrap_or_default())
        .allow_empty(true)
        .interact_text()
        .map_err(|e| GadsError::Config(format!("Input error: {e}")))?;

    // Apply non-empty values
    if !client_id.is_empty() {
        config.client_id = Some(client_id);
    }
    if !client_secret.is_empty() {
        config.client_secret = Some(client_secret);
    }
    if !developer_token.is_empty() {
        config.developer_token = Some(developer_token);
    }
    if !customer_id.is_empty() {
        config.customer_id = Some(Config::normalize_customer_id(&customer_id));
    }

    config.save()?;

    println!("\nConfiguration saved!");
    println!("Run 'gadscli auth login' to complete OAuth2 setup.");
    Ok(())
}

/// Display-safe version: returns either the value or a mask
pub fn display_value(key: &str, value: &str) -> String {
    match key {
        "client_secret" | "client-secret"
        | "refresh_token" | "refresh-token"
        | "developer_token" | "developer-token" => "[hidden]".to_string(),
        _ => value.to_string(),
    }
}
