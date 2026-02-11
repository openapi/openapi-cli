use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://domains.altravia.com";
const SANDBOX: &str = "https://test.domains.altravia.com";

#[derive(Debug, Subcommand)]
pub enum DomainsCommands {
    /// Check .it domain availability
    Check {
        /// Domain name (e.g. "example.it")
        #[arg(long)]
        domain: String,
    },
    /// List managed domains
    List,
    /// Get domain info
    Get {
        /// Domain name
        #[arg(long)]
        domain: String,
    },
    /// List contacts
    Contacts,
}

pub async fn execute(cmd: &DomainsCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        DomainsCommands::Check { domain } => {
            let resp = client
                .get(&format!("{}/check/{}", base, domain))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        DomainsCommands::List => {
            let resp = client.get(&format!("{}/domain", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        DomainsCommands::Get { domain } => {
            let resp = client
                .get(&format!("{}/domain/{}", base, domain))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        DomainsCommands::Contacts => {
            let resp = client.get(&format!("{}/contact", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
