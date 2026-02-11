use anyhow::Result;
use clap::Subcommand;

use crate::client::ApiClient;

const PROD: &str = "https://risk.openapi.com";
const SANDBOX: &str = "https://test.risk.openapi.com";

#[derive(Debug, Subcommand)]
pub enum RiskCommands {
    /// Verify a fiscal code
    VerifyCf {
        /// Fiscal code
        #[arg(long)]
        fiscal_code: String,
    },
    /// Get credit score (start)
    CreditscoreStart {
        /// VAT code, tax code, or ID
        #[arg(long)]
        code: String,
    },
    /// Get credit score (advanced)
    CreditscoreAdvanced {
        /// VAT code, tax code, or ID
        #[arg(long)]
        code: String,
    },
    /// List person risk reports
    ReportPersona,
    /// Get person risk report by ID
    GetReportPersona {
        /// Report ID
        #[arg(long)]
        id: String,
    },
    /// List company risk reports
    ReportAzienda,
    /// Get company risk report by ID
    GetReportAzienda {
        /// Report ID
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(cmd: &RiskCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        RiskCommands::VerifyCf { fiscal_code } => {
            let resp = client
                .get(&format!("{}/IT-verifica_cf/{}", base, fiscal_code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        RiskCommands::CreditscoreStart { code } => {
            let resp = client
                .get(&format!("{}/IT-creditscore-start/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        RiskCommands::CreditscoreAdvanced { code } => {
            let resp = client
                .get(&format!("{}/IT-creditscore-advanced/{}", base, code))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        RiskCommands::ReportPersona => {
            let resp = client
                .get(&format!("{}/IT-report-persona", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        RiskCommands::GetReportPersona { id } => {
            let resp = client
                .get(&format!("{}/IT-report-persona/{}", base, id))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        RiskCommands::ReportAzienda => {
            let resp = client
                .get(&format!("{}/IT-report-azienda", base))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        RiskCommands::GetReportAzienda { id } => {
            let resp = client
                .get(&format!("{}/IT-report-azienda/{}", base, id))
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
