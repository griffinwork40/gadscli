# GAQL Query Guide

GAQL (Google Ads Query Language) is a SQL-like language for querying Google Ads data. `gadscli` lets you run raw GAQL queries directly or use pre-built report templates.

## Basic Syntax

```
SELECT <fields>
FROM <resource>
[WHERE <conditions>]
[ORDER BY <field> [ASC | DESC]]
[LIMIT <n>]
[PARAMETERS <options>]
```

### Example

```sql
SELECT
  campaign.id,
  campaign.name,
  metrics.impressions,
  metrics.clicks,
  metrics.cost_micros
FROM campaign
WHERE campaign.status = 'ENABLED'
ORDER BY metrics.cost_micros DESC
LIMIT 50
```

Run it:

```bash
gadscli report query "SELECT campaign.id, campaign.name, metrics.impressions, metrics.clicks, metrics.cost_micros FROM campaign WHERE campaign.status = 'ENABLED' ORDER BY metrics.cost_micros DESC LIMIT 50"
```

## Common Resources

| Resource | Description |
|---|---|
| `campaign` | Campaign-level data and metrics |
| `ad_group` | Ad group-level data and metrics |
| `ad_group_ad` | Individual ads within ad groups |
| `keyword_view` | Keyword performance data |
| `search_term_view` | Actual search terms that triggered ads |
| `geographic_view` | Performance by geographic location |
| `customer` | Account-level aggregate data |
| `campaign_budget` | Campaign budget details |
| `bidding_strategy` | Bidding strategy details |
| `asset` | Assets (text, images, sitelinks, etc.) |
| `conversion_action` | Conversion action definitions |
| `label` | Labels applied to resources |
| `recommendation` | Optimization recommendations |

Explore available fields for any resource:

```bash
gadscli field list campaign
gadscli field list ad_group
```

## Available Metrics

Metrics fields are prefixed with `metrics.`.

| Field | Description |
|---|---|
| `metrics.impressions` | Number of times the ad was shown |
| `metrics.clicks` | Number of clicks |
| `metrics.ctr` | Click-through rate (clicks / impressions) |
| `metrics.cost_micros` | Total cost in micros (divide by 1,000,000 for dollars) |
| `metrics.average_cpc` | Average cost per click in micros |
| `metrics.conversions` | Number of conversions |
| `metrics.conversions_value` | Total conversion value |
| `metrics.all_conversions` | All conversions including cross-device |
| `metrics.cost_per_conversion` | Cost per conversion in micros |
| `metrics.conversion_rate` | Conversions / clicks |
| `metrics.average_cpm` | Average cost per thousand impressions |
| `metrics.engagements` | Number of engagements |
| `metrics.interaction_rate` | Interactions / impressions |

## Segments

Segments split results by a dimension. Add a segment field to break down metrics.

| Field | Description |
|---|---|
| `segments.date` | Daily breakdown |
| `segments.week` | Weekly breakdown |
| `segments.month` | Monthly breakdown |
| `segments.device` | Device type (MOBILE, DESKTOP, TABLET) |
| `segments.hour` | Hour of day (0-23) |
| `segments.day_of_week` | Day of week |
| `segments.network` | Ad network |

```sql
-- Performance by device
SELECT segments.device, metrics.impressions, metrics.clicks, metrics.cost_micros
FROM campaign
WHERE campaign.status != 'REMOVED'
```

## Date Ranges

### Named Date Ranges

Use `--date-range` with `report run` or `--date-range` with `report query`:

| Value | Description |
|---|---|
| `TODAY` | Current day |
| `YESTERDAY` | Previous day |
| `LAST_7_DAYS` | Last 7 days (not including today) |
| `LAST_14_DAYS` | Last 14 days |
| `LAST_30_DAYS` | Last 30 days |
| `THIS_WEEK_SUN_TODAY` | This week, Sunday through today |
| `LAST_WEEK_SUN_SAT` | Last full week (Sunday to Saturday) |
| `THIS_MONTH` | Current calendar month |
| `LAST_MONTH` | Previous calendar month |

```bash
gadscli report query "SELECT campaign.name, metrics.clicks FROM campaign" --date-range LAST_7_DAYS
```

### Custom Date Ranges

Use `--start-date` and `--end-date` (format: `YYYY-MM-DD`):

