use rusqlite::params;

use super::reader::EditorDatabase;
use super::sqlite_err;
use crate::editor::types::*;
use crate::error::Result;

impl EditorDatabase {
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

}
