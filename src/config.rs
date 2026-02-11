use anyhow::{bail, Result};

#[derive(Debug, Clone)]
pub struct Config {
    pub token: String,
    pub sandbox: bool,
}

impl Config {
    pub fn load(sandbox: bool) -> Result<Self> {
        let token = if sandbox {
            std::env::var("OPENAPI_SANDBOX_TOKEN").map_err(|_| {
                anyhow::anyhow!("OPENAPI_SANDBOX_TOKEN environment variable not set")
            })?
        } else {
            std::env::var("OPENAPI_TOKEN")
                .map_err(|_| anyhow::anyhow!("OPENAPI_TOKEN environment variable not set"))?
        };

        if token.is_empty() {
            bail!("Token cannot be empty");
        }

        Ok(Self { token, sandbox })
    }
}
