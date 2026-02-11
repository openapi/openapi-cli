use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://automotive.openapi.com";
const SANDBOX: &str = "https://test.automotive.openapi.com";

#[derive(Debug, Subcommand)]
pub enum AutomotiveCommands {
    /// Search Italian car by license plate
    Car {
        /// License plate number
        #[arg(long)]
        plate: String,
    },
    /// Search Italian bike by license plate
    Bike {
        /// License plate number
        #[arg(long)]
        plate: String,
    },
    /// Get insurance info for a vehicle
    Insurance {
        /// License plate number
        #[arg(long)]
        plate: String,
    },
    /// Check async request status
    CheckId {
        /// Request ID
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(cmd: &AutomotiveCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        AutomotiveCommands::Car { plate } => {
            let resp = client
                .get(&format!("{}/IT-car/{}", base, plate))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        AutomotiveCommands::Bike { plate } => {
            let resp = client
                .get(&format!("{}/IT-bike/{}", base, plate))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        AutomotiveCommands::Insurance { plate } => {
            let resp = client
                .get(&format!("{}/IT-insurance/{}", base, plate))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        AutomotiveCommands::CheckId { id } => {
            let resp = client
                .get(&format!("{}/check_id/{}", base, id))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
