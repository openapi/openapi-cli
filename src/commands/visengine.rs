use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://visengine2.altravia.com";
const SANDBOX: &str = "https://test.visengine2.altravia.com";

#[derive(Debug, Subcommand)]
pub enum VisengineCommands {
    /// List available document types (visure)
    Visure,
    /// Get document type details
    VisuraDetails {
        /// Hash visura
        #[arg(long)]
        hash: String,
    },
    /// List requests
    Requests,
    /// Get request by ID
    Get {
        /// Request ID
        #[arg(long)]
        id: String,
    },
    /// Get document by ID
    Document {
        /// Document ID
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(cmd: &VisengineCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        VisengineCommands::Visure => {
            let resp = client.get(&format!("{}/visure", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        VisengineCommands::VisuraDetails { hash } => {
            let resp = client
                .get(&format!("{}/visure/{}", base, hash))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        VisengineCommands::Requests => {
            let resp = client.get(&format!("{}/richiesta", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        VisengineCommands::Get { id } => {
            let resp = client.get(&format!("{}/richiesta/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        VisengineCommands::Document { id } => {
            let resp = client
                .get(&format!("{}/documento/{}", base, id))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
