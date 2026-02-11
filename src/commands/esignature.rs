use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::ApiClient;

const PROD: &str = "https://esignature.openapi.com";
const SANDBOX: &str = "https://test.esignature.openapi.com";

#[derive(Debug, Subcommand)]
pub enum EsignatureCommands {
    /// List certificates
    Certificates,
    /// Get certificate details
    Certificate {
        /// Certificate ID
        #[arg(long)]
        id: String,
    },
    /// List signatures
    Signatures,
    /// Sign a document with EU-SES
    Ses {
        /// Base64-encoded document
        #[arg(long)]
        document: String,
    },
    /// Verify a signed document
    Verify {
        /// Base64-encoded signed document
        #[arg(long)]
        document: String,
    },
}

pub async fn execute(cmd: &EsignatureCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        EsignatureCommands::Certificates => {
            let resp = client.get(&format!("{}/certificates", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        EsignatureCommands::Certificate { id } => {
            let resp = client.get(&format!("{}/certificates/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        EsignatureCommands::Signatures => {
            let resp = client.get(&format!("{}/signatures", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        EsignatureCommands::Ses { document } => {
            let body = json!({ "document": document });
            let resp = client.post(&format!("{}/EU-SES", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        EsignatureCommands::Verify { document } => {
            let body = json!({ "document": document });
            let resp = client.post(&format!("{}/verify", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
