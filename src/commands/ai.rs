use anyhow::Result;
use clap::Subcommand;
use serde_json::json;

use crate::client::ApiClient;

const PROD: &str = "https://ai.openapi.com";
const SANDBOX: &str = "https://test.ai.openapi.com";

#[derive(Debug, Subcommand)]
pub enum AiCommands {
    /// List RAG collections
    List,
    /// Get RAG collection details
    Get {
        /// RAG collection ID
        #[arg(long)]
        id: String,
    },
    /// Search within a RAG collection
    Search {
        /// RAG collection ID
        #[arg(long)]
        id: String,

        /// Search query
        #[arg(long)]
        query: String,
    },
    /// Search with AI-generated answer
    SearchWithAnswer {
        /// RAG collection ID
        #[arg(long)]
        id: String,

        /// Search query
        #[arg(long)]
        query: String,
    },
}

pub async fn execute(cmd: &AiCommands, client: &ApiClient) -> Result<()> {
    let base = client.base_url(PROD, SANDBOX);
    match cmd {
        AiCommands::List => {
            let resp = client.get(&format!("{}/rag", base)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        AiCommands::Get { id } => {
            let resp = client.get(&format!("{}/rag/{}", base, id)).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        AiCommands::Search { id, query } => {
            let body = json!({ "rag_id": id, "query": query });
            let resp = client.post(&format!("{}/rag-search", base), &body).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
        AiCommands::SearchWithAnswer { id, query } => {
            let body = json!({ "rag_id": id, "query": query });
            let resp = client
                .post(&format!("{}/rag-search-with-answer", base), &body)
                .await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    }
    Ok(())
}
