use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::ApiClient;

const PROD: &str = "https://oauth.openapi.it";
const SANDBOX: &str = "https://test.oauth.openapi.it";

#[derive(Debug, Subcommand)]
pub enum TokenCommands {
    /// Create a new API token
    Create {
        /// Comma-separated list of scopes (e.g. "GET:test.imprese.openapi.it/advance")
        #[arg(long)]
        scopes: String,
    },
    /// List active tokens
    List,
    /// Revoke a token
    Revoke {
        /// Token to revoke
        #[arg(long)]
        token: String,
    },
    /// List available scopes
    Scopes,
    /// Get credit info
    Credit,
}

pub async fn execute(cmd: &TokenCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        TokenCommands::Create { scopes } => {
            let scopes_list: Vec<&str> = scopes.split(',').map(|s| s.trim()).collect();
            let body = json!({ "scopes": scopes_list });
            let resp = client.post(&format!("{}/token", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TokenCommands::List => {
            let resp = client.get(&format!("{}/token", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TokenCommands::Revoke { token } => {
            let resp = client.delete(&format!("{}/token/{}", base, token)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TokenCommands::Scopes => {
            let resp = client.get(&format!("{}/scopes", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TokenCommands::Credit => {
            let resp = client.get(&format!("{}/credit", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
