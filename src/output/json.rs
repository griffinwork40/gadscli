#![allow(dead_code)]

use serde::Serialize;
use crate::error::Result;

/// Pretty-print JSON
pub fn print<T: Serialize>(data: &T) -> Result<()> {
    let json = serde_json::to_string_pretty(data)
        .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;
    println!("{}", json);
    Ok(())
}

/// Print NDJSON (newline-delimited JSON) — one object per line
pub fn print_ndjson<T: Serialize>(items: &[T]) -> Result<()> {
    for item in items {
        let json = serde_json::to_string(item)
            .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;
        println!("{}", json);
    }
    Ok(())
}

/// Return JSON string without printing
pub fn to_string<T: Serialize>(data: &T) -> Result<String> {
    serde_json::to_string_pretty(data)
        .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))
}
