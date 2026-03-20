use rusqlite::{Connection, OpenFlags, OptionalExtension, params};

use super::make_container_id;
use super::reader::EditorDatabase;
use super::sqlite_err;
use crate::error::{GadsError, Result};

pub struct EditorDatabaseWriter {
    conn: Connection,
}

impl EditorDatabaseWriter {
    pub fn new(customer_id: u64) -> Result<Self> {
        let path = EditorDatabase::db_path(customer_id);
        Self::open(&path)
    }

    pub fn open(path: &std::path::Path) -> Result<Self> {
        if !path.exists() {
            return Err(GadsError::Other(format!(
                "Editor database not found at {}",
                path.display()
            )));
        }

        let flags = OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        let conn = Connection::open_with_flags(path, flags).map_err(sqlite_err)?;
        Ok(Self { conn })
    }

    /// Find an ad group's localId by looking up the campaign name + ad group name
    pub fn find_ad_group(&self, campaign_name: &str, ad_group_name: &str) -> Result<Option<i64>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT ag.localId FROM AdGroup ag \
                 JOIN Campaign c ON (ag.parentId & 0xFFFFFFFF) = c.localId \
                 WHERE c.name = ?1 AND ag.name = ?2 LIMIT 1",
            )
            .map_err(sqlite_err)?;

        let result = stmt
            .query_row(params![campaign_name, ad_group_name], |row| row.get(0))
            .optional()
            .map_err(sqlite_err)?;

        Ok(result)
    }

    /// Add a keyword to an ad group. Sets state=2 (New) so Editor recognizes it as a pending add.
    pub fn add_keyword(
        &self,
        ad_group_local_id: i64,
        text: &str,
        criterion_type: i32,
        max_cpc_micros: i64,
    ) -> Result<i64> {
        let parent_id = make_container_id(4, ad_group_local_id); // 4 = AdGroup table type

        self.conn
            .execute(
                "INSERT INTO Keyword (parentId, state, text, criterionType, maxCpc, status) \
                 VALUES (?1, 2, ?2, ?3, ?4, 0)",
                params![parent_id, text, criterion_type, max_cpc_micros],
            )
            .map_err(sqlite_err)?;

        Ok(self.conn.last_insert_rowid())
    }

    /// Pause a keyword by setting status=3 and state=1 (Edited)
    pub fn pause_keyword(&self, local_id: i64) -> Result<()> {
        self.conn
            .execute(
                "UPDATE Keyword SET status = 3, status_revert = \
                 COALESCE(status_revert, status), state = 1 WHERE localId = ?1",
                params![local_id],
            )
            .map_err(sqlite_err)?;
        Ok(())
    }

    /// Enable a keyword by setting status=0 and state=1 (Edited)
    pub fn enable_keyword(&self, local_id: i64) -> Result<()> {
        self.conn
            .execute(
                "UPDATE Keyword SET status = 0, status_revert = \
                 COALESCE(status_revert, status), state = 1 WHERE localId = ?1",
                params![local_id],
            )
            .map_err(sqlite_err)?;
        Ok(())
    }

    /// Remove a keyword by setting status=4 and state=1 (Edited)
    pub fn remove_keyword(&self, local_id: i64) -> Result<()> {
        self.conn
            .execute(
                "UPDATE Keyword SET status = 4, status_revert = \
                 COALESCE(status_revert, status), state = 1 WHERE localId = ?1",
                params![local_id],
            )
            .map_err(sqlite_err)?;
        Ok(())
    }

    /// Update a campaign's status. state=1 (Edited).
    pub fn set_campaign_status(&self, local_id: i64, status: i32) -> Result<()> {
        self.conn
            .execute(
                "UPDATE Campaign SET status = ?1, status_revert = \
                 COALESCE(status_revert, status), state = 1 WHERE localId = ?2",
                params![status, local_id],
            )
            .map_err(sqlite_err)?;
        Ok(())
    }

    /// Update a campaign's budget in micros. state=1 (Edited).
    pub fn set_campaign_budget(&self, local_id: i64, budget_micros: i64) -> Result<()> {
        self.conn
            .execute(
                "UPDATE Campaign SET budgetAmount = ?1, budgetAmount_revert = \
                 COALESCE(budgetAmount_revert, budgetAmount), state = 1 WHERE localId = ?2",
                params![budget_micros, local_id],
            )
            .map_err(sqlite_err)?;
        Ok(())
    }
}
