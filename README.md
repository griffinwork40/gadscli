# gadscli

A fast, ergonomic CLI for the Google Ads API, written in Rust.

## Features

- **16 command groups** — auth, config, account, campaign, ad-group, ad, keyword, budget, bidding, report, asset, conversion, label, recommendation, batch, field
- **GAQL reports** — run raw queries or pre-built report templates
- **Multiple output formats** — table, JSON, CSV, YAML
- **OAuth2 and service account authentication** — supports both interactive and server-to-server flows
- **Named profiles** — switch between accounts and configurations with `--profile`
- **Environment variable overrides** — all settings configurable via `GADS_*` vars
- **Dry-run mode** — validate mutations without executing them
- **Automatic pagination** — `--page-all` fetches every page transparently

## Installation

Requires [Rust](https://rustup.rs/) 1.70 or later.

```bash
git clone <repo>
cd google-ads-cli
cargo install --path .
```

Verify the install:

```bash
gadscli --version
```

## Quick Start

```bash
# 1. Initialize configuration interactively
gadscli config init

# 2. Authenticate with OAuth2
gadscli auth login

# 3. List campaigns
gadscli campaign list

# 4. Run a performance report
gadscli report run campaign-performance --date-range LAST_30_DAYS
```

## Configuration

The config file is stored at `~/.config/gadscli/config.toml`. Override the location with `GADS_CONFIG_DIR`.

### Environment Variables

| Variable | Description |
|---|---|
| `GADS_DEVELOPER_TOKEN` | Google Ads developer token |
| `GADS_CUSTOMER_ID` | Default customer ID (no hyphens) |
| `GADS_LOGIN_CUSTOMER_ID` | Login customer ID for MCC accounts |
| `GADS_CLIENT_ID` | OAuth2 client ID |
| `GADS_CLIENT_SECRET` | OAuth2 client secret |
| `GADS_REFRESH_TOKEN` | OAuth2 refresh token |
| `GADS_ACCESS_TOKEN` | Direct access token (bypasses OAuth flow) |
| `GADS_SERVICE_ACCOUNT_KEY` | Path to service account JSON key file |
| `GADS_SERVICE_ACCOUNT_SUBJECT` | Subject email for service account impersonation |
| `GADS_CONFIG_DIR` | Override config directory path |

### Named Profiles

Profiles let you store separate credentials for different accounts:

```toml
# ~/.config/gadscli/config.toml
[profiles.production]
customer_id = "1234567890"
developer_token = "..."

[profiles.staging]
customer_id = "0987654321"
developer_token = "..."
```

Use a profile with any command:

```bash
gadscli --profile production campaign list
```

## Output Formats

| Format | Flag | Description |
|---|---|---|
| Table | `--format table` | Human-readable ASCII table (default) |
| JSON | `--format json` | Machine-readable JSON |
| CSV | `--format csv` | Comma-separated values |
| YAML | `--format yaml` | YAML output |

## Global Flags

```
--customer-id <ID>         Customer ID (without hyphens)
--login-customer-id <ID>   Login customer ID for MCC accounts
--format <FORMAT>          Output format: table, json, csv, yaml [default: table]
--profile <NAME>           Config profile to use
--dry-run                  Validate mutations without executing
--verbose / -v             Verbose output
--quiet / -q               Suppress non-essential output
--page-size <N>            Page size for list operations
--page-all                 Fetch all pages automatically
--api-version <VERSION>    Override API version
```

## Documentation

- [Getting Started](docs/GETTING_STARTED.md)
- [Authentication](docs/AUTHENTICATION.md)
- [Command Reference](docs/COMMANDS.md)
- [GAQL Query Guide](docs/GAQL.md)

## License

MIT
