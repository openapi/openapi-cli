use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::ApiClient;

const PROD: &str = "https://sms.openapi.com";
const SANDBOX: &str = "https://test.sms.openapi.com";

#[derive(Debug, Subcommand)]
pub enum SmsCommands {
    /// Send an SMS (IT)
    Send {
        /// Recipient phone number (e.g. "+391234567890")
        #[arg(long)]
        to: String,

        /// Message text
        #[arg(long)]
        message: String,

        /// Custom sender name
        #[arg(long)]
        sender: Option<String>,
    },
    /// List sent messages
    List,
    /// Get message details
    Get {
        /// Message ID
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(cmd: &SmsCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        SmsCommands::Send {
            to,
            message,
            sender,
        } => {
            let mut body = json!({
                "recipient": to,
                "message": message,
            });
            if let Some(s) = sender {
                body["sender"] = json!(s);
            }
            let resp = client.post(&format!("{}/IT-messages", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        SmsCommands::List => {
            let resp = client.get(&format!("{}/messages", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        SmsCommands::Get { id } => {
            let resp = client.get(&format!("{}/messages/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
