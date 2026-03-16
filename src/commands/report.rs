use crate::cli::{Cli, FormatOption, ReportCommands};
use crate::client::GoogleAdsClient;
use crate::error::{GadsError, Result};
use crate::gaql::parser::validate_query;
use crate::gaql::templates;
use crate::output;
use crate::types::common::OutputFormat;
use crate::types::query::DateRange;

pub async fn handle(command: &ReportCommands, client: &GoogleAdsClient, cli: &Cli) -> Result<()> {
    let format = cli_format_to_output(&cli.format);

    match command {
        ReportCommands::Query {
            query,
            date_range,
            start_date,
            end_date,
        } => {
            validate_query(query)?;
            let full_query = apply_date_range(
                query,
                date_range.as_deref(),
                start_date.as_deref(),
                end_date.as_deref(),
            );
            let customer_id = client.customer_id(cli.customer_id.as_deref())?;
            run_query(client, &customer_id, &full_query, cli, &format).await
        }

        ReportCommands::Run {
            template,
            date_range,
        } => {
            let tmpl = templates::get_template(template).ok_or_else(|| {
                let available: Vec<String> = templates::get_all_templates()
                    .iter()
                    .map(|t| format!("  {}", t.name))
                    .collect();
                GadsError::Validation(format!(
                    "Unknown template '{}'. Available templates:\n{}",
                    template,
                    available.join("\n")
                ))
            })?;

            // Use override date range if provided, otherwise fall back to template default
            let effective_date_range = date_range
                .as_deref()
                .or(tmpl.default_date_range.as_deref());
            let full_query = apply_date_range(&tmpl.query, effective_date_range, None, None);

            if !cli.quiet {
                eprintln!("Template: {}", tmpl.name);
                eprintln!("Description: {}", tmpl.description);
                if let Some(dr) = &effective_date_range {
                    eprintln!("Date range: {}", dr);
                }
                eprintln!();
            }

            let customer_id = client.customer_id(cli.customer_id.as_deref())?;
            run_query(client, &customer_id, &full_query, cli, &format).await
        }

        ReportCommands::Templates => {
            list_templates();
            Ok(())
        }
    }
}

async fn run_query(
    client: &GoogleAdsClient,
    customer_id: &str,
    query: &str,
    cli: &Cli,
    format: &OutputFormat,
) -> Result<()> {
    let rows = if cli.page_all {
        client.search_all(customer_id, query, cli.page_size).await?
    } else {
        let response = client.search(customer_id, query, cli.page_size, None).await?;
        if let Some(token) = &response.next_page_token {
            if !token.is_empty() && !cli.quiet {
                eprintln!("(More results available. Use --page-all to fetch all pages.)");
            }
        }
        response.results
    };

    let count = rows.len();
    output::format_output(&rows, format)?;

    if !cli.quiet {
        eprintln!("\n{} result{} returned", count, if count == 1 { "" } else { "s" });
    }

    Ok(())
}

/// Convert CLI FormatOption to the OutputFormat used by the output module.
fn cli_format_to_output(fmt: &FormatOption) -> OutputFormat {
    match fmt {
        FormatOption::Json => OutputFormat::Json,
        FormatOption::Table => OutputFormat::Table,
        FormatOption::Csv => OutputFormat::Csv,
        FormatOption::Yaml => OutputFormat::Yaml,
    }
}

