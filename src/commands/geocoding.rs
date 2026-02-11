use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::ApiClient;

const PROD: &str = "https://geocoding.openapi.it";
const SANDBOX: &str = "https://test.geocoding.openapi.it";

#[derive(Debug, Subcommand)]
pub enum GeocodingCommands {
    /// Geocode an address to coordinates
    Search {
        /// Address or place name
        #[arg(long)]
        query: String,
    },
    /// Reverse geocoding: get address from coordinates
    Reverse {
        /// Latitude
        #[arg(long)]
        lat: f64,

        /// Longitude
        #[arg(long)]
        lon: f64,
    },
}

pub async fn execute(cmd: &GeocodingCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        GeocodingCommands::Search { query } => {
            let body = json!({ "address": query });
            let resp = client.post(&format!("{}/geocode", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        GeocodingCommands::Reverse { lat, lon } => {
            let body = json!({ "lat": lat, "lng": lon });
            let resp = client.post(&format!("{}/reverse", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
