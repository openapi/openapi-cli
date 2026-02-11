use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://docuengine.openapi.com";
const SANDBOX: &str = "https://test.docuengine.openapi.com";

#[derive(Debug, Subcommand)]
pub enum DocuengineCommands {
    /// List available document types
    Documents,
    /// List requests
    Requests,
    /// Get request by ID
    Get {
        /// Request ID
        #[arg(long)]
        id: String,
    },
    /// Get request documents
    GetDocuments {
        /// Request ID
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(cmd: &DocuengineCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        DocuengineCommands::Documents => {
            let resp = client.get(&format!("{}/documents", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        DocuengineCommands::Requests => {
            let resp = client.get(&format!("{}/requests", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        DocuengineCommands::Get { id } => {
            let resp = client.get(&format!("{}/requests/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        DocuengineCommands::GetDocuments { id } => {
            let resp = client
                .get(&format!("{}/requests/{}/documents", base, id))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
