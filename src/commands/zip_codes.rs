use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://cap.openapi.it";
const SANDBOX: &str = "https://test.cap.openapi.it";

#[derive(Debug, Subcommand)]
pub enum ZipCodesCommands {
    /// Search municipalities
    SearchMunicipalities {
        /// Search query
        #[arg(long)]
        query: String,
    },
    /// Get municipality info by ISTAT code (basic)
    MunicipalityBase {
        /// ISTAT code
        #[arg(long)]
        istat: String,
    },
    /// Search by zip code (CAP)
    Cap {
        /// Zip code
        #[arg(long)]
        cap: String,
    },
    /// List all regions
    Regions,
    /// List all provinces
    Provinces,
    /// Get province by code
    Province {
        /// Province code (e.g. "RM")
        #[arg(long)]
        code: String,
    },
}

pub async fn execute(cmd: &ZipCodesCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        ZipCodesCommands::SearchMunicipalities { query } => {
            let resp = client
                .get(&format!("{}/cerca_comuni?nome={}", base, query))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ZipCodesCommands::MunicipalityBase { istat } => {
            let resp = client
                .get(&format!("{}/comuni_base/{}", base, istat))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ZipCodesCommands::Cap { cap } => {
            let resp = client.get(&format!("{}/cap/{}", base, cap)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ZipCodesCommands::Regions => {
            let resp = client.get(&format!("{}/regioni", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ZipCodesCommands::Provinces => {
            let resp = client.get(&format!("{}/province", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        ZipCodesCommands::Province { code } => {
            let resp = client
                .get(&format!("{}/province/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
