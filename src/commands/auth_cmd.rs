use crate::cli::AuthCommands;
use crate::config::Config;
use crate::error::{GadsError, Result};
use dialoguer::{Input, Password};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";
const GOOGLE_SCOPES: &str = "https://www.googleapis.com/auth/adwords";

pub async fn handle(command: &AuthCommands, config: &Config) -> Result<()> {
    match command {
        AuthCommands::Login => login(config).await,
        AuthCommands::Logout => logout(config),
        AuthCommands::Status => status(config),
        AuthCommands::Whoami => whoami(config).await,
    }
}

async fn login(config: &Config) -> Result<()> {
    println!("Google Ads CLI - OAuth2 Login");
    println!("------------------------------");

    let client_id: String = Input::new()
        .with_prompt("OAuth2 Client ID")
        .with_initial_text(config.client_id.clone().unwrap_or_default())
        .interact_text()
        .map_err(|e| GadsError::Auth(format!("Input error: {e}")))?;

    let client_secret: String = Password::new()
        .with_prompt("OAuth2 Client Secret")
        .interact()
        .map_err(|e| GadsError::Auth(format!("Input error: {e}")))?;

    // Bind a local server to receive the OAuth2 callback
    let listener = TcpListener::bind("127.0.0.1:8085")
        .await
        .map_err(|e| GadsError::Auth(format!("Failed to bind local server on port 8085: {e}")))?;
    let redirect_uri = "http://localhost:8085".to_string();

    // Build the authorization URL
    let auth_url = format!(
        "{}?client_id={}&redirect_uri={}&scope={}&response_type=code&access_type=offline&prompt=consent",
        GOOGLE_AUTH_URL,
        urlencoding(&client_id),
        urlencoding(&redirect_uri),
        urlencoding(GOOGLE_SCOPES),
    );

    println!("\nOpening browser for authorization...");
    if open_browser(&auth_url).is_err() {
        println!("Could not open browser. Visit the following URL manually:");
        println!("\n  {}\n", auth_url);
    }

    // Wait for the OAuth2 callback
    println!("Waiting for authorization...");
    let auth_code = wait_for_callback(listener).await?;

    // Exchange authorization code for tokens
    println!("Exchanging authorization code for tokens...");
    let refresh_token =
        exchange_code_for_token(&client_id, &client_secret, &auth_code, &redirect_uri).await?;

    // Save credentials to config
    let mut updated_config = config.clone();
    updated_config.client_id = Some(client_id);
    updated_config.client_secret = Some(client_secret);
    updated_config.refresh_token = Some(refresh_token);
    updated_config.save()?;

    println!("Login successful! Credentials saved to config.");
    Ok(())
}

fn open_browser(url: &str) -> std::result::Result<(), ()> {
    #[cfg(target_os = "macos")]
    let status = std::process::Command::new("open").arg(url).status();
    #[cfg(target_os = "linux")]
    let status = std::process::Command::new("xdg-open").arg(url).status();
    #[cfg(target_os = "windows")]
    let status = std::process::Command::new("cmd").args(["/C", "start", url]).status();

    match status {
        Ok(s) if s.success() => Ok(()),
        _ => Err(()),
    }
}

async fn wait_for_callback(listener: TcpListener) -> Result<String> {
    let (mut stream, _) = listener
        .accept()
        .await
        .map_err(|e| GadsError::Auth(format!("Failed to accept connection: {e}")))?;

    let mut buf = vec![0u8; 4096];
    let n = stream
        .read(&mut buf)
        .await
        .map_err(|e| GadsError::Auth(format!("Failed to read request: {e}")))?;
    let request = String::from_utf8_lossy(&buf[..n]);

    // Parse the authorization code from the query string (GET /?code=...&scope=...)
    let code = request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1)) // "/path?query"
        .and_then(|path| url::Url::parse(&format!("http://localhost{path}")).ok())
        .and_then(|url| {
            url.query_pairs()
                .find(|(k, _)| k == "code")
                .map(|(_, v)| v.into_owned())
        });

    // Check for error in the callback
    if code.is_none() {
        let error = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|path| url::Url::parse(&format!("http://localhost{path}")).ok())
            .and_then(|url| {
                url.query_pairs()
                    .find(|(k, _)| k == "error")
                    .map(|(_, v)| v.into_owned())
            });

        let html = if let Some(err) = &error {
            format!(
                "<html><body><h2>Authorization failed</h2><p>Error: {err}</p><p>You can close this tab.</p></body></html>"
            )
        } else {
            "<html><body><h2>Authorization failed</h2><p>No authorization code received.</p></body></html>".to_string()
        };

        let response = format!(
            "HTTP/1.1 400 Bad Request\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
            html.len(),
            html
        );
        let _ = stream.write_all(response.as_bytes()).await;
        let _ = stream.shutdown().await;

        return Err(GadsError::Auth(format!(
            "Authorization failed: {}",
            error.unwrap_or_else(|| "no code received".into())
        )));
    }

    // Send a success response to the browser
    let html = "<html><body><h2>Authorization successful!</h2><p>You can close this tab and return to the terminal.</p></body></html>";
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
        html.len(),
        html
    );
    let _ = stream.write_all(response.as_bytes()).await;
    let _ = stream.shutdown().await;

    Ok(code.unwrap())
}

