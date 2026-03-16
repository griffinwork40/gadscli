#![allow(dead_code)]

use comfy_table::{Table, ContentArrangement, presets::UTF8_FULL};
use serde::Serialize;
use crate::error::Result;

/// Print any serializable data as a table.
/// Serializes to JSON first, then renders as table.
pub fn print<T: Serialize>(data: &T) -> Result<()> {
    let value = serde_json::to_value(data)
        .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;

    match value {
        serde_json::Value::Array(arr) => {
            if arr.is_empty() {
                println!("No results found.");
                return Ok(());
            }
            // Extract headers from first object's keys
            if let Some(serde_json::Value::Object(first)) = arr.first() {
                let headers: Vec<&str> = first.keys().map(|k| k.as_str()).collect();
                let rows: Vec<Vec<String>> = arr.iter().map(|item| {
                    if let serde_json::Value::Object(obj) = item {
                        headers.iter().map(|h| format_value(obj.get(*h))).collect()
                    } else {
                        vec![item.to_string()]
                    }
                }).collect();
                print_table(&headers, &rows)?;
            }
        }
        serde_json::Value::Object(obj) => {
            // Single object: print as key-value pairs
            let mut table = Table::new();
            table.load_preset(UTF8_FULL);
            table.set_content_arrangement(ContentArrangement::Dynamic);
            table.set_header(vec!["Field", "Value"]);
            for (key, val) in &obj {
                table.add_row(vec![key.as_str(), &format_value(Some(val))]);
            }
            println!("{table}");
        }
        _ => println!("{}", value),
    }
    Ok(())
}

/// Print a table from explicit headers and rows
pub fn print_table(headers: &[&str], rows: &[Vec<String>]) -> Result<()> {
    if rows.is_empty() {
        println!("No results found.");
        return Ok(());
    }

    let mut table = Table::new();
    table.load_preset(UTF8_FULL);
    table.set_content_arrangement(ContentArrangement::Dynamic);
    table.set_header(headers.iter().map(|h| h.to_uppercase()));

    for row in rows {
        let colored_row: Vec<String> = row.iter().map(|cell| {
            colorize_status(cell)
        }).collect();
        table.add_row(colored_row);
    }

    println!("{table}");
    Ok(())
}

fn format_value(val: Option<&serde_json::Value>) -> String {
    match val {
        None => String::new(),
        Some(serde_json::Value::Null) => String::new(),
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(serde_json::Value::Bool(b)) => b.to_string(),
        Some(serde_json::Value::Array(a)) => {
            // join array elements
            a.iter().map(|v| format_value(Some(v))).collect::<Vec<_>>().join(", ")
        }
        Some(v) => v.to_string(),
    }
}

fn colorize_status(cell: &str) -> String {
    use colored::Colorize;
    match cell {
        "ENABLED" | "Enabled" => cell.green().to_string(),
        "PAUSED" | "Paused" => cell.yellow().to_string(),
        "REMOVED" | "Removed" => cell.red().to_string(),
        _ => cell.to_string(),
    }
}
