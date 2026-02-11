use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://visurecamerali.openapi.it";
const SANDBOX: &str = "https://test.visurecamerali.openapi.it";

#[derive(Debug, Subcommand)]
pub enum ChamberOfCommerceCommands {
    /// Search company
    Search {
        /// Tax code or VAT number
        #[arg(long)]
        code: String,
    },
    /// Get company details
    Get {
        /// Tax code, VAT number, or ID
        #[arg(long)]
        code: String,
    },
    /// List ordinary requests for individual companies
    OrdinariaImpresaIndividuale,
    /// List ordinary requests for capital companies
    OrdinariaSocietaCapitale,
    /// List active shareholders requests
    SociAttivi,
}

pub async fn execute(cmd: &ChamberOfCommerceCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        ChamberOfCommerceCommands::Search { code } => {
            let resp = client
                .get(&format!("{}/impresa?cf_piva={}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ChamberOfCommerceCommands::Get { code } => {
            let resp = client
                .get(&format!("{}/impresa/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ChamberOfCommerceCommands::OrdinariaImpresaIndividuale => {
            let resp = client
                .get(&format!("{}/ordinaria-impresa-individuale", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ChamberOfCommerceCommands::OrdinariaSocietaCapitale => {
            let resp = client
                .get(&format!("{}/ordinaria-societa-capitale", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ChamberOfCommerceCommands::SociAttivi => {
            let resp = client
                .get(&format!("{}/soci-attivi", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
