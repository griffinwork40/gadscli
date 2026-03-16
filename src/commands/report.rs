use crate::cli::{Cli, ReportCommands};
use crate::client::GoogleAdsClient;
use crate::error::{GadsError, Result};

pub async fn handle(command: &ReportCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let customer_id = client.customer_id(cli.customer_id.as_deref())?;

    match command {
        ReportCommands::Query {
            query,
            date_range,
            start_date,
            end_date,
        } => {
            let full_query = build_query_with_date(query, date_range.as_deref(), start_date.as_deref(), end_date.as_deref());
            run_raw_query(client, &customer_id, &full_query, cli).await
        }

        ReportCommands::Run {
            template,
            date_range,
        } => {
            let query = resolve_template(template, date_range.as_deref())?;
            run_raw_query(client, &customer_id, &query, cli).await
        }

        ReportCommands::Templates => {
            list_templates();
            Ok(())
        }
    }
}

async fn run_raw_query(client: &GoogleAdsClient, customer_id: &str, query: &str, cli: &Cli) -> Result<()> {
    let page_size = cli.page_size;

    if cli.page_all {
        let rows = client.search_all(customer_id, query, page_size).await?;
        print_rows_as_json(&rows);
        eprintln!("\nTotal rows: {}", rows.len());
    } else {
        let response = client.search(customer_id, query, page_size, None).await?;
        print_rows_as_json(&response.results);
        if let Some(token) = &response.next_page_token {
            if !token.is_empty() {
                eprintln!("\n(More results available. Use --page-all to fetch all pages.)");
            }
        }
        eprintln!("\nRows returned: {}", response.results.len());
    }

    Ok(())
}

fn print_rows_as_json(rows: &[crate::types::responses::SearchRow]) {
    match serde_json::to_string_pretty(rows) {
        Ok(json) => println!("{}", json),
        Err(e) => eprintln!("Error serializing results: {}", e),
    }
}

fn build_query_with_date(
    query: &str,
    date_range: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> String {
    let mut q = query.to_string();

    if let Some(range) = date_range {
        let during_clause = format!(" DURING {}", range.to_uppercase());
        if !q.to_uppercase().contains("DURING") {
            q.push_str(&during_clause);
        }
    } else if let (Some(start), Some(end)) = (start_date, end_date) {
        let between_clause = format!(" BETWEEN '{}' AND '{}'", start, end);
        // Append to WHERE clause or add one
        if q.to_uppercase().contains(" WHERE ") {
            q.push_str(&format!(" AND segments.date {}", between_clause));
        } else {
            q.push_str(&format!(" WHERE segments.date {}", between_clause));
        }
    }

    q
}

fn resolve_template(name: &str, date_range: Option<&str>) -> Result<String> {
    let range_clause = date_range
        .map(|r| format!(" DURING {}", r.to_uppercase()))
        .unwrap_or_default();

    let query = match name.to_lowercase().as_str() {
        "campaign-performance" | "campaign_performance" => format!(
            "SELECT campaign.id, campaign.name, campaign.status, \
             metrics.impressions, metrics.clicks, metrics.cost_micros, \
             metrics.conversions, metrics.ctr \
             FROM campaign \
             WHERE campaign.status != 'REMOVED'{}",
            range_clause
        ),
        "ad-group-performance" | "ad_group_performance" => format!(
            "SELECT ad_group.id, ad_group.name, ad_group.status, \
             campaign.name, \
             metrics.impressions, metrics.clicks, metrics.cost_micros, \
             metrics.conversions \
             FROM ad_group \
             WHERE ad_group.status != 'REMOVED'{}",
            range_clause
        ),
        "keyword-performance" | "keyword_performance" => format!(
            "SELECT ad_group_criterion.keyword.text, ad_group_criterion.keyword.match_type, \
             ad_group_criterion.status, campaign.name, ad_group.name, \
             metrics.impressions, metrics.clicks, metrics.cost_micros, \
             metrics.conversions \
             FROM keyword_view \
             WHERE ad_group_criterion.status != 'REMOVED'{}",
            range_clause
        ),
        "search-terms" | "search_terms" => format!(
            "SELECT search_term_view.search_term, search_term_view.status, \
             campaign.name, ad_group.name, \
             metrics.impressions, metrics.clicks, metrics.cost_micros, \
             metrics.conversions \
             FROM search_term_view{}",
            range_clause
        ),
        "account-summary" | "account_summary" => format!(
            "SELECT customer.descriptive_name, customer.id, \
             metrics.impressions, metrics.clicks, metrics.cost_micros, \
             metrics.conversions \
             FROM customer{}",
            range_clause
        ),
        other => {
            return Err(GadsError::Validation(format!(
                "Unknown template '{}'. Run 'gadscli report templates' to list available templates.",
                other
            )));
        }
    };

    Ok(query)
}

fn list_templates() {
    println!("Available Report Templates");
    println!("--------------------------");
    println!("{:<30} {}", "NAME", "DESCRIPTION");
    println!("{}", "-".repeat(80));
    let templates = [
        ("campaign-performance", "Impressions, clicks, cost, conversions by campaign"),
        ("ad-group-performance", "Impressions, clicks, cost, conversions by ad group"),
        ("keyword-performance", "Impressions, clicks, cost, conversions by keyword"),
        ("search-terms", "Search terms report with performance metrics"),
        ("account-summary", "Top-level account performance summary"),
    ];
    for (name, desc) in &templates {
        println!("{:<30} {}", name, desc);
    }
    println!("\nUsage: gadscli report run <template-name> [--date-range LAST_30_DAYS]");
}
