use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://exchange.altravia.com";
const SANDBOX: &str = "https://test.exchange.altravia.com";

#[derive(Debug, Subcommand)]
pub enum ExchangeRateCommands {
    /// Get exchange rates
    Get,
}

pub async fn execute(cmd: &ExchangeRateCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        ExchangeRateCommands::Get => {
            let resp = client.get(base).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
