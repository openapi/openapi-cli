use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://trust.openapi.com";
const SANDBOX: &str = "https://test.trust.openapi.com";

#[derive(Debug, Subcommand)]
pub enum TrustCommands {
    /// Verify a mobile phone (start)
    MobileStart {
        /// Phone number
        #[arg(long)]
        phone: String,
    },
    /// Verify an email (start)
    EmailStart {
        /// Email address
        #[arg(long)]
        email: String,
    },
    /// Verify a mobile phone (advanced)
    MobileAdvanced {
        /// Phone number
        #[arg(long)]
        phone: String,
    },
    /// Verify an email (advanced)
    EmailAdvanced {
        /// Email address
        #[arg(long)]
        email: String,
    },
    /// Verify an IP address (advanced)
    IpAdvanced {
        /// IP address
        #[arg(long)]
        ip: String,
    },
    /// Verify a URL (advanced)
    UrlAdvanced {
        /// URL to verify
        #[arg(long)]
        url: String,
    },
}

pub async fn execute(cmd: &TrustCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        TrustCommands::MobileStart { phone } => {
            let resp = client
                .post(&format!("{}/mobile-start/{}", base, phone), &serde_json::json!({}))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TrustCommands::EmailStart { email } => {
            let resp = client
                .post(&format!("{}/email-start/{}", base, email), &serde_json::json!({}))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TrustCommands::MobileAdvanced { phone } => {
            let resp = client
                .post(&format!("{}/mobile-advanced/{}", base, phone), &serde_json::json!({}))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TrustCommands::EmailAdvanced { email } => {
            let resp = client
                .post(&format!("{}/email-advanced/{}", base, email), &serde_json::json!({}))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TrustCommands::IpAdvanced { ip } => {
            let resp = client
                .post(&format!("{}/ip-advanced/{}", base, ip), &serde_json::json!({}))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        TrustCommands::UrlAdvanced { url } => {
            let resp = client
                .post(&format!("{}/url-advanced/{}", base, url), &serde_json::json!({}))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
