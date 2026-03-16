# Getting Started

This guide walks you from zero to running your first Google Ads query.

## Prerequisites

- **Rust 1.70+** — install via [rustup.rs](https://rustup.rs/)
- **A Google Cloud project** — you will create OAuth2 credentials here
- **A Google Ads account** — either a standard account or a manager (MCC) account
- **A Google Ads developer token** — required for all API access

## 1. Installation

```bash
git clone <repo>
cd google-ads-cli
cargo install --path .

# Verify
gadscli --version
```

## 2. Google Cloud Setup

### Create a Google Cloud Project

1. Go to the [Google Cloud Console](https://console.cloud.google.com/)
2. Click **New Project**, give it a name, and click **Create**
3. Note your **Project ID**

### Enable the Google Ads API

1. In your project, go to **APIs & Services > Library**
2. Search for "Google Ads API"
3. Click **Enable**

### Create OAuth2 Credentials

1. Go to **APIs & Services > Credentials**
2. Click **Create Credentials > OAuth client ID**
3. If prompted, configure the OAuth consent screen first:
   - Set User Type to **External** (or Internal if using a Google Workspace org)
   - Fill in the required fields (app name, support email)
   - Add the scope `https://www.googleapis.com/auth/adwords`
   - Add your email as a test user
4. Back in Create Credentials, choose **Desktop app** as the application type
5. Give it a name and click **Create**
6. Download or copy your **Client ID** and **Client Secret**

## 3. Get a Developer Token

A developer token is required for all Google Ads API requests. It is tied to a **manager (MCC) account**, not a Google Cloud project.

1. Sign in to your Google Ads manager account
2. Go to **Tools & Settings > Setup > API Center**
3. Accept the Terms of Service if prompted
4. Your developer token is shown on this page
5. New tokens start in **test mode** — they can only access test accounts. Apply for basic or standard access to use production accounts

## 4. Initial Configuration

Run the interactive setup wizard:

```bash
gadscli config init
```

This will prompt you for:
- Developer token
- OAuth2 client ID and client secret
- Default customer ID
- Preferred output format

You can also set values individually:

```bash
gadscli config set developer-token YOUR_TOKEN
gadscli config set client-id YOUR_CLIENT_ID
gadscli config set client-secret YOUR_CLIENT_SECRET
gadscli config set customer-id 1234567890
```

The config is saved to `~/.config/gadscli/config.toml`.

## 5. First Authentication

Run the OAuth2 login flow:

```bash
gadscli auth login
```

This opens a browser window where you authorize the app. After authorizing, the refresh token is saved to `~/.config/gadscli/credentials.enc`.

Verify authentication succeeded:

```bash
gadscli auth status
gadscli auth whoami
```

## 6. Verify Setup

List accounts you have access to:

```bash
gadscli account list
```

If you are using a manager account, check the account hierarchy:

```bash
gadscli account hierarchy
```

## 7. Running Your First Report

List your campaigns:

```bash
gadscli campaign list
```

Run a pre-built performance report:

```bash
gadscli report run campaign-performance
```

Run a raw GAQL query:

```bash
gadscli report query "SELECT campaign.name, metrics.impressions, metrics.clicks FROM campaign WHERE campaign.status = 'ENABLED'"
```

See available report templates:

```bash
gadscli report templates
```

## Next Steps

- [Authentication Guide](AUTHENTICATION.md) — service accounts, access tokens, env vars
- [Command Reference](COMMANDS.md) — every command with examples
- [GAQL Guide](GAQL.md) — writing and running custom queries
