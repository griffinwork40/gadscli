# Command Reference

## Global Flags

These flags apply to every command.

| Flag | Env Var | Description |
|---|---|---|
| `--customer-id <ID>` | `GADS_CUSTOMER_ID` | Customer ID (without hyphens) |
| `--login-customer-id <ID>` | `GADS_LOGIN_CUSTOMER_ID` | Login customer ID for MCC accounts |
| `--format <FORMAT>` | — | Output format: `table`, `json`, `csv`, `yaml` (default: `table`) |
| `--profile <NAME>` | — | Named config profile to use |
| `--dry-run` | — | Validate mutations without executing |
| `--verbose` / `-v` | — | Verbose output |
| `--quiet` / `-q` | — | Suppress non-essential output |
| `--page-size <N>` | — | Results per page for list operations |
| `--page-all` | — | Automatically fetch all pages |
| `--api-version <VERSION>` | — | Override Google Ads API version |

---

## auth

Authentication management.

### auth login

Start the OAuth2 authorization flow and save credentials.

```bash
gadscli auth login
```

### auth logout

Clear saved credentials.

```bash
gadscli auth logout
```

### auth status

Show whether credentials are present and valid.

```bash
gadscli auth status
```

### auth whoami

Show information about the currently authenticated user.

```bash
gadscli auth whoami
```

---

## config

Configuration management. Settings are stored in `~/.config/gadscli/config.toml`.

### config init

Initialize configuration interactively. Prompts for all required values.

```bash
gadscli config init
```

### config set

Set a configuration value by key.

```bash
gadscli config set developer-token YOUR_TOKEN
gadscli config set customer-id 1234567890
gadscli config set client-id YOUR_CLIENT_ID
gadscli config set client-secret YOUR_CLIENT_SECRET
gadscli config set output-format json
gadscli config set page-size 500
gadscli config set api-version 18
```

Valid keys: `developer-token`, `customer-id`, `login-customer-id`, `client-id`, `client-secret`, `refresh-token`, `output-format`, `page-size`, `api-version`

### config get

Get the current value of a configuration key.

```bash
gadscli config get customer-id
gadscli config get output-format
```

### config list

List all configuration values.

```bash
gadscli config list
```

---

## account

Account management.

### account list

List all accounts accessible to the authenticated user.

```bash
gadscli account list
gadscli account list --format json
```

### account info

Show details for the current customer account.

```bash
gadscli account info
gadscli account info --customer-id 1234567890
```

### account hierarchy

Show the full account hierarchy for a manager account.

```bash
gadscli account hierarchy
gadscli account hierarchy --login-customer-id 1111111111
```

---

## campaign

Campaign management.

### campaign list

List campaigns with optional filters.

```bash
gadscli campaign list
gadscli campaign list --status ENABLED
gadscli campaign list --status PAUSED --limit 10
gadscli campaign list --format json
gadscli campaign list --page-all
```

### campaign get

Get details for a specific campaign.

```bash
gadscli campaign get 12345678
gadscli campaign get 12345678 --format json
```

### campaign create

Create a new campaign.

```bash
gadscli campaign create \
  --name "My Search Campaign" \
  --budget-id 98765432 \
  --campaign-type SEARCH \
  --bidding-strategy MANUAL_CPC
```

`--campaign-type` defaults to `SEARCH`. `--bidding-strategy` defaults to `MANUAL_CPC`.

### campaign update

Update a campaign's name or status.

```bash
gadscli campaign update 12345678 --name "Updated Name"
gadscli campaign update 12345678 --status PAUSED
gadscli campaign update 12345678 --name "New Name" --status ENABLED
```

### campaign pause

Pause a campaign.

```bash
gadscli campaign pause 12345678
gadscli campaign pause 12345678 --dry-run
```

### campaign enable

Enable a paused campaign.

```bash
gadscli campaign enable 12345678
```

### campaign remove

Remove (delete) a campaign.

```bash
gadscli campaign remove 12345678
gadscli campaign remove 12345678 --dry-run
```

---

## ad-group

Ad group management.

### ad-group list

List ad groups with optional filters.

```bash
gadscli ad-group list
gadscli ad-group list --campaign-id 12345678
gadscli ad-group list --status ENABLED
gadscli ad-group list --campaign-id 12345678 --status PAUSED
```

### ad-group get

Get details for a specific ad group.

```bash
gadscli ad-group get 87654321
```

### ad-group create

Create a new ad group within a campaign.

