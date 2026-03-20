#![allow(dead_code)]

use std::path::Path;

use crate::cli::Cli;
use crate::config::Config;
use crate::editor::binary;
use crate::editor::types::*;
use crate::error::Result;

use super::editor_cmd::{print_editor_output, resolve_customer_id};

pub fn handle_download(
    cli: &Cli,
    config: &Config,
    ecfg: Option<&crate::config::EditorConfig>,
    user_email: &str,
    campaign_names: &[String],
    campaign_remote_ids: &[String],
    download_type: Option<&str>,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    println!("Downloading account data for customer {}...", customer_id);
    let output = binary::download(
        customer_id,
        user_email,
        campaign_names,
        campaign_remote_ids,
        download_type,
        None,
        ecfg,
    )?;
    print_editor_output(&output);
    Ok(())
}

pub fn handle_import(
    cli: &Cli,
    config: &Config,
    ecfg: Option<&crate::config::EditorConfig>,
    file: &str,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let path = Path::new(file);
    println!("Importing CSV file: {}", path.display());
    let output = binary::import_csv(customer_id, path, None, ecfg)?;
    print_editor_output(&output);
    Ok(())
}

pub fn handle_post(
    cli: &Cli,
    config: &Config,
    ecfg: Option<&crate::config::EditorConfig>,
    user_email: &str,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    println!("Posting pending changes for customer {}...", customer_id);
    let output = binary::post(customer_id, user_email, None, ecfg)?;
    print_editor_output(&output);
    Ok(())
}

pub fn handle_validate(
    cli: &Cli,
    config: &Config,
    ecfg: Option<&crate::config::EditorConfig>,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    println!("Validating pending changes for customer {}...", customer_id);
    let output = binary::validate(customer_id, None, ecfg)?;
    print_editor_output(&output);
    Ok(())
}

pub fn handle_export_xml(
    cli: &Cli,
    config: &Config,
    ecfg: Option<&crate::config::EditorConfig>,
    output_path: &str,
    format: &str,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let xml_format = match format.to_lowercase().as_str() {
        "share" => XmlExportFormat::Share,
        "upgrade" => XmlExportFormat::Upgrade,
        _ => XmlExportFormat::Standard,
    };
    let path = Path::new(output_path);
    println!("Exporting XML to {}...", path.display());
    let result = binary::export_xml(customer_id, path, &xml_format, ecfg)?;
    print_editor_output(&result);
    Ok(())
}

pub fn handle_import_xml(
    cli: &Cli,
    config: &Config,
    ecfg: Option<&crate::config::EditorConfig>,
    file: &str,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let path = Path::new(file);
    println!("Importing XML from {}...", path.display());
    let output = binary::import_xml(customer_id, path, None, ecfg)?;
    print_editor_output(&output);
    Ok(())
}

pub fn handle_accept_proposals(
    cli: &Cli,
    config: &Config,
    ecfg: Option<&crate::config::EditorConfig>,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    println!("Accepting proposals for customer {}...", customer_id);
    let output = binary::accept_proposals(customer_id, None, ecfg)?;
    print_editor_output(&output);
    Ok(())
}

pub fn handle_export_html(
    cli: &Cli,
    config: &Config,
    ecfg: Option<&crate::config::EditorConfig>,
    output_path: &str,
) -> Result<()> {
    let customer_id = resolve_customer_id(cli, config)?;
    let path = Path::new(output_path);
    println!("Exporting HTML to {}...", path.display());
    let result = binary::export_html(customer_id, path, ecfg)?;
    print_editor_output(&result);
    Ok(())
}
