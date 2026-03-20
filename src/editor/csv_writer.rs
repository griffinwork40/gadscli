#![allow(dead_code)]

use std::path::Path;

use crate::error::{GadsError, Result};

use super::types::{
    AdEntry, AdGroupEntry, BudgetEntry, CalloutEntry, CampaignEntry, KeywordEntry,
    LabelEntry, NegativeKeywordEntry, SitelinkEntry,
};

pub fn write_keyword_csv(path: &Path, entries: &[KeywordEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record(["Campaign", "Ad group", "Keyword", "Match type", "Max CPC", "Status"])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        let cpc = entry
            .max_cpc
            .map(|v| format!("{:.2}", v))
            .unwrap_or_default();

        wtr.write_record([
            &entry.campaign,
            &entry.ad_group,
            &entry.keyword,
            &entry.match_type,
            &cpc,
            &entry.status,
        ])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;

    Ok(())
}

pub fn write_campaign_csv(path: &Path, entries: &[CampaignEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record(["Campaign", "Budget", "Campaign status", "Bid strategy type"])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        wtr.write_record([
            &entry.campaign,
            &format!("{:.2}", entry.budget),
            &entry.status,
            &entry.bidding_strategy,
        ])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;

    Ok(())
}

pub fn write_ad_csv(path: &Path, entries: &[AdEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record([
        "Campaign",
        "Ad group",
        "Headline 1",
        "Headline 2",
        "Headline 3",
        "Description 1",
        "Description 2",
        "Final URL",
        "Path 1",
        "Path 2",
        "Ad status",
    ])
    .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        let h1 = entry.headlines.first().map(|s| s.as_str()).unwrap_or("");
        let h2 = entry.headlines.get(1).map(|s| s.as_str()).unwrap_or("");
        let h3 = entry.headlines.get(2).map(|s| s.as_str()).unwrap_or("");
        let d1 = entry.descriptions.first().map(|s| s.as_str()).unwrap_or("");
        let d2 = entry.descriptions.get(1).map(|s| s.as_str()).unwrap_or("");
        let p1 = entry.path1.as_deref().unwrap_or("");
        let p2 = entry.path2.as_deref().unwrap_or("");

        wtr.write_record([
            &entry.campaign,
            &entry.ad_group,
            h1,
            h2,
            h3,
            d1,
            d2,
            &entry.final_url,
            p1,
            p2,
            &entry.status,
        ])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;

    Ok(())
}

pub fn write_ad_group_csv(path: &Path, entries: &[AdGroupEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record(["Campaign", "Ad group", "Max CPC", "Ad group status"])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        let cpc = entry
            .max_cpc
            .map(|v| format!("{:.2}", v))
            .unwrap_or_default();

        wtr.write_record([&entry.campaign, &entry.ad_group, &cpc, &entry.status])
            .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;
    Ok(())
}

pub fn write_negative_keyword_csv(path: &Path, entries: &[NegativeKeywordEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record(["Campaign", "Ad group", "Negative keyword", "Match type"])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        wtr.write_record([
            &entry.campaign,
            entry.ad_group.as_deref().unwrap_or(""),
            &entry.keyword,
            &entry.match_type,
        ])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;
    Ok(())
}

pub fn write_budget_csv(path: &Path, entries: &[BudgetEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record(["Budget name", "Budget", "Budget status"])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        wtr.write_record([
            &entry.budget_name,
            &format!("{:.2}", entry.amount),
            &entry.status,
        ])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;
    Ok(())
}

pub fn write_sitelink_csv(path: &Path, entries: &[SitelinkEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record([
        "Campaign",
        "Ad group",
        "Sitelink text",
        "Sitelink final URL",
        "Sitelink description 1",
        "Sitelink description 2",
    ])
    .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        wtr.write_record([
            &entry.campaign,
            entry.ad_group.as_deref().unwrap_or(""),
            &entry.sitelink_text,
            &entry.final_url,
            entry.description1.as_deref().unwrap_or(""),
            entry.description2.as_deref().unwrap_or(""),
        ])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;
    Ok(())
}

pub fn write_callout_csv(path: &Path, entries: &[CalloutEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record(["Campaign", "Ad group", "Callout text"])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        wtr.write_record([
            &entry.campaign,
            entry.ad_group.as_deref().unwrap_or(""),
            &entry.callout_text,
        ])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;
    Ok(())
}

pub fn write_label_csv(path: &Path, entries: &[LabelEntry]) -> Result<()> {
    let mut wtr = csv::Writer::from_path(path)
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    wtr.write_record(["Label name", "Label description", "Label color"])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;

    for entry in entries {
        wtr.write_record([
            &entry.label_name,
            entry.description.as_deref().unwrap_or(""),
            entry.color.as_deref().unwrap_or(""),
        ])
        .map_err(|e| GadsError::Other(format!("CSV write error: {}", e)))?;
    }

    wtr.flush()
        .map_err(|e| GadsError::Other(format!("CSV flush error: {}", e)))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_write_keyword_csv() {
        let tmp = NamedTempFile::new().unwrap();
        let entries = vec![
            KeywordEntry {
                campaign: "Campaign 1".to_string(),
                ad_group: "Ad Group 1".to_string(),
                keyword: "test keyword".to_string(),
                match_type: "Broad".to_string(),
                max_cpc: Some(1.50),
                status: "Enabled".to_string(),
            },
            KeywordEntry {
                campaign: "Campaign 1".to_string(),
                ad_group: "Ad Group 1".to_string(),
                keyword: "[exact match]".to_string(),
                match_type: "Exact".to_string(),
                max_cpc: None,
                status: "Paused".to_string(),
            },
        ];

        write_keyword_csv(tmp.path(), &entries).unwrap();

        let content = std::fs::read_to_string(tmp.path()).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        assert_eq!(lines[0], "Campaign,Ad group,Keyword,Match type,Max CPC,Status");
        assert_eq!(lines[1], "Campaign 1,Ad Group 1,test keyword,Broad,1.50,Enabled");
        assert_eq!(lines[2], "Campaign 1,Ad Group 1,[exact match],Exact,,Paused");
    }

    #[test]
    fn test_write_campaign_csv() {
        let tmp = NamedTempFile::new().unwrap();
        let entries = vec![CampaignEntry {
            campaign: "My Campaign".to_string(),
            budget: 50.0,
            status: "Enabled".to_string(),
            bidding_strategy: "Manual CPC".to_string(),
        }];

        write_campaign_csv(tmp.path(), &entries).unwrap();

        let content = std::fs::read_to_string(tmp.path()).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        assert_eq!(lines[0], "Campaign,Budget,Campaign status,Bid strategy type");
        assert_eq!(lines[1], "My Campaign,50.00,Enabled,Manual CPC");
    }

    #[test]
    fn test_write_ad_csv() {
        let tmp = NamedTempFile::new().unwrap();
        let entries = vec![AdEntry {
            campaign: "Campaign 1".to_string(),
            ad_group: "Ad Group 1".to_string(),
            headlines: vec!["H1".to_string(), "H2".to_string(), "H3".to_string()],
            descriptions: vec!["D1".to_string(), "D2".to_string()],
            final_url: "https://example.com".to_string(),
            path1: Some("path1".to_string()),
            path2: Some("path2".to_string()),
            status: "Enabled".to_string(),
        }];

        write_ad_csv(tmp.path(), &entries).unwrap();

        let content = std::fs::read_to_string(tmp.path()).unwrap();
        let lines: Vec<&str> = content.lines().collect();

        assert_eq!(
            lines[0],
            "Campaign,Ad group,Headline 1,Headline 2,Headline 3,Description 1,Description 2,Final URL,Path 1,Path 2,Ad status"
        );
        assert_eq!(
            lines[1],
            "Campaign 1,Ad Group 1,H1,H2,H3,D1,D2,https://example.com,path1,path2,Enabled"
        );
    }

    #[test]
    fn test_write_keyword_csv_empty() {
        let tmp = NamedTempFile::new().unwrap();
        write_keyword_csv(tmp.path(), &[]).unwrap();

        let content = std::fs::read_to_string(tmp.path()).unwrap();
        let lines: Vec<&str> = content.lines().collect();
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0], "Campaign,Ad group,Keyword,Match type,Max CPC,Status");
    }
}
