use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://invoice.openapi.com";
const SANDBOX: &str = "https://test.invoice.openapi.com";

#[derive(Debug, Subcommand)]
pub enum InvoiceCommands {
    /// List invoices
    List,
    /// Get invoice by ID
    Get {
        /// Invoice ID
        #[arg(long)]
        id: String,
    },
    /// List configurations
    Configurations,
    /// List receipts
    Receipts,
}

pub async fn execute(cmd: &InvoiceCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        InvoiceCommands::List => {
            let resp = client.get(&format!("{}/IT-invoices", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        InvoiceCommands::Get { id } => {
            let resp = client.get(&format!("{}/IT-invoices/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        InvoiceCommands::Configurations => {
            let resp = client.get(&format!("{}/IT-configurations", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        InvoiceCommands::Receipts => {
            let resp = client.get(&format!("{}/IT-receipts", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