async fn exchange_code_for_token(
    client_id: &str,
    client_secret: &str,
    auth_code: &str,
    redirect_uri: &str,
) -> Result<String> {
    #[derive(serde::Deserialize)]
    struct TokenResponse {
        refresh_token: Option<String>,
        error: Option<String>,
        error_description: Option<String>,
    }

    let client = reqwest::Client::new();
    let params = [
        ("grant_type", "authorization_code"),
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("code", auth_code),
    ];

    let response = client
        .post(GOOGLE_TOKEN_URL)
        .form(&params)
        .send()
        .await
        .map_err(|e| GadsError::Auth(format!("HTTP error during token exchange: {e}")))?;

    let token_response: TokenResponse = response
        .json()
        .await
        .map_err(|e| GadsError::Auth(format!("Failed to parse token response: {e}")))?;

    if let Some(err) = token_response.error {
        let desc = token_response.error_description.unwrap_or_default();
        return Err(GadsError::Auth(format!("Token exchange failed: {} - {}", err, desc)));
    }

    token_response
        .refresh_token
        .ok_or_else(|| GadsError::Auth("No refresh token in response. Ensure 'access_type=offline' and 'prompt=consent' were set.".into()))
}

fn logout(config: &Config) -> Result<()> {
    let mut updated_config = config.clone();
    updated_config.refresh_token = None;
    updated_config.access_token = None;
    updated_config.client_secret = None;
    updated_config.save()?;

    // Remove token cache file if it exists
    if let Ok(creds_path) = Config::credentials_path() {
        if creds_path.exists() {
            std::fs::remove_file(&creds_path)
                .map_err(GadsError::Io)?;
        }
    }

    println!("Logged out. Credentials cleared from config.");
    Ok(())
}

fn status(config: &Config) -> Result<()> {
    println!("Authentication Status");
    println!("---------------------");

    let has_client_id = config.client_id.is_some();
    let has_client_secret = config.client_secret.is_some();
    let has_refresh_token = config.refresh_token.is_some();
    let has_access_token = config.access_token.is_some();
    let has_service_account = config.service_account_key_path.is_some();
    let has_developer_token = config.developer_token.is_some();

    println!("Developer token:   {}", if has_developer_token { "set" } else { "not set" });
    println!("Client ID:         {}", if has_client_id { "set" } else { "not set" });
    println!("Client secret:     {}", if has_client_secret { "set" } else { "not set" });
    println!("Refresh token:     {}", if has_refresh_token { "set" } else { "not set" });
    println!("Access token:      {}", if has_access_token { "set (static override)" } else { "not set" });
    println!("Service account:   {}", if has_service_account { "set" } else { "not set" });

    println!();
    if has_access_token {
        println!("Auth method: Static access token");
        println!("Status: Ready");
    } else if has_service_account {
        println!("Auth method: Service account");
        println!("Status: Ready");
    } else if has_client_id && has_client_secret && has_refresh_token {
        println!("Auth method: OAuth2 refresh token");
        println!("Status: Ready");
    } else {
        println!("Status: Not configured");
        println!("Run 'gadscli auth login' to set up credentials.");
    }

    Ok(())
}

async fn whoami(config: &Config) -> Result<()> {
    // Validate credentials exist before attempting API call
    let has_credentials = config.access_token.is_some()
        || config.service_account_key_path.is_some()
        || (config.client_id.is_some() && config.client_secret.is_some() && config.refresh_token.is_some());

    if !has_credentials {
        return Err(GadsError::Auth(
            "Not authenticated. Run 'gadscli auth login' first.".into(),
        ));
    }

    let customer_id = config.customer_id.as_deref().ok_or_else(|| {
        GadsError::Config("No customer ID set. Use 'gadscli config set customer_id <id>'.".into())
    })?;

    // Build auth provider and client
    let creds = crate::auth::Credentials::from_env_and_config(
        config.client_id.clone().unwrap_or_default(),
        config.client_secret.clone().unwrap_or_default(),
        config.refresh_token.clone(),
        config.developer_token.clone().unwrap_or_default(),
        config.login_customer_id.clone(),
        config.access_token.clone(),
        config.service_account_key_path.clone(),
        config.service_account_subject.clone(),
    );
    let auth_provider = creds.into_provider();
    let client = crate::client::GoogleAdsClient::new(
        auth_provider,
        config.api_version.clone(),
        Some(customer_id.to_string()),
    );

    // Use the REST resource endpoint to get customer details
    let url = format!("{}/customers/{}", client.base_url(), customer_id);
    let response = client.http().execute(reqwest::Method::GET, &url, None).await?;

    println!("Authenticated Account");
    println!("---------------------");

    if let Some(v) = response.get("id") {
        println!("Customer ID:   {}", v.to_string().trim_matches('"'));
    }
    if let Some(v) = response.get("descriptiveName").and_then(|v| v.as_str()) {
        println!("Name:          {}", v);
    }
    if let Some(v) = response.get("currencyCode").and_then(|v| v.as_str()) {
        println!("Currency:      {}", v);
    }
    if let Some(v) = response.get("timeZone").and_then(|v| v.as_str()) {
        println!("Time zone:     {}", v);
    }
    if let Some(v) = response.get("status").and_then(|v| v.as_str()) {
        println!("Status:        {}", v);
    }

    Ok(())
}

/// Simple percent-encoding for URL parameters
fn urlencoding(s: &str) -> String {
    let mut encoded = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => encoded.push(c),
            ' ' => encoded.push('+'),
            _ => {
                for byte in c.to_string().as_bytes() {
                    encoded.push_str(&format!("%{:02X}", byte));
                }
            }
        }
    }
    encoded
}