```bash
gadscli report query \
  "SELECT campaign.name, metrics.clicks FROM campaign" \
  --start-date 2024-01-01 \
  --end-date 2024-01-31
```

### WHERE Clause Date Filters

You can also filter by date inline:

```sql
SELECT campaign.name, metrics.impressions
FROM campaign
WHERE segments.date DURING LAST_30_DAYS
```

```sql
SELECT campaign.name, metrics.clicks
FROM campaign
WHERE segments.date BETWEEN '2024-01-01' AND '2024-01-31'
```

## WHERE Clause Operators

| Operator | Example |
|---|---|
| `=` | `campaign.status = 'ENABLED'` |
| `!=` | `campaign.status != 'REMOVED'` |
| `>`, `<`, `>=`, `<=` | `metrics.impressions > 1000` |
| `IN` | `campaign.status IN ('ENABLED', 'PAUSED')` |
| `NOT IN` | `campaign.status NOT IN ('REMOVED')` |
| `LIKE` | `campaign.name LIKE '%brand%'` |
| `NOT LIKE` | `campaign.name NOT LIKE '%test%'` |
| `CONTAINS ANY` | `campaign.labels CONTAINS ANY ('label1')` |
| `DURING` | `segments.date DURING LAST_30_DAYS` |
| `BETWEEN` | `segments.date BETWEEN '2024-01-01' AND '2024-01-31'` |

Multiple conditions use `AND`:

```sql
WHERE campaign.status = 'ENABLED'
  AND metrics.impressions > 100
  AND metrics.cost_micros > 0
```

## Example Queries

### Top Campaigns by Spend

```sql
SELECT campaign.id, campaign.name, metrics.cost_micros, metrics.clicks, metrics.conversions
FROM campaign
WHERE campaign.status != 'REMOVED'
ORDER BY metrics.cost_micros DESC
LIMIT 10
```

### Keywords with Low Quality Score

```sql
SELECT
  ad_group_criterion.keyword.text,
  ad_group_criterion.keyword.match_type,
  ad_group_criterion.quality_info.quality_score
FROM keyword_view
WHERE ad_group_criterion.status = 'ENABLED'
  AND ad_group_criterion.quality_info.quality_score < 5
ORDER BY ad_group_criterion.quality_info.quality_score ASC
```

### Search Terms with Conversions

```sql
SELECT
  search_term_view.search_term,
  campaign.name,
  ad_group.name,
  metrics.impressions,
  metrics.clicks,
  metrics.conversions,
  metrics.cost_micros
FROM search_term_view
WHERE metrics.conversions > 0
ORDER BY metrics.conversions DESC
```

### Daily Performance Trend

```sql
SELECT
  segments.date,
  metrics.impressions,
  metrics.clicks,
  metrics.cost_micros,
  metrics.conversions
FROM campaign
WHERE campaign.status != 'REMOVED'
  AND segments.date DURING LAST_30_DAYS
ORDER BY segments.date ASC
```

### Account-Level Summary

```sql
SELECT
  metrics.impressions,
  metrics.clicks,
  metrics.ctr,
  metrics.cost_micros,
  metrics.conversions,
  metrics.conversions_value
FROM customer
```

### Ad Performance by Ad Group

```sql
SELECT
  ad_group.name,
  ad_group_ad.ad.id,
  ad_group_ad.ad.final_urls,
  ad_group_ad.status,
  metrics.impressions,
  metrics.clicks,
  metrics.ctr,
  metrics.cost_micros
FROM ad_group_ad
WHERE ad_group_ad.status != 'REMOVED'
ORDER BY metrics.impressions DESC
```

## Using the Query Builder Programmatically

If you are building tooling on top of `gadscli`, the `QueryBuilder` struct provides a fluent interface for constructing GAQL queries:

```rust
use gadscli::gaql::builder::QueryBuilder;

let query = QueryBuilder::new()
    .select(&[
        "campaign.id",
        "campaign.name",
        "metrics.impressions",
        "metrics.clicks",
        "metrics.cost_micros",
    ])
    .from("campaign")
    .where_not("campaign.status", "REMOVED")
    .where_if("campaign.status", "=", Some("ENABLED"))
    .order_by("metrics.cost_micros", true)  // true = DESC
    .limit(50)
    .build()?;
```

Helper methods:

