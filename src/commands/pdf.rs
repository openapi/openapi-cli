use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::ApiClient;

const PROD: &str = "https://pdf.openapi.it";
const SANDBOX: &str = "https://test.pdf.openapi.it";

#[derive(Debug, Subcommand)]
pub enum PdfCommands {
    /// Convert HTML to PDF
    Convert {
        /// HTML content (inline string or file path with --from-file)
        #[arg(long)]
        html: String,

        /// Read HTML from file instead of inline
        #[arg(long)]
        from_file: bool,
    },
}

pub async fn execute(cmd: &PdfCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        PdfCommands::Convert { html, from_file } => {
            let html_content = if *from_file {
                std::fs::read_to_string(html)?
            } else {
                html.clone()
            };
            let body = json!({ "html": html_content });
            let resp = client.post(&format!("{}/base", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
