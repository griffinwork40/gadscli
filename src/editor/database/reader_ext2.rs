use rusqlite::params;

use super::reader::EditorDatabase;
use super::sqlite_err;
use crate::editor::types::*;
use crate::error::Result;

impl EditorDatabase {
    pub fn list_placements(&self) -> Result<Vec<EditorPlacement>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, parentId, url, state \
                 FROM Placement ORDER BY url",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorPlacement {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    url: row.get(3)?,
                    state: row.get(4)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_search_terms(
        &self,
        ad_group_local_id: Option<i64>,
    ) -> Result<Vec<EditorSearchTerm>> {
        let sql = if ad_group_local_id.is_some() {
            "SELECT localId, parentId, searchTerm, keywordText \
             FROM SearchTerm WHERE (parentId & 0xFFFFFFFF) = ?1 ORDER BY searchTerm"
        } else {
            "SELECT localId, parentId, searchTerm, keywordText \
             FROM SearchTerm ORDER BY searchTerm"
        };

        let mut stmt = self.conn.prepare(sql).map_err(sqlite_err)?;
        let row_mapper = |row: &rusqlite::Row| {
            Ok(EditorSearchTerm {
                local_id: row.get(0)?,
                parent_id: row.get(1)?,
                search_term: row.get(2)?,
                keyword_text: row.get(3)?,
            })
        };

        let rows = if let Some(id) = ad_group_local_id {
            stmt.query_map(params![id], row_mapper).map_err(sqlite_err)?
        } else {
            stmt.query_map([], row_mapper).map_err(sqlite_err)?
        };

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_negative_keyword_lists(&self) -> Result<Vec<EditorNegativeKeywordList>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, name, state \
                 FROM NegativeKeywordList ORDER BY name",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorNegativeKeywordList {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    name: row.get(2)?,
                    state: row.get(3)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_asset_groups(&self) -> Result<Vec<EditorAssetGroup>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, parentId, name, state \
                 FROM AssetGroup ORDER BY name",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorAssetGroup {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    name: row.get(3)?,
                    state: row.get(4)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn pending_changes(&self) -> Result<Vec<PendingChange>> {
        let mut changes = Vec::new();

        let tables = [
            ("Campaign", "name"),
            ("AdGroup", "name"),
            ("Keyword", "text"),
            ("ResponsiveSearchAd", "headline1"),
            ("Budget", "name"),
            ("Label", "name"),
            ("KeywordNegative", "text"),
            ("BiddingStrategy", "name"),
            ("SitelinkV2", "linkText"),
            ("CalloutV2", "text"),
            ("StructuredSnippetV2", "header"),
            ("GeoTarget", "locationName"),
            ("Audience", "audienceName"),
            ("Placement", "url"),
            ("NegativeKeywordList", "name"),
            ("AssetGroup", "name"),
        ];

        for (table, name_col) in &tables {
            let sql = format!(
                "SELECT localId, COALESCE({}, '(unnamed)'), state FROM {} WHERE state != 0",
                name_col, table
            );
            let mut stmt = self.conn.prepare(&sql).map_err(sqlite_err)?;
            let rows = stmt
                .query_map([], |row| {
                    Ok(PendingChange {
                        entity_type: table.to_string(),
                        local_id: row.get(0)?,
                        name: row.get(1)?,
                        state: row.get(2)?,
                    })
                })
                .map_err(sqlite_err)?;

            for row in rows {
                changes.push(row.map_err(sqlite_err)?);
            }
        }

        Ok(changes)
    }
}
