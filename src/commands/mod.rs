pub mod auth_cmd;
pub mod config_cmd;
pub mod account;
pub mod campaign;
pub mod ad_group;
pub mod ad;
pub mod keyword;
pub mod budget;
pub mod bidding;
pub mod report;
pub mod asset;
pub mod conversion;
pub mod label;
pub mod recommendation;
pub mod batch;
pub mod field;
pub mod editor_cmd;

use crate::cli::{Cli, Commands};
use crate::client::GoogleAdsClient;
use crate::config::Config;
use crate::error::Result;

pub async fn dispatch(cli: &Cli, client: &GoogleAdsClient, config: &Config) -> Result<()> {
    match &cli.command {
        Commands::Auth { .. } | Commands::Config { .. } | Commands::Editor { .. } => unreachable!(),
        Commands::Account { command } => account::handle(command, client, config).await,
        Commands::Campaign { command } => campaign::handle(command, client, cli).await,
        Commands::AdGroup { command } => ad_group::handle(command, client, cli).await,
        Commands::Ad { command } => ad::handle(command, client, cli).await,
        Commands::Keyword { command } => keyword::handle(command, client, cli).await,
        Commands::Budget { command } => budget::handle(command, client, cli).await,
        Commands::Bidding { command } => bidding::handle(command, client, cli).await,
        Commands::Report { command } => report::handle(command, client, cli).await,
        Commands::Asset { command } => asset::handle(command, client, cli).await,
        Commands::Conversion { command } => conversion::handle(command, client, cli).await,
        Commands::Label { command } => label::handle(command, client, cli).await,
        Commands::Recommendation { command } => recommendation::handle(command, client, cli).await,
        Commands::Batch { command } => batch::handle(command, client, cli).await,
        Commands::Field { command } => field::handle(command, client, cli).await,
    }
}