/// Apply date range constraints to a GAQL query.
///
/// Priority order:
/// 1. `--start-date` / `--end-date` explicit range → BETWEEN clause
/// 2. `--date-range` named range → resolved to BETWEEN if a DateRange helper exists,
///    otherwise appended as DURING <NAME>
///
/// If the query already contains a WHERE clause the new condition is ANDed in;
/// otherwise a new WHERE clause is inserted before any ORDER BY / LIMIT.
fn apply_date_range(
    query: &str,
    date_range: Option<&str>,
    start_date: Option<&str>,
    end_date: Option<&str>,
) -> String {
    if let (Some(start), Some(end)) = (start_date, end_date) {
        let condition = format!("segments.date BETWEEN '{}' AND '{}'", start, end);
        return inject_where(query, &condition);
    }

    if let Some(range) = date_range {
        // Try to resolve to an explicit date range first; otherwise use DURING keyword.
        if let Some(dr) = resolve_named_date_range(range) {
            let condition = format!(
                "segments.date BETWEEN '{}' AND '{}'",
                dr.start_date, dr.end_date
            );
            return inject_where(query, &condition);
        }
        // Unknown named range — pass through as DURING clause (Google Ads supports these
        // server-side for ranges like LAST_30_DAYS, THIS_MONTH, etc.)
        let upper = range.to_uppercase();
        if !query.to_uppercase().contains("DURING") {
            return format!("{} DURING {}", query.trim_end(), upper);
        }
    }

    query.to_string()
}

/// Resolve a named date range string to a `DateRange` with concrete start/end dates.
fn resolve_named_date_range(name: &str) -> Option<DateRange> {
    match name.to_uppercase().as_str() {
        "TODAY" => Some(DateRange::today()),
        "YESTERDAY" => Some(DateRange::yesterday()),
        "LAST_7_DAYS" => Some(DateRange::last_7_days()),
        "LAST_30_DAYS" => Some(DateRange::last_30_days()),
        "THIS_MONTH" => Some(DateRange::this_month()),
        "LAST_MONTH" => Some(DateRange::last_month()),
        _ => None,
    }
}

/// Inject a WHERE condition into a GAQL query.
///
/// - If a WHERE clause already exists, AND the condition.
/// - Otherwise insert `WHERE <condition>` before ORDER BY / LIMIT / PARAMETERS,
///   or append at end if none of those clauses are present.
fn inject_where(query: &str, condition: &str) -> String {
    let upper = query.to_uppercase();

    if upper.contains(" WHERE ") {
        // Find the position just before ORDER BY / LIMIT / PARAMETERS so we don't
        // accidentally insert after them.
        let insert_before = ["ORDER BY", "LIMIT", "PARAMETERS"]
            .iter()
            .filter_map(|kw| upper.find(kw))
            .min();

        if let Some(pos) = insert_before {
            let (before, after) = query.split_at(pos);
            format!("{} AND {} {}", before.trim_end(), condition, after.trim_start())
        } else {
            format!("{} AND {}", query.trim_end(), condition)
        }
    } else {
        // No WHERE clause yet — find insertion point before ORDER BY / LIMIT / PARAMETERS.
        let insert_before = ["ORDER BY", "LIMIT", "PARAMETERS"]
            .iter()
            .filter_map(|kw| upper.find(kw))
            .min();

        if let Some(pos) = insert_before {
            let (before, after) = query.split_at(pos);
            format!(
                "{} WHERE {} {}",
                before.trim_end(),
                condition,
                after.trim_start()
            )
        } else {
            format!("{} WHERE {}", query.trim_end(), condition)
        }
    }
}

fn list_templates() {
    let templates = templates::get_all_templates();

    // Column widths
    let name_w = 28usize;
    let desc_w = 48usize;
    let range_w = 16usize;

    let header = format!(
        "{:<name_w$}  {:<desc_w$}  {:<range_w$}",
        "NAME", "DESCRIPTION", "DEFAULT DATE RANGE",
        name_w = name_w,
        desc_w = desc_w,
        range_w = range_w,
    );
    let separator = "-".repeat(header.len());

    println!("Available Report Templates");
    println!("{}", separator);
    println!("{}", header);
    println!("{}", separator);

    for tmpl in &templates {
        let range = tmpl
            .default_date_range
            .as_deref()
            .unwrap_or("(none)");
        println!(
            "{:<name_w$}  {:<desc_w$}  {:<range_w$}",
            tmpl.name,
            tmpl.description,
            range,
            name_w = name_w,
            desc_w = desc_w,
            range_w = range_w,
        );
    }

    println!();
    println!("Usage: gadscli report run <template-name> [--date-range LAST_30_DAYS]");
}
