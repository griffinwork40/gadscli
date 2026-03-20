pub mod auth_cmd;
pub mod config_cmd;
pub mod account;
pub mod campaign;
pub mod ad_group;
pub mod ad;
pub mod keyword;
pub mod keyword_list;
pub mod keyword_mutate;
pub mod keyword_exclude;
pub mod keyword_ideas;
pub mod budget;
pub mod bidding;
pub mod report;
pub mod asset;
pub mod conversion;
pub mod label;
pub mod recommendation;
pub mod batch;
pub mod field;
pub mod negative_list;
pub mod device;
pub mod schedule;
pub mod location;
pub mod audience;
pub mod editor_cmd;
pub mod editor_read;
pub mod editor_read_ext;
pub mod editor_ops;
pub mod editor_write;

use crate::cli::{Cli, Commands};
use crate::client::GoogleAdsClient;
use crate::config::Config;
use crate::error::Result;

pub async fn dispatch(cli: &Cli, client: &GoogleAdsClient, config: &Config) -> Result<()> {
    match &cli.command {
        Commands::Auth { .. } | Commands::Config { .. } | Commands::Editor { .. } => unreachable!(),
        Commands::NegativeList { command } => negative_list::handle(command, client, cli).await,
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
        Commands::Device { command } => device::handle(command, client, cli).await,
        Commands::Schedule { command } => schedule::handle(command, client, cli).await,
        Commands::Location { command } => location::handle(command, client, cli).await,
        Commands::Audience { command } => audience::handle(command, client, cli).await,
    }
}
