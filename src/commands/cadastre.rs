use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://catasto.openapi.it";
const SANDBOX: &str = "https://test.catasto.openapi.it";

#[derive(Debug, Subcommand)]
pub enum CadastreCommands {
    /// List territories (provinces)
    Territorio,
    /// Get territory details by province
    TerritorioDetails {
        /// Province code or name
        #[arg(long)]
        province: String,
    },
    /// List requests
    Requests,
    /// Get request by ID
    Get {
        /// Request ID
        #[arg(long)]
        id: String,
    },
    /// List cadastral surveys (visure catastali)
    VisuraCatastale,
    /// Get cadastral survey by ID
    GetVisuraCatastale {
        /// Survey ID
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(cmd: &CadastreCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        CadastreCommands::Territorio => {
            let resp = client.get(&format!("{}/territorio", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CadastreCommands::TerritorioDetails { province } => {
            let resp = client
                .get(&format!("{}/territorio/{}", base, province))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CadastreCommands::Requests => {
            let resp = client.get(&format!("{}/richiesta", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CadastreCommands::Get { id } => {
            let resp = client.get(&format!("{}/richiesta/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CadastreCommands::VisuraCatastale => {
            let resp = client
                .get(&format!("{}/visura_catastale", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        CadastreCommands::GetVisuraCatastale { id } => {
            let resp = client
                .get(&format!("{}/visura_catastale/{}", base, id))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
