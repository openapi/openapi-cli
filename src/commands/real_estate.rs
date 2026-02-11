use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::ApiClient;

const PROD: &str = "https://realestate.openapi.com";
const SANDBOX: &str = "https://test.realestate.openapi.com";

#[derive(Debug, Subcommand)]
pub enum RealEstateCommands {
    /// Get RMV (Real Market Value) data
    Rmv {
        /// Address
        #[arg(long)]
        address: String,

        /// Municipality
        #[arg(long)]
        municipality: String,
    },
    /// Get square meter value (start)
    SqmValueStart {
        /// Address
        #[arg(long)]
        address: String,

        /// Municipality
        #[arg(long)]
        municipality: String,
    },
    /// Get square meter value (advanced)
    SqmValueAdvanced {
        /// Address
        #[arg(long)]
        address: String,

        /// Municipality
        #[arg(long)]
        municipality: String,
    },
}

pub async fn execute(cmd: &RealEstateCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        RealEstateCommands::Rmv {
            address,
            municipality,
        } => {
            let body = json!({ "address": address, "municipality": municipality });
            let resp = client.post(&format!("{}/IT-rmv", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        RealEstateCommands::SqmValueStart {
            address,
            municipality,
        } => {
            let body = json!({ "address": address, "municipality": municipality });
            let resp = client
                .post(&format!("{}/IT-sqm_value_start", base), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        RealEstateCommands::SqmValueAdvanced {
            address,
            municipality,
        } => {
            let body = json!({ "address": address, "municipality": municipality });
            let resp = client
                .post(&format!("{}/IT-sqm_value_advanced", base), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
