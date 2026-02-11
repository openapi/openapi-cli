use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://ws.marchetemporali.com";
const SANDBOX: &str = "https://test.ws.marchetemporali.com";

#[derive(Debug, Subcommand)]
pub enum TimeStampingCommands {
    /// List purchased time stamp batches
    List,
    /// Check availability for a batch
    Availability {
        /// Stamp type
        #[arg(long)]
        stamp_type: String,

        /// Quantity
        #[arg(long)]
        qty: u32,
    },
    /// Get batch info
    Get {
        /// Stamp type
        #[arg(long)]
        stamp_type: String,

        /// Quantity
        #[arg(long)]
        qty: u32,
    },
}

pub async fn execute(cmd: &TimeStampingCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        TimeStampingCommands::List => {
            let resp = client.get(&format!("{}/marche", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TimeStampingCommands::Availability { stamp_type, qty } => {
            let resp = client
                .get(&format!("{}/availability/{}/{}", base, stamp_type, qty))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TimeStampingCommands::Get { stamp_type, qty } => {
            let resp = client
                .get(&format!("{}/marche/{}/{}", base, stamp_type, qty))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
