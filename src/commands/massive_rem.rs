use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::ApiClient;

const PROD: &str = "https://ws.pecmassiva.com";
const SANDBOX: &str = "https://test.ws.pecmassiva.com";

#[derive(Debug, Subcommand)]
pub enum MassiveRemCommands {
    /// Send a massive PEC
    Send {
        /// Mailbox to send from
        #[arg(long)]
        mailbox: String,

        /// Recipient address
        #[arg(long)]
        to: String,

        /// Subject
        #[arg(long)]
        subject: String,

        /// Body text
        #[arg(long)]
        body: String,
    },
    /// Get send status
    Get {
        /// Send code
        #[arg(long)]
        code: String,
    },
    /// List inbox messages
    Inbox,
    /// Get inbox message
    GetInbox {
        /// Message ID
        #[arg(long)]
        id: String,
    },
    /// Check mailbox quota
    Quota {
        /// Mailbox name
        #[arg(long)]
        mailbox: String,
    },
}

pub async fn execute(cmd: &MassiveRemCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        MassiveRemCommands::Send {
            mailbox,
            to,
            subject,
            body,
        } => {
            let payload = json!({
                "mailbox": mailbox,
                "to": to,
                "subject": subject,
                "body": body,
            });
            let resp = client.post(&format!("{}/send", base), &payload).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        MassiveRemCommands::Get { code } => {
            let resp = client.get(&format!("{}/send/{}", base, code)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        MassiveRemCommands::Inbox => {
            let resp = client.get(&format!("{}/inbox", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        MassiveRemCommands::GetInbox { id } => {
            let resp = client.get(&format!("{}/inbox/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        MassiveRemCommands::Quota { mailbox } => {
            let resp = client
                .get(&format!("{}/quota/{}", base, mailbox))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
