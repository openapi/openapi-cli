use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://ws.ufficiopostale.com";
const SANDBOX: &str = "https://test.ws.ufficiopostale.com";

#[derive(Debug, Subcommand)]
pub enum PostalServiceCommands {
    /// List pricing info
    Pricing,
    /// List registered mail (raccomandate)
    Raccomandate,
    /// List ordinary mail (ordinarie)
    Ordinarie,
    /// List priority mail (prioritarie)
    Prioritarie,
    /// List telegrams
    Telegrammi,
    /// Get tracking info
    Tracking {
        /// Shipment ID
        #[arg(long)]
        id: String,
    },
    /// List available nations for a service
    Nazioni {
        /// Service type
        #[arg(long)]
        service: Option<String>,
    },
}

pub async fn execute(cmd: &PostalServiceCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        PostalServiceCommands::Pricing => {
            let resp = client.get(&format!("{}/pricing/", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        PostalServiceCommands::Raccomandate => {
            let resp = client.get(&format!("{}/raccomandate/", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        PostalServiceCommands::Ordinarie => {
            let resp = client.get(&format!("{}/ordinarie/", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        PostalServiceCommands::Prioritarie => {
            let resp = client.get(&format!("{}/prioritarie/", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        PostalServiceCommands::Telegrammi => {
            let resp = client.get(&format!("{}/telegrammi/", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        PostalServiceCommands::Tracking { id } => {
            let resp = client.get(&format!("{}/tracking/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        PostalServiceCommands::Nazioni { service } => {
            let path = match service {
                Some(s) => format!("{}/nazioni/{}", base, s),
                None => format!("{}/nazioni/", base),
            };
            let resp = client.get(&path).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
