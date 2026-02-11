use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://company.openapi.com";
const SANDBOX: &str = "https://test.company.openapi.com";

#[derive(Debug, Subcommand)]
pub enum CompanyCommands {
    /// Search for Italian companies
    Search {
        /// Company name or tax code
        #[arg(long)]
        query: String,
    },
    /// Get basic company data (IT-start)
    Start {
        /// VAT code, tax code, or ID
        #[arg(long)]
        code: String,
    },
    /// Get advanced company data (IT-advanced)
    Advanced {
        /// VAT code, tax code, or ID
        #[arg(long)]
        code: String,
    },
    /// Get company PEC
    Pec {
        /// VAT code, tax code, or ID
        #[arg(long)]
        code: String,
    },
    /// Get company shareholders (IT-shareholders)
    Shareholders {
        /// VAT code, tax code, or ID
        #[arg(long)]
        code: String,
    },
    /// Get company address
    Address {
        /// VAT code, tax code, or ID
        #[arg(long)]
        code: String,
    },
    /// Get SDI code
    Sdicode {
        /// VAT code, tax code, or ID
        #[arg(long)]
        code: String,
    },
    /// List legal forms
    Legalforms,
}

pub async fn execute(cmd: &CompanyCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        CompanyCommands::Search { query } => {
            let resp = client
                .get(&format!("{}/IT-search?name={}", base, query))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CompanyCommands::Start { code } => {
            let resp = client
                .get(&format!("{}/IT-start/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CompanyCommands::Advanced { code } => {
            let resp = client
                .get(&format!("{}/IT-advanced/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CompanyCommands::Pec { code } => {
            let resp = client
                .get(&format!("{}/IT-pec/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CompanyCommands::Shareholders { code } => {
            let resp = client
                .get(&format!("{}/IT-shareholders/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CompanyCommands::Address { code } => {
            let resp = client
                .get(&format!("{}/IT-address/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CompanyCommands::Sdicode { code } => {
            let resp = client
                .get(&format!("{}/IT-sdicode/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CompanyCommands::Legalforms => {
            let resp = client
                .get(&format!("{}/IT-legalforms", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