```bash
gadscli ad-group create \
  --campaign-id 12345678 \
  --name "My Ad Group" \
  --cpc-bid-micros 1000000
```

`--cpc-bid-micros` is the bid in micros (1,000,000 = $1.00).

### ad-group update

Update an ad group.

```bash
gadscli ad-group update 87654321 --name "Updated Ad Group"
gadscli ad-group update 87654321 --status PAUSED
gadscli ad-group update 87654321 --cpc-bid-micros 2000000
```

### ad-group pause

Pause an ad group.

```bash
gadscli ad-group pause 87654321
```

### ad-group enable

Enable a paused ad group.

```bash
gadscli ad-group enable 87654321
```

### ad-group remove

Remove an ad group.

```bash
gadscli ad-group remove 87654321
```

---

## ad

Ad management.

### ad list

List ads with an optional ad group filter.

```bash
gadscli ad list
gadscli ad list --ad-group-id 87654321
gadscli ad list --format csv
```

### ad get

Get details for a specific ad.

```bash
gadscli ad get 11223344
```

### ad create

Create a responsive search ad.

```bash
gadscli ad create \
  --ad-group-id 87654321 \
  --headlines "Headline One" "Headline Two" "Headline Three" \
  --descriptions "Description line one" "Description line two" \
  --final-url "https://example.com/landing-page"
```

### ad pause

Pause an ad.

```bash
gadscli ad pause 11223344
```

### ad enable

Enable a paused ad.

```bash
gadscli ad enable 11223344
```

### ad remove

Remove an ad.

```bash
gadscli ad remove 11223344
```

---

## keyword

Keyword management.

### keyword list

List keywords with optional filters.

```bash
gadscli keyword list
gadscli keyword list --ad-group-id 87654321
gadscli keyword list --campaign-id 12345678
gadscli keyword list --ad-group-id 87654321 --format table
```

### keyword add

Add a keyword to an ad group.

```bash
gadscli keyword add \
  --ad-group-id 87654321 \
  --text "running shoes" \
  --match-type EXACT

gadscli keyword add \
  --ad-group-id 87654321 \
  --text "buy sneakers" \
  --match-type PHRASE \
  --cpc-bid-micros 1500000
```

`--match-type` defaults to `BROAD`. Valid values: `BROAD`, `PHRASE`, `EXACT`.

### keyword update

Update a keyword's status or bid.

```bash
gadscli keyword update 55667788 --status PAUSED
gadscli keyword update 55667788 --cpc-bid-micros 2000000
```

### keyword remove

Remove a keyword.

```bash
gadscli keyword remove 55667788
```

---

## budget

Campaign budget management.

### budget list

List all campaign budgets.

```bash
gadscli budget list
gadscli budget list --format json
```

### budget get

Get details for a specific budget.

```bash
gadscli budget get 98765432
```

### budget create

Create a new campaign budget.

```bash
gadscli budget create --name "Daily Budget $50" --amount-micros 50000000
```

`--amount-micros` is the daily budget in micros (50,000,000 = $50.00).

### budget update

Update a budget's name or amount.

```bash
gadscli budget update 98765432 --name "Updated Budget"
gadscli budget update 98765432 --amount-micros 100000000
```

### budget remove

Remove a budget.

```bash
gadscli budget remove 98765432
```

---

## bidding

Bidding strategy management.

### bidding list

List all bidding strategies.

```bash
gadscli bidding list
```

### bidding get

Get details for a specific bidding strategy.

```bash
gadscli bidding get 44556677
```

### bidding create

Create a bidding strategy.

```bash
# Target CPA
gadscli bidding create \
  --name "Target CPA $10" \
  --strategy-type TARGET_CPA \
  --target-cpa-micros 10000000

# Target ROAS
gadscli bidding create \
  --name "Target ROAS 3x" \
  --strategy-type TARGET_ROAS \
  --target-roas 3.0

# Maximize conversions
gadscli bidding create \
  --name "Maximize Conversions" \
  --strategy-type MAXIMIZE_CONVERSIONS
```

### bidding update

Update a bidding strategy.

```bash
gadscli bidding update 44556677 --name "Updated Strategy"
gadscli bidding update 44556677 --target-cpa-micros 15000000
gadscli bidding update 44556677 --target-roas 4.0
```

### bidding remove

Remove a bidding strategy.

```bash
gadscli bidding remove 44556677
```

---

## report

Run GAQL reports and queries.

### report query

Execute a raw GAQL query.

