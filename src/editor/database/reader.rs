use std::path::PathBuf;

use rusqlite::{Connection, OpenFlags, params};

use super::sqlite_err;
use crate::editor::types::*;
use crate::error::{GadsError, Result};

pub struct EditorDatabase {
    pub(super) conn: Connection,
}

impl EditorDatabase {
    pub fn new(customer_id: u64) -> Result<Self> {
        let path = Self::db_path(customer_id);
        Self::open(&path)
    }

    pub fn open(path: &std::path::Path) -> Result<Self> {
        if !path.exists() {
            return Err(GadsError::Other(format!(
                "Editor database not found at {}",
                path.display()
            )));
        }

        let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_NO_MUTEX;
        let conn = Connection::open_with_flags(path, flags).map_err(sqlite_err)?;
        Ok(Self { conn })
    }

    pub fn data_dir() -> PathBuf {
        let mut dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        dir.push("Library/Application Support/Google/Google-AdWords-Editor/735");
        dir
    }

    pub fn db_path(customer_id: u64) -> PathBuf {
        let mut path = Self::data_dir();
        path.push(format!("ape_{}.db", customer_id));
        path
    }

    pub fn list_campaigns(&self) -> Result<Vec<EditorCampaign>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, name, status, campaignType, budgetAmount, \
                 biddingStrategyType, startDate, endDate, state \
                 FROM Campaign WHERE status != 4 ORDER BY name",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorCampaign {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    name: row.get(2)?,
                    status: row.get(3)?,
                    campaign_type: row.get(4)?,
                    budget_amount: row.get(5)?,
                    bidding_strategy_type: row.get(6)?,
                    start_date: row.get(7)?,
                    end_date: row.get(8)?,
                    state: row.get(9)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn get_campaign(&self, remote_id: i64) -> Result<Option<EditorCampaign>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, name, status, campaignType, budgetAmount, \
                 biddingStrategyType, startDate, endDate, state \
                 FROM Campaign WHERE remoteId = ?1",
            )
            .map_err(sqlite_err)?;

        let mut rows = stmt
            .query_map(params![remote_id], |row| {
                Ok(EditorCampaign {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    name: row.get(2)?,
                    status: row.get(3)?,
                    campaign_type: row.get(4)?,
                    budget_amount: row.get(5)?,
                    bidding_strategy_type: row.get(6)?,
                    start_date: row.get(7)?,
                    end_date: row.get(8)?,
                    state: row.get(9)?,
                })
            })
            .map_err(sqlite_err)?;

        match rows.next() {
            Some(Ok(c)) => Ok(Some(c)),
            Some(Err(e)) => Err(sqlite_err(e)),
            None => Ok(None),
        }
    }

    pub fn list_ad_groups(
        &self,
        campaign_local_id: Option<i64>,
    ) -> Result<Vec<(EditorAdGroup, String)>> {
        let sql = if campaign_local_id.is_some() {
            "SELECT ag.localId, ag.remoteId, ag.parentId, ag.name, ag.status, \
             ag.maxCpc, ag.state, c.name \
             FROM AdGroup ag JOIN Campaign c ON (ag.parentId & 0xFFFFFFFF) = c.localId \
             WHERE (ag.parentId & 0xFFFFFFFF) = ?1 ORDER BY c.name, ag.name"
        } else {
            "SELECT ag.localId, ag.remoteId, ag.parentId, ag.name, ag.status, \
             ag.maxCpc, ag.state, c.name \
             FROM AdGroup ag JOIN Campaign c ON (ag.parentId & 0xFFFFFFFF) = c.localId \
             ORDER BY c.name, ag.name"
        };

        let mut stmt = self.conn.prepare(sql).map_err(sqlite_err)?;

        let row_mapper = |row: &rusqlite::Row| {
            Ok((
                EditorAdGroup {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    name: row.get(3)?,
                    status: row.get(4)?,
                    max_cpc: row.get(5)?,
                    state: row.get(6)?,
                },
                row.get::<_, String>(7)?,
            ))
        };

        let rows = if let Some(id) = campaign_local_id {
            stmt.query_map(params![id], row_mapper).map_err(sqlite_err)?
        } else {
            stmt.query_map([], row_mapper).map_err(sqlite_err)?
        };

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_keywords(
        &self,
        ad_group_local_id: Option<i64>,
    ) -> Result<Vec<(EditorKeyword, String, String)>> {
        let sql = if ad_group_local_id.is_some() {
            "SELECT kw.localId, kw.remoteId, kw.parentId, kw.text, kw.criterionType, \
             kw.status, kw.maxCpc, kw.qualityScore, kw.state, \
             ag.name, c.name \
             FROM Keyword kw \
             JOIN AdGroup ag ON (kw.parentId & 0xFFFFFFFF) = ag.localId \
             JOIN Campaign c ON (ag.parentId & 0xFFFFFFFF) = c.localId \
             WHERE (kw.parentId & 0xFFFFFFFF) = ?1 ORDER BY kw.text"
        } else {
            "SELECT kw.localId, kw.remoteId, kw.parentId, kw.text, kw.criterionType, \
             kw.status, kw.maxCpc, kw.qualityScore, kw.state, \
             ag.name, c.name \
             FROM Keyword kw \
             JOIN AdGroup ag ON (kw.parentId & 0xFFFFFFFF) = ag.localId \
             JOIN Campaign c ON (ag.parentId & 0xFFFFFFFF) = c.localId \
             ORDER BY c.name, ag.name, kw.text"
        };

        let mut stmt = self.conn.prepare(sql).map_err(sqlite_err)?;

        let row_mapper = |row: &rusqlite::Row| {
            Ok((
                EditorKeyword {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    text: row.get(3)?,
                    criterion_type: row.get(4)?,
                    status: row.get(5)?,
                    max_cpc: row.get(6)?,
                    quality_score: row.get(7)?,
                    state: row.get(8)?,
                },
                row.get::<_, String>(9)?,
                row.get::<_, String>(10)?,
            ))
        };

        let rows = if let Some(id) = ad_group_local_id {
            stmt.query_map(params![id], row_mapper).map_err(sqlite_err)?
        } else {
            stmt.query_map([], row_mapper).map_err(sqlite_err)?
        };

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_budgets(&self) -> Result<Vec<EditorBudget>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, name, budgetAmount, status, state \
                 FROM Budget ORDER BY name",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorBudget {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    name: row.get(2)?,
                    budget_amount: row.get(3)?,
                    status: row.get(4)?,
                    state: row.get(5)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_labels(&self) -> Result<Vec<EditorLabel>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, name, description, color, state \
                 FROM Label ORDER BY name",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorLabel {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    name: row.get(2)?,
                    description: row.get(3)?,
                    color: row.get(4)?,
                    state: row.get(5)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn get_account_settings(&self) -> Result<EditorAccountSetting> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT name, currencyCode, timeZone, optimizationScore \
                 FROM AccountSetting LIMIT 1",
            )
            .map_err(sqlite_err)?;

        stmt.query_row([], |row| {
            Ok(EditorAccountSetting {
                name: row.get(0)?,
                currency_code: row.get(1)?,
                time_zone: row.get(2)?,
                optimization_score: row.get(3)?,
            })
        })
        .map_err(sqlite_err)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_dir() {
        let dir = EditorDatabase::data_dir();
        assert!(dir.to_str().unwrap().contains("Google-AdWords-Editor"));
        assert!(dir.to_str().unwrap().contains("735"));
    }

    #[test]
    fn test_db_path() {
        let path = EditorDatabase::db_path(1234567890);
        assert!(path.to_str().unwrap().ends_with("ape_1234567890.db"));
        assert!(path.to_str().unwrap().contains("Google-AdWords-Editor"));
    }
}
