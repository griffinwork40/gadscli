#![allow(dead_code)]

use std::path::{Path, PathBuf};
use std::process::Command;

use crate::config::EditorConfig;
use crate::error::{GadsError, Result};

use super::types::EditorOutput;

/// Glob pattern for finding Editor binary across versions
const EDITOR_APP_BASE: &str = "/Applications/Google Ads Editor.app/Contents/Versions";
const EDITOR_INNER_PATH: &str = "Google Ads Editor.app/Contents/MacOS/Google Ads Editor";

/// Detect the Editor binary path.
/// Priority: config override -> env var -> glob for highest installed version
pub fn editor_binary_path(config: Option<&EditorConfig>) -> Result<PathBuf> {
    // 1. Config override
    if let Some(cfg) = config {
        if let Some(ref path_str) = cfg.binary_path {
            let path = PathBuf::from(path_str);
            if path.exists() {
                return Ok(path);
            }
            return Err(GadsError::Other(format!(
                "Configured Editor binary not found at {}",
                path.display()
            )));
        }
    }

    // 2. Env var override
    if let Ok(path_str) = std::env::var("GADS_EDITOR_BINARY") {
        let path = PathBuf::from(&path_str);
        if path.exists() {
            return Ok(path);
        }
        return Err(GadsError::Other(format!(
            "GADS_EDITOR_BINARY path not found: {}",
            path_str
        )));
    }

    // 3. Auto-detect via glob
    auto_detect_binary()
}

/// Find the highest-version Editor binary installed
fn auto_detect_binary() -> Result<PathBuf> {
    let versions_dir = PathBuf::from(EDITOR_APP_BASE);
    if !versions_dir.exists() {
        return Err(GadsError::Other(
            "Google Ads Editor not found. Install it from https://ads.google.com/intl/en/home/tools/ads-editor/".to_string()
        ));
    }

    let mut candidates: Vec<PathBuf> = Vec::new();

    if let Ok(entries) = std::fs::read_dir(&versions_dir) {
        for entry in entries.flatten() {
            let binary = entry.path().join(EDITOR_INNER_PATH);
            if binary.exists() {
                candidates.push(binary);
            }
        }
    }

    if candidates.is_empty() {
        return Err(GadsError::Other(format!(
            "No Editor binary found in {}",
            versions_dir.display()
        )));
    }

    // Sort by version directory name (descending) to pick the highest version
    candidates.sort_by(|a, b| b.cmp(a));
    Ok(candidates.into_iter().next().unwrap())
}

/// Get the Editor version string from the detected binary path
pub fn editor_version(config: Option<&EditorConfig>) -> Result<String> {
    let path = editor_binary_path(config)?;
    // Extract version from path: .../Versions/14.12.4.0/...
    let path_str = path.to_string_lossy();
    if let Some(start) = path_str.find("/Versions/") {
        let after = &path_str[start + 10..];
        if let Some(end) = after.find('/') {
            return Ok(after[..end].to_string());
        }
    }
    Ok("unknown".to_string())
}

/// Common helper to build and run an Editor command
fn run_editor_command(
    config: Option<&EditorConfig>,
    customer_id: u64,
    user_email: Option<&str>,
    log_file: Option<&Path>,
    extra_args: &[&str],
) -> Result<EditorOutput> {
    let binary = editor_binary_path(config)?;

    let mut cmd = Command::new(&binary);
    cmd.env("QT_QPA_PLATFORM", "offscreen");

    // Add the operation-specific flags first
    for arg in extra_args {
        cmd.arg(arg);
    }

    cmd.arg("-customerId").arg(customer_id.to_string());

    // Add user email if provided (from parameter, then config)
    let email = user_email.or_else(|| {
        config.and_then(|c| c.user_email.as_deref())
    });
    if let Some(email) = email {
        cmd.arg("-userEmail").arg(email);
    }

    // Add log file if provided (from parameter, then config log_dir)
    if let Some(log) = log_file {
        cmd.arg("-logFile").arg(log);
    } else if let Some(cfg) = config {
        if let Some(ref log_dir) = cfg.log_dir {
            let log_path = PathBuf::from(log_dir).join("editor.log");
            cmd.arg("-logFile").arg(log_path);
        }
    }

    let output = cmd.output().map_err(|e| {
        GadsError::Other(format!("Failed to launch Editor: {}", e))
    })?;

    Ok(EditorOutput {
        exit_code: output.status.code(),
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        success: output.status.success(),
    })
}

