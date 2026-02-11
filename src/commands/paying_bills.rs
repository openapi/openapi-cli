use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://ws.pagasubito.it";
const SANDBOX: &str = "https://test.ws.pagasubito.it";

#[derive(Debug, Subcommand)]
pub enum PayingBillsCommands {
    /// List payments
    List,
    /// Get payment by ID
    Get {
        /// Payment ID
        #[arg(long)]
        id: String,
    },
    /// Get payment receipt
    Receipt {
        /// Payment ID
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(cmd: &PayingBillsCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        PayingBillsCommands::List => {
            let resp = client.get(&format!("{}/pay/", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        PayingBillsCommands::Get { id } => {
            let resp = client.get(&format!("{}/pay/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        PayingBillsCommands::Receipt { id } => {
            let resp = client
                .get(&format!("{}/pay/{}/ricevuta", base, id))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
