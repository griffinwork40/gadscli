#![allow(dead_code)]

use serde::Serialize;
use crate::error::Result;

/// Print data as YAML
pub fn print<T: Serialize>(data: &T) -> Result<()> {
    let yaml = serde_yaml::to_string(data)
        .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))?;
    print!("{}", yaml);
    Ok(())
}

/// Return YAML string without printing
pub fn to_string<T: Serialize>(data: &T) -> Result<String> {
    serde_yaml::to_string(data)
        .map_err(|e| crate::error::GadsError::Serialization(e.to_string()))
}
