# Authentication

`gadscli` supports three authentication methods. Each method requires a Google Ads **developer token** in addition to the access credentials.

## Method 1: OAuth2 (Recommended)

OAuth2 is the standard method for user-facing applications. You provide a client ID, client secret, and obtain a refresh token through a browser-based authorization flow.

### Setup

1. Create OAuth2 credentials in Google Cloud Console (see [Getting Started](GETTING_STARTED.md))
2. Configure your credentials:

```bash
gadscli config set client-id YOUR_CLIENT_ID
gadscli config set client-secret YOUR_CLIENT_SECRET
gadscli config set developer-token YOUR_DEVELOPER_TOKEN
```

3. Run the login flow:

```bash
gadscli auth login
```

A browser window opens. Sign in with your Google account and grant access. The refresh token is automatically saved.

### How It Works

On each API call, `gadscli` exchanges your refresh token for a short-lived access token using the Google token endpoint. Access tokens are cached in memory and reused until they expire (typically 1 hour).

The refresh token is persisted to `~/.config/gadscli/credentials.enc` and reused across sessions.

### Commands

```bash
gadscli auth login     # Start OAuth2 flow, save refresh token
gadscli auth logout    # Clear saved credentials
gadscli auth status    # Show whether credentials are present and valid
gadscli auth whoami    # Show the authenticated user's info
```

## Method 2: Service Account

Service accounts are used for server-to-server authentication where no user interaction is possible (CI/CD pipelines, cron jobs, automation scripts).

### Setup

1. In Google Cloud Console, go to **IAM & Admin > Service Accounts**
2. Click **Create Service Account**, fill in the details
3. Click **Create and Continue**
4. On the Keys tab, click **Add Key > Create new key > JSON**
5. Save the downloaded JSON file securely

The service account must be granted access to your Google Ads account:

1. In Google Ads, go to **Tools & Settings > Setup > Access and security**
2. Add the service account's `client_email` as an account user

### Configuration

```bash
gadscli config set service-account-key-path /path/to/key.json
```

Or use environment variables:

```bash
export GADS_SERVICE_ACCOUNT_KEY=/path/to/key.json
export GADS_SERVICE_ACCOUNT_SUBJECT=user@example.com  # optional: for domain-wide delegation
```

### Domain-Wide Delegation

If your Google Workspace organization has granted domain-wide delegation to the service account, you can impersonate a user by setting the subject:

```bash
gadscli config set service-account-subject admin@yourdomain.com
```

## Method 3: Direct Access Token

For quick testing or scripting, you can provide a pre-obtained access token directly. This bypasses the OAuth flow entirely.

```bash
export GADS_ACCESS_TOKEN=ya29.your_access_token_here
gadscli campaign list
```

Or pass it via config:

```bash
gadscli config set access-token YOUR_ACCESS_TOKEN
```

Note: Access tokens expire after approximately 1 hour. This method is not suitable for long-running processes.

## Environment Variable Overrides

All authentication values can be set via environment variables. These override any values in the config file.

| Variable | Description |
|---|---|
| `GADS_DEVELOPER_TOKEN` | Google Ads developer token (required) |
| `GADS_CLIENT_ID` | OAuth2 client ID |
| `GADS_CLIENT_SECRET` | OAuth2 client secret |
| `GADS_REFRESH_TOKEN` | OAuth2 refresh token |
| `GADS_ACCESS_TOKEN` | Direct access token (overrides OAuth flow) |
| `GADS_SERVICE_ACCOUNT_KEY` | Path to service account JSON key file |
| `GADS_SERVICE_ACCOUNT_SUBJECT` | Subject email for service account impersonation |
| `GADS_CUSTOMER_ID` | Default customer ID (without hyphens) |
| `GADS_LOGIN_CUSTOMER_ID` | Login customer ID for MCC/manager accounts |
| `GADS_CONFIG_DIR` | Override config directory (default: `~/.config/gadscli`) |

## Credential Storage

| File | Contents |
|---|---|
| `~/.config/gadscli/config.toml` | Non-secret settings, customer IDs, profile definitions |
| `~/.config/gadscli/credentials.enc` | Encrypted refresh token and other sensitive credentials |

The config directory location can be overridden with `GADS_CONFIG_DIR`.

## Manager (MCC) Accounts

When accessing sub-accounts through a manager account, set the login customer ID to your manager account ID:

```bash
gadscli config set login-customer-id MANAGER_ACCOUNT_ID
gadscli --customer-id SUB_ACCOUNT_ID campaign list
```

Or use the `--login-customer-id` global flag:

```bash
gadscli --login-customer-id 1111111111 --customer-id 2222222222 campaign list
```

## Security Best Practices

- Never commit credentials to version control. Use `.env` files (add to `.gitignore`) or a secrets manager.
- Prefer OAuth2 with refresh tokens over direct access tokens for any persistent use.
- For production automation, use service accounts with minimal required permissions.
- Rotate refresh tokens and service account keys periodically.
- Use named profiles to separate test and production credentials rather than switching env vars manually.
- The `--dry-run` flag lets you validate mutation commands without making changes — use it when testing scripts.
