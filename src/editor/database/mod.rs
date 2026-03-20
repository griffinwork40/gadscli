#![allow(dead_code)]

mod reader;
mod reader_ext;
mod reader_ext2;
mod writer;

pub use reader::EditorDatabase;
pub use writer::EditorDatabaseWriter;

use crate::error::GadsError;

pub(crate) fn sqlite_err(e: rusqlite::Error) -> GadsError {
    GadsError::Other(format!("SQLite: {}", e))
}

/// Container ID encoding used by Editor: (tableType << 32) | localId
pub(crate) fn make_container_id(table_type: i64, local_id: i64) -> i64 {
    (table_type << 32) | local_id
}
