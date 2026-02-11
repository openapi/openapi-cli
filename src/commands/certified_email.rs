use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://pec.openapi.it";
const SANDBOX: &str = "https://test.pec.openapi.it";

#[derive(Debug, Subcommand)]
pub enum CertifiedEmailCommands {
    /// List PEC mailboxes
    List,
    /// Get PEC mailbox details
    Get {
        /// PEC ID
        #[arg(long)]
        id: String,
    },
    /// Verify a PEC address
    Verify {
        /// PEC address
        #[arg(long)]
        pec: String,
    },
    /// Check PEC domain availability
    CheckDomain {
        /// Domain name
        #[arg(long)]
        domain: String,
    },
    /// List comunica_pec requests
    ComunicaPec,
}

pub async fn execute(cmd: &CertifiedEmailCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        CertifiedEmailCommands::List => {
            let resp = client.get(&format!("{}/pec", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CertifiedEmailCommands::Get { id } => {
            let resp = client.get(&format!("{}/pec/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CertifiedEmailCommands::Verify { pec } => {
            let resp = client
                .get(&format!("{}/verifica_pec/{}", base, pec))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CertifiedEmailCommands::CheckDomain { domain } => {
            let resp = client
                .get(&format!("{}/domini_pec/{}", base, domain))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CertifiedEmailCommands::ComunicaPec => {
            let resp = client.get(&format!("{}/comunica_pec", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
