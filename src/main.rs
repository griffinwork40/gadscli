#![allow(dead_code)]

pub mod auth;
pub mod cli;
pub mod client;
pub mod commands;
pub mod config;
pub mod error;
pub mod gaql;
pub mod output;
pub mod types;
pub mod util;

use clap::Parser;
use cli::{Cli, Commands};
use config::Config;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli).await {
        eprintln!("Error: {}", e);
        std::process::exit(e.exit_code());
    }
}

async fn run(cli: Cli) -> error::Result<()> {
    // Initialize tracing
    if cli.verbose {
        tracing_subscriber::fmt()
            .with_env_filter("gadscli=debug")
            .init();
    }

    // Load config
    let mut config = Config::load()?;
    config.apply_env_overrides();

    if let Some(profile) = &cli.profile {
        config.apply_profile(profile)?;
    }

    // Handle commands that don't need auth/client
    match &cli.command {
        Commands::Auth { command } => return commands::auth_cmd::handle(command, &config).await,
        Commands::Config { command } => return commands::config_cmd::handle(command, &mut config),
        _ => {}
    }

    // Build auth provider from config
    let creds = auth::Credentials::from_env_and_config(
        config.client_id.clone().unwrap_or_default(),
        config.client_secret.clone().unwrap_or_default(),
        config.refresh_token.clone(),
        config.developer_token.clone().unwrap_or_default(),
        cli.login_customer_id.clone().or(config.login_customer_id.clone()),
        config.access_token.clone(),
        config.service_account_key_path.clone(),
        config.service_account_subject.clone(),
    );
    let auth_provider = creds.into_provider();

    // Build client
    let api_version = cli.api_version.clone().unwrap_or(config.api_version.clone());
    let customer_id = cli.customer_id.clone().or(config.customer_id.clone());
    let gads_client = client::GoogleAdsClient::new(auth_provider, api_version, customer_id);

    // Dispatch commands
    commands::dispatch(&cli, &gads_client, &config).await
}
