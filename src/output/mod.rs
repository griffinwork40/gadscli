#![allow(dead_code)]

pub mod json;
pub mod table;
pub mod csv;
pub mod yaml;

use serde::Serialize;
use crate::types::common::OutputFormat;

/// Format and print data in the specified output format
pub fn format_output<T: Serialize>(data: &T, format: &OutputFormat) -> crate::error::Result<()> {
    match format {
        OutputFormat::Json => json::print(data),
        OutputFormat::Table => table::print(data),
        OutputFormat::Csv => csv::print(data),
        OutputFormat::Yaml => yaml::print(data),
    }
}

/// Format and print a list of items as a table with headers
pub fn format_table(headers: &[&str], rows: &[Vec<String>]) -> crate::error::Result<()> {
    table::print_table(headers, rows)
}

/// Format and print data as JSON
pub fn format_json<T: Serialize>(data: &T) -> crate::error::Result<()> {
    json::print(data)
}

/// Format and print data as NDJSON (one JSON object per line)
pub fn format_ndjson<T: Serialize>(items: &[T]) -> crate::error::Result<()> {
    json::print_ndjson(items)
}
