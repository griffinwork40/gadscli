#![allow(dead_code)]

mod entities;
mod extensions;
mod csv_entries;

pub use entities::*;
pub use extensions::*;
pub use csv_entries::*;

pub(crate) fn status_to_str(status: i32) -> &'static str {
    match status {
        0 => "Enabled",
        2 => "Enabled",
        3 => "Paused",
        4 => "Removed",
        _ => "Unknown",
    }
}

pub(crate) fn state_to_str(state: i32) -> &'static str {
    match state {
        0 => "Normal",
        1 => "Edited",
        2 => "New",
        _ => "Unknown",
    }
}

pub(crate) fn micros_to_dollars(micros: Option<i64>) -> f64 {
    micros.map(|m| m as f64 / 1_000_000.0).unwrap_or(0.0)
}
