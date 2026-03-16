#![allow(dead_code)]

use serde::Serialize;
use crate::error::Result;

/// Print data as RFC 4180 CSV
pub fn print<T: Serialize>(data: &T) -> Result<()> {
    let value = serde_json::to_value(data)
        .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;

    match value {
        serde_json::Value::Array(arr) => {
            if arr.is_empty() { return Ok(()); }
            if let Some(serde_json::Value::Object(first)) = arr.first() {
                let headers: Vec<&str> = first.keys().map(|k| k.as_str()).collect();

                let mut wtr = csv::Writer::from_writer(std::io::stdout());
                wtr.write_record(&headers)
                    .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;

                for item in &arr {
                    if let serde_json::Value::Object(obj) = item {
                        let row: Vec<String> = headers.iter().map(|h| {
                            format_csv_value(obj.get(*h))
                        }).collect();
                        wtr.write_record(&row)
                            .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;
                    }
                }
                wtr.flush().map_err(|e| crate::error::GadsError::Io(e))?;
            }
        }
        serde_json::Value::Object(obj) => {
            let mut wtr = csv::Writer::from_writer(std::io::stdout());
            let headers: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
            wtr.write_record(&headers)
                .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;
            let row: Vec<String> = headers.iter().map(|h| {
                format_csv_value(obj.get(*h))
            }).collect();
            wtr.write_record(&row)
                .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;
            wtr.flush().map_err(|e| crate::error::GadsError::Io(e))?;
        }
        _ => println!("{}", value),
    }
    Ok(())
}

fn format_csv_value(val: Option<&serde_json::Value>) -> String {
    match val {
        None | Some(serde_json::Value::Null) => String::new(),
        Some(serde_json::Value::String(s)) => s.clone(),
        Some(serde_json::Value::Number(n)) => n.to_string(),
        Some(serde_json::Value::Bool(b)) => b.to_string(),
        Some(v) => v.to_string(),
    }
}