pub fn download(
    customer_id: u64,
    user_email: &str,
    campaign_names: &[String],
    campaign_remote_ids: &[String],
    download_type: Option<&str>,
    log_file: Option<&Path>,
    config: Option<&EditorConfig>,
) -> Result<EditorOutput> {
    let mut args: Vec<String> = vec!["-download".to_string(), "-noics".to_string()];
    if !campaign_names.is_empty() {
        args.push("-campaignNames".to_string());
        args.extend(campaign_names.iter().cloned());
    }
    if !campaign_remote_ids.is_empty() {
        args.push("-campaignRemoteIds".to_string());
        args.extend(campaign_remote_ids.iter().cloned());
    }
    if let Some(dt) = download_type {
        args.push("-downloadType".to_string());
        args.push(dt.to_string());
    }
    let str_args: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    run_editor_command(config, customer_id, Some(user_email), log_file, &str_args)
}

pub fn import_csv(
    customer_id: u64,
    csv_path: &Path,
    log_file: Option<&Path>,
    config: Option<&EditorConfig>,
) -> Result<EditorOutput> {
    if !csv_path.exists() {
        return Err(GadsError::Other(format!(
            "CSV file not found: {}",
            csv_path.display()
        )));
    }
    let path_str = csv_path.to_string_lossy().to_string();
    run_editor_command(config, customer_id, None, log_file, &["-importCSV", "-importFile", &path_str])
}

pub fn post(
    customer_id: u64,
    user_email: &str,
    log_file: Option<&Path>,
    config: Option<&EditorConfig>,
) -> Result<EditorOutput> {
    run_editor_command(config, customer_id, Some(user_email), log_file, &["-post"])
}

pub fn export_html(
    customer_id: u64,
    output_path: &Path,
    config: Option<&EditorConfig>,
) -> Result<EditorOutput> {
    let path_str = output_path.to_string_lossy().to_string();
    run_editor_command(config, customer_id, None, None, &["-exportHTML", "-exportFile", &path_str])
}

pub fn validate(
    customer_id: u64,
    log_file: Option<&Path>,
    config: Option<&EditorConfig>,
) -> Result<EditorOutput> {
    run_editor_command(config, customer_id, None, log_file, &["-validate"])
}

pub fn export_xml(
    customer_id: u64,
    output_path: &Path,
    format: &super::types::XmlExportFormat,
    config: Option<&EditorConfig>,
) -> Result<EditorOutput> {
    let flag = match format {
        super::types::XmlExportFormat::Standard => "-exportXml",
        super::types::XmlExportFormat::Share => "-exportXmlShare",
        super::types::XmlExportFormat::Upgrade => "-exportXmlUpgrade",
    };
    let path_str = output_path.to_string_lossy().to_string();
    run_editor_command(config, customer_id, None, None, &[flag, "-exportFile", &path_str])
}

pub fn import_xml(
    customer_id: u64,
    xml_path: &Path,
    log_file: Option<&Path>,
    config: Option<&EditorConfig>,
) -> Result<EditorOutput> {
    if !xml_path.exists() {
        return Err(GadsError::Other(format!(
            "XML file not found: {}",
            xml_path.display()
        )));
    }
    let path_str = xml_path.to_string_lossy().to_string();
    run_editor_command(config, customer_id, None, log_file, &["-importXML", "-importFile", &path_str])
}

pub fn accept_proposals(
    customer_id: u64,
    log_file: Option<&Path>,
    config: Option<&EditorConfig>,
) -> Result<EditorOutput> {
    run_editor_command(config, customer_id, None, log_file, &["-forceAcceptChanges"])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auto_detect_binary_path_format() {
        // This test verifies the function doesn't panic; it may fail on CI without Editor
        let _ = auto_detect_binary();
    }

    #[test]
    fn test_editor_binary_path_with_config_override() {
        let config = EditorConfig {
            binary_path: Some("/nonexistent/path".to_string()),
            ..Default::default()
        };
        let result = editor_binary_path(Some(&config));
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("not found"));
    }

    #[test]
    fn test_editor_binary_path_none_config() {
        // With no config, falls through to auto-detect
        let _ = editor_binary_path(None);
    }

    #[test]
    fn test_editor_version_extraction() {
        // Test that version extraction doesn't panic
        let _ = editor_version(None);
    }
}