| Method | Description |
|---|---|
| `.select(&[...])` | Add fields to SELECT clause |
| `.from("resource")` | Set the FROM resource |
| `.where_clause("condition")` | Add a raw WHERE condition |
| `.where_if(field, op, Option<value>)` | Add condition only if value is Some |
| `.where_not(field, value)` | Add a `field != 'value'` condition |
| `.order_by(field, is_desc)` | Add ORDER BY clause |
| `.limit(n)` | Set LIMIT |
| `.limit_if(Option<n>)` | Set LIMIT only if Some |
| `.parameters("opts")` | Add PARAMETERS clause |
| `.build()` | Build and return the query string |

## Pre-Built Report Templates

Run any template with:

```bash
gadscli report run <template-name> [--date-range <RANGE>]
```

List all templates:

```bash
gadscli report templates
```

### Available Templates

#### campaign-performance

Campaign performance metrics with spend, clicks, and conversions.

Default date range: `LAST_30_DAYS`

Fields: `campaign.id`, `campaign.name`, `campaign.status`, `metrics.impressions`, `metrics.clicks`, `metrics.ctr`, `metrics.cost_micros`, `metrics.conversions`, `metrics.conversions_value`, `metrics.average_cpc`

```bash
gadscli report run campaign-performance
gadscli report run campaign-performance --date-range LAST_7_DAYS --format csv
```

#### ad-group-performance

Ad group performance metrics.

Default date range: `LAST_30_DAYS`

Fields: `ad_group.id`, `ad_group.name`, `ad_group.status`, `ad_group.campaign`, `metrics.impressions`, `metrics.clicks`, `metrics.ctr`, `metrics.cost_micros`, `metrics.conversions`, `metrics.average_cpc`

```bash
gadscli report run ad-group-performance
```

#### keyword-performance

Keyword performance with quality score.

Default date range: `LAST_30_DAYS`

Fields: `ad_group_criterion.criterion_id`, `ad_group_criterion.keyword.text`, `ad_group_criterion.keyword.match_type`, `ad_group_criterion.status`, `metrics.impressions`, `metrics.clicks`, `metrics.ctr`, `metrics.cost_micros`, `metrics.conversions`, `metrics.average_cpc`

```bash
gadscli report run keyword-performance --format csv
```

#### search-terms

Search terms report showing actual queries that triggered ads.

Default date range: `LAST_30_DAYS`

Fields: `search_term_view.search_term`, `search_term_view.status`, `campaign.name`, `ad_group.name`, `metrics.impressions`, `metrics.clicks`, `metrics.cost_micros`, `metrics.conversions`

```bash
gadscli report run search-terms
```

#### quality-score

Quality score breakdown by keyword.

No default date range (quality score is not a time-based metric).

Fields: `ad_group_criterion.criterion_id`, `ad_group_criterion.keyword.text`, `ad_group_criterion.quality_info.quality_score`, `ad_group_criterion.quality_info.creative_quality_score`, `ad_group_criterion.quality_info.post_click_quality_score`, `ad_group_criterion.quality_info.search_predicted_ctr`

```bash
gadscli report run quality-score
gadscli report run quality-score --format json
```

#### account-summary

Overall account performance summary.

Default date range: `LAST_30_DAYS`

Fields: `metrics.impressions`, `metrics.clicks`, `metrics.ctr`, `metrics.cost_micros`, `metrics.conversions`, `metrics.conversions_value`, `metrics.all_conversions`, `metrics.average_cpc`

```bash
gadscli report run account-summary
```

#### geographic-performance

Performance breakdown by geographic location.

Default date range: `LAST_30_DAYS`

Fields: `geographic_view.country_criterion_id`, `geographic_view.location_type`, `metrics.impressions`, `metrics.clicks`, `metrics.cost_micros`, `metrics.conversions`

```bash
gadscli report run geographic-performance
```

#### device-performance

Performance breakdown by device type (mobile, desktop, tablet).

Default date range: `LAST_30_DAYS`

Fields: `segments.device`, `metrics.impressions`, `metrics.clicks`, `metrics.ctr`, `metrics.cost_micros`, `metrics.conversions`

```bash
gadscli report run device-performance --format csv
```

#### hourly-performance

Performance breakdown by hour of day.

Default date range: `LAST_7_DAYS`

Fields: `segments.hour`, `metrics.impressions`, `metrics.clicks`, `metrics.cost_micros`, `metrics.conversions`

```bash
gadscli report run hourly-performance
```
