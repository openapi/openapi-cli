use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://sdi.openapi.it";
const SANDBOX: &str = "https://test.sdi.openapi.it";

#[derive(Debug, Subcommand)]
pub enum SdiCommands {
    /// List invoices
    Invoices,
    /// Get invoice by UUID
    Get {
        /// Invoice UUID
        #[arg(long)]
        uuid: String,
    },
    /// Download invoice
    Download {
        /// Invoice UUID
        #[arg(long)]
        uuid: String,
    },
    /// Get invoice notifications
    Notifications {
        /// Invoice UUID
        #[arg(long)]
        uuid: String,
    },
    /// List business registry configurations
    Configurations,
    /// List API configurations
    ApiConfigurations,
}

pub async fn execute(cmd: &SdiCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        SdiCommands::Invoices => {
            let resp = client.get(&format!("{}/invoices", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        SdiCommands::Get { uuid } => {
            let resp = client
                .get(&format!("{}/invoices/{}", base, uuid))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        SdiCommands::Download { uuid } => {
            let resp = client
                .get(&format!("{}/invoices_download/{}", base, uuid))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        SdiCommands::Notifications { uuid } => {
            let resp = client
                .get(&format!("{}/invoices_notifications/{}", base, uuid))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        SdiCommands::Configurations => {
            let resp = client
                .get(&format!("{}/business_registry_configurations", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        SdiCommands::ApiConfigurations => {
            let resp = client
                .get(&format!("{}/api_configurations", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
