use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub username: String,
    pub api_key: String,
    pub sandbox: bool,
}

impl Config {
    pub fn load(sandbox: bool) -> Result<Self> {
        let username = std::env::var("OPENAPI_USERNAME")
            .map_err(|_| anyhow::anyhow!("OPENAPI_USERNAME environment variable not set"))?;

        let api_key = if sandbox {
            std::env::var("OPENAPI_SANDBOX_KEY").map_err(|_| {
                anyhow::anyhow!("OPENAPI_SANDBOX_KEY environment variable not set")
            })?
        } else {
            std::env::var("OPENAPI_KEY")
                .map_err(|_| anyhow::anyhow!("OPENAPI_KEY environment variable not set"))?
        };

        if username.is_empty() {
            bail!("OPENAPI_USERNAME cannot be empty");
        }
        if api_key.is_empty() {
            bail!("API key cannot be empty");
        }

        Ok(Self {
            username,
            api_key,
            sandbox,
        })
    }
}
