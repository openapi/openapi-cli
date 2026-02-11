mod cli;
mod client;
mod commands;
mod config;
mod scopes;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Commands};
use client::{ApiClient, OAuthClient};
use config::{Config, OAuthConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if let Commands::Info = &cli.command {
        return commands::info::execute().await;
    }

    // Token management uses Basic auth (OPENAPI_USERNAME + OPENAPI_KEY)
    if let Commands::Token { command } = &cli.command {
        let oauth_config = OAuthConfig::load(cli.sandbox)?;
        let oauth_client = OAuthClient::new(oauth_config)?;
        return commands::token::execute(command, &oauth_client).await;
    }

    // All other commands use Bearer auth (OPENAPI_TOKEN)
    let config = Config::load(cli.sandbox)?;
    let client = ApiClient::new(config)?;

    match &cli.command {
        Commands::Company { command } => commands::company::execute(command, &client).await,
        Commands::Sms { command } => commands::sms::execute(command, &client).await,
        Commands::Esignature { command } => commands::esignature::execute(command, &client).await,
        Commands::Risk { command } => commands::risk::execute(command, &client).await,
        Commands::Ai { command } => commands::ai::execute(command, &client).await,
        Commands::Trust { command } => commands::trust::execute(command, &client).await,
        Commands::Geocoding { command } => commands::geocoding::execute(command, &client).await,
        Commands::Invoice { command } => commands::invoice::execute(command, &client).await,
        Commands::Automotive { command } => commands::automotive::execute(command, &client).await,
        Commands::Docuengine { command } => commands::docuengine::execute(command, &client).await,
        Commands::ChamberOfCommerce { command } => {
            commands::chamber_of_commerce::execute(command, &client).await
        }
        Commands::RealEstate { command } => commands::real_estate::execute(command, &client).await,
        Commands::ZipCodes { command } => commands::zip_codes::execute(command, &client).await,
        Commands::Visengine { command } => commands::visengine::execute(command, &client).await,
        Commands::Cadastre { command } => commands::cadastre::execute(command, &client).await,
        Commands::PostalService { command } => {
            commands::postal_service::execute(command, &client).await
        }
        Commands::MassiveRem { command } => commands::massive_rem::execute(command, &client).await,
        Commands::Pdf { command } => commands::pdf::execute(command, &client).await,
        Commands::TimeStamping { command } => {
            commands::time_stamping::execute(command, &client).await
        }
        Commands::CertifiedEmail { command } => {
            commands::certified_email::execute(command, &client).await
        }
        Commands::PayingBills { command } => {
            commands::paying_bills::execute(command, &client).await
        }
        Commands::ExchangeRate { command } => {
            commands::exchange_rate::execute(command, &client).await
        }
        Commands::Domains { command } => commands::domains::execute(command, &client).await,
        Commands::Sdi { command } => commands::sdi::execute(command, &client).await,
        Commands::Info | Commands::Token { .. } => unreachable!(),
    }
}
