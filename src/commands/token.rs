use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::OAuthClient;
use crate::scopes;

#[derive(Debug, Subcommand)]
pub enum TokenCommands {
    /// Create a new API token
    Create {
        /// Comma-separated list of scopes or aliases (e.g. "sms,company" or "post:sms")
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

pub async fn execute(cmd: &TokenCommands, client: &OAuthClient) -> Result<()> {
    let base = client.base_url();
    match cmd {
        TokenCommands::Create { scopes: scopes_input } => {
            // Fetch all available scopes to expand aliases
            let all_scopes = fetch_all_scopes(client).await?;
            let expanded = scopes::expand_aliases(scopes_input, &all_scopes, client.sandbox);

            if expanded.is_empty() {
                anyhow::bail!(
                    "No scopes matched. Use 'openapi token scopes' to list available scopes, \
                     or check USAGE.md for alias documentation."
                );
            }

            println!("Creating token with {} scope(s):", expanded.len());
            for s in &expanded {
                println!("  {}", s);
            }
            println!();

            let body = json!({ "scopes": expanded });
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

/// Fetch all available scopes from the OAuth API.
async fn fetch_all_scopes(client: &OAuthClient) -> Result<Vec<String>> {
    let base = client.base_url();
    let resp = client.get(&format!("{}/scopes", base)).await?;

    // The response can be { "data": ["scope1", ...] } or { "data": { "data": ["scope1", ...] } }
    let scopes_array = resp["data"]
        .as_array()
        .or_else(|| resp["data"]["data"].as_array());

    match scopes_array {
        Some(arr) => Ok(arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect()),
        None => anyhow::bail!("Unexpected response format from /scopes endpoint"),
    }
}