```bash
gadscli report query "SELECT campaign.name, metrics.impressions FROM campaign"

gadscli report query \
  "SELECT campaign.name, metrics.clicks FROM campaign WHERE campaign.status = 'ENABLED'" \
  --date-range LAST_7_DAYS

gadscli report query \
  "SELECT campaign.name, metrics.cost_micros FROM campaign" \
  --start-date 2024-01-01 \
  --end-date 2024-01-31

gadscli report query "SELECT campaign.name, metrics.impressions FROM campaign" --format csv
```

### report run

Run a pre-built report template.

```bash
gadscli report run campaign-performance
gadscli report run campaign-performance --date-range LAST_7_DAYS
gadscli report run keyword-performance --format csv
gadscli report run search-terms --date-range LAST_30_DAYS
```

### report templates

List all available report templates.

```bash
gadscli report templates
```

---

## asset

Asset management.

### asset list

List assets with an optional type filter.

```bash
gadscli asset list
gadscli asset list --asset-type TEXT
gadscli asset list --asset-type IMAGE
```

### asset get

Get details for a specific asset.

```bash
gadscli asset get 22334455
```

### asset create

Create a new asset.

```bash
gadscli asset create \
  --name "My Sitelink Text" \
  --asset-type TEXT \
  --text-content "Shop Now"
```

### asset remove

Remove an asset.

```bash
gadscli asset remove 22334455
```

---

## conversion

Conversion action management.

### conversion list

List conversion actions.

```bash
gadscli conversion list
gadscli conversion list --format json
```

### conversion get

Get details for a specific conversion action.

```bash
gadscli conversion get 33445566
```

### conversion create

Create a conversion action.

```bash
gadscli conversion create \
  --name "Purchase" \
  --action-type WEBPAGE \
  --category PURCHASE

gadscli conversion create \
  --name "Phone Call" \
  --action-type PHONE_CALL
```

`--action-type` defaults to `WEBPAGE`.

### conversion update

Update a conversion action.

```bash
gadscli conversion update 33445566 --name "Updated Conversion"
gadscli conversion update 33445566 --status ENABLED
```

### conversion upload

Upload an offline conversion.

```bash
gadscli conversion upload \
  --conversion-action-id 33445566 \
  --gclid "EAIaIQobChMI..." \
  --conversion-date-time "2024-01-15 14:30:00+00:00" \
  --conversion-value 99.99
```

---

## label

Label management.

### label list

List all labels.

```bash
gadscli label list
```

### label get

Get details for a specific label.

```bash
gadscli label get 77889900
```

### label create

Create a label.

```bash
gadscli label create --name "High Priority"
gadscli label create --name "Seasonal" --description "Q4 holiday campaigns" --color "#FF0000"
```

### label update

Update a label.

```bash
gadscli label update 77889900 --name "Very High Priority"
gadscli label update 77889900 --description "Updated description"
```

### label remove

Remove a label.

```bash
gadscli label remove 77889900
```

### label assign

Assign a label to a resource.

```bash
gadscli label assign \
  --label-id 77889900 \
  --resource-type campaign \
  --resource-id 12345678

gadscli label assign \
  --label-id 77889900 \
  --resource-type ad_group \
  --resource-id 87654321
```

---

## recommendation

Recommendation management.

### recommendation list

List recommendations with an optional type filter.

```bash
gadscli recommendation list
gadscli recommendation list --recommendation-type KEYWORD
gadscli recommendation list --recommendation-type CAMPAIGN_BUDGET
```

### recommendation apply

Apply a recommendation.

```bash
gadscli recommendation apply REC_ID_HERE
gadscli recommendation apply REC_ID_HERE --dry-run
```

### recommendation dismiss

Dismiss a recommendation.

```bash
gadscli recommendation dismiss REC_ID_HERE
```

---

## batch

Batch job management for bulk operations.

### batch create

Create a new batch job.

```bash
gadscli batch create
```

### batch run

Run a batch job.

```bash
gadscli batch run BATCH_JOB_ID
```

### batch status

Check the status of a batch job.

```bash
gadscli batch status BATCH_JOB_ID
```

### batch results

Retrieve the results of a completed batch job.

```bash
gadscli batch results BATCH_JOB_ID
gadscli batch results BATCH_JOB_ID --format json
```

---

## field

Field metadata queries. Useful for exploring the Google Ads API schema.

### field list

List available fields for a resource type.

```bash
gadscli field list campaign
gadscli field list ad_group
gadscli field list keyword_view
```

### field search

Search for field metadata across resource types.

```bash
gadscli field search campaign
gadscli field search metrics.clicks
gadscli field search "keyword"
```
