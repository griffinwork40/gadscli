#![allow(dead_code)]

use std::path::PathBuf;

use rusqlite::{Connection, OpenFlags, OptionalExtension, params};

use crate::error::{GadsError, Result};

use super::types::*;

fn sqlite_err(e: rusqlite::Error) -> GadsError {
    GadsError::Other(format!("SQLite: {}", e))
}

pub struct EditorDatabase {
    conn: Connection,
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

    pub fn list_ads(
        &self,
        ad_group_local_id: Option<i64>,
    ) -> Result<Vec<(EditorAd, String, String)>> {
        let sql = if ad_group_local_id.is_some() {
            "SELECT ad.localId, ad.remoteId, ad.parentId, ad.status, \
             ad.headline1, ad.headline2, ad.headline3, ad.headline4, ad.headline5, \
             ad.headline6, ad.headline7, ad.headline8, ad.headline9, ad.headline10, \
             ad.headline11, ad.headline12, ad.headline13, ad.headline14, ad.headline15, \
             ad.description1, ad.description2, ad.description3, ad.description4, \
             ad.path1, ad.path2, ad.finalUrls, ad.state, \
             ag.name, c.name \
             FROM ResponsiveSearchAd ad \
             JOIN AdGroup ag ON (ad.parentId & 0xFFFFFFFF) = ag.localId \
             JOIN Campaign c ON (ag.parentId & 0xFFFFFFFF) = c.localId \
             WHERE (ad.parentId & 0xFFFFFFFF) = ?1 ORDER BY ag.name"
        } else {
            "SELECT ad.localId, ad.remoteId, ad.parentId, ad.status, \
             ad.headline1, ad.headline2, ad.headline3, ad.headline4, ad.headline5, \
             ad.headline6, ad.headline7, ad.headline8, ad.headline9, ad.headline10, \
             ad.headline11, ad.headline12, ad.headline13, ad.headline14, ad.headline15, \
             ad.description1, ad.description2, ad.description3, ad.description4, \
             ad.path1, ad.path2, ad.finalUrls, ad.state, \
             ag.name, c.name \
             FROM ResponsiveSearchAd ad \
             JOIN AdGroup ag ON (ad.parentId & 0xFFFFFFFF) = ag.localId \
             JOIN Campaign c ON (ag.parentId & 0xFFFFFFFF) = c.localId \
             ORDER BY c.name, ag.name"
        };

        let mut stmt = self.conn.prepare(sql).map_err(sqlite_err)?;

        let row_mapper = |row: &rusqlite::Row| {
            Ok((
                EditorAd {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    status: row.get(3)?,
                    headline1: row.get(4)?,
                    headline2: row.get(5)?,
                    headline3: row.get(6)?,
                    headline4: row.get(7)?,
                    headline5: row.get(8)?,
                    headline6: row.get(9)?,
                    headline7: row.get(10)?,
                    headline8: row.get(11)?,
                    headline9: row.get(12)?,
                    headline10: row.get(13)?,
                    headline11: row.get(14)?,
                    headline12: row.get(15)?,
                    headline13: row.get(16)?,
                    headline14: row.get(17)?,
                    headline15: row.get(18)?,
                    description1: row.get(19)?,
                    description2: row.get(20)?,
                    description3: row.get(21)?,
                    description4: row.get(22)?,
                    path1: row.get(23)?,
                    path2: row.get(24)?,
                    final_urls: row.get(25)?,
                    state: row.get(26)?,
                },
                row.get::<_, String>(27)?,
                row.get::<_, String>(28)?,
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

    pub fn list_negative_keywords(
        &self,
        campaign_local_id: Option<i64>,
    ) -> Result<Vec<EditorNegativeKeyword>> {
        let sql = if campaign_local_id.is_some() {
            "SELECT localId, remoteId, parentId, text, criterionType, status, state \
             FROM KeywordNegative WHERE (parentId & 0xFFFFFFFF) = ?1 ORDER BY text"
        } else {
            "SELECT localId, remoteId, parentId, text, criterionType, status, state \
             FROM KeywordNegative ORDER BY text"
        };

        let mut stmt = self.conn.prepare(sql).map_err(sqlite_err)?;
        let row_mapper = |row: &rusqlite::Row| {
            Ok(EditorNegativeKeyword {
                local_id: row.get(0)?,
                remote_id: row.get(1)?,
                parent_id: row.get(2)?,
                text: row.get(3)?,
                criterion_type: row.get(4)?,
                status: row.get(5)?,
                state: row.get(6)?,
            })
        };

        let rows = if let Some(id) = campaign_local_id {
            stmt.query_map(params![id], row_mapper).map_err(sqlite_err)?
        } else {
            stmt.query_map([], row_mapper).map_err(sqlite_err)?
        };

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_bidding_strategies(&self) -> Result<Vec<EditorBiddingStrategy>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, name, strategyType, state \
                 FROM BiddingStrategy ORDER BY name",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorBiddingStrategy {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    name: row.get(2)?,
                    strategy_type: row.get(3)?,
                    state: row.get(4)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_sitelinks(&self) -> Result<Vec<EditorSitelink>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, parentId, linkText, finalUrls, \
                 description1, description2, state \
                 FROM SitelinkV2 ORDER BY linkText",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorSitelink {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    link_text: row.get(3)?,
                    final_urls: row.get(4)?,
                    description1: row.get(5)?,
                    description2: row.get(6)?,
                    state: row.get(7)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_callouts(&self) -> Result<Vec<EditorCallout>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, parentId, text, state \
                 FROM CalloutV2 ORDER BY text",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorCallout {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    text: row.get(3)?,
                    state: row.get(4)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_structured_snippets(&self) -> Result<Vec<EditorStructuredSnippet>> {
        let mut stmt = self
            .conn
            .prepare(
                "SELECT localId, remoteId, parentId, header, \"values\", state \
                 FROM StructuredSnippetV2 ORDER BY header",
            )
            .map_err(sqlite_err)?;

        let rows = stmt
            .query_map([], |row| {
                Ok(EditorStructuredSnippet {
                    local_id: row.get(0)?,
                    remote_id: row.get(1)?,
                    parent_id: row.get(2)?,
                    header: row.get(3)?,
                    values: row.get(4)?,
                    state: row.get(5)?,
                })
            })
            .map_err(sqlite_err)?;

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_geo_targets(
        &self,
        campaign_local_id: Option<i64>,
    ) -> Result<Vec<EditorGeoTarget>> {
        let sql = if campaign_local_id.is_some() {
            "SELECT localId, remoteId, parentId, locationId, locationName, state \
             FROM GeoTarget WHERE (parentId & 0xFFFFFFFF) = ?1 ORDER BY locationName"
        } else {
            "SELECT localId, remoteId, parentId, locationId, locationName, state \
             FROM GeoTarget ORDER BY locationName"
        };

        let mut stmt = self.conn.prepare(sql).map_err(sqlite_err)?;
        let row_mapper = |row: &rusqlite::Row| {
            Ok(EditorGeoTarget {
                local_id: row.get(0)?,
                remote_id: row.get(1)?,
                parent_id: row.get(2)?,
                location_id: row.get(3)?,
                location_name: row.get(4)?,
                state: row.get(5)?,
            })
        };

        let rows = if let Some(id) = campaign_local_id {
            stmt.query_map(params![id], row_mapper).map_err(sqlite_err)?
        } else {
            stmt.query_map([], row_mapper).map_err(sqlite_err)?
        };

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

    pub fn list_audiences(
        &self,
        campaign_local_id: Option<i64>,
    ) -> Result<Vec<EditorAudience>> {
        let sql = if campaign_local_id.is_some() {
            "SELECT localId, remoteId, parentId, audienceId, audienceName, state \
             FROM Audience WHERE (parentId & 0xFFFFFFFF) = ?1 ORDER BY audienceName"
        } else {
            "SELECT localId, remoteId, parentId, audienceId, audienceName, state \
             FROM Audience ORDER BY audienceName"
        };

        let mut stmt = self.conn.prepare(sql).map_err(sqlite_err)?;
        let row_mapper = |row: &rusqlite::Row| {
            Ok(EditorAudience {
                local_id: row.get(0)?,
                remote_id: row.get(1)?,
                parent_id: row.get(2)?,
                audience_id: row.get(3)?,
                audience_name: row.get(4)?,
                state: row.get(5)?,
            })
        };

        let rows = if let Some(id) = campaign_local_id {
            stmt.query_map(params![id], row_mapper).map_err(sqlite_err)?
        } else {
            stmt.query_map([], row_mapper).map_err(sqlite_err)?
        };

        rows.collect::<std::result::Result<Vec<_>, _>>()
            .map_err(sqlite_err)
    }

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

/// Container ID encoding used by Editor: (tableType << 32) | localId
/// Table types: Campaign=2, AdGroup=4, Keyword=5
fn make_container_id(table_type: i64, local_id: i64) -> i64 {
    (table_type << 32) | local_id
}

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
