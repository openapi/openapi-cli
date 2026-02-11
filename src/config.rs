use anyhow::{bail, Result};

/// Config for service API commands (Bearer auth with OPENAPI_TOKEN).
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

/// Config for OAuth/token management commands (Basic auth with OPENAPI_USERNAME + OPENAPI_KEY).
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub username: String,
    pub key: String,
    pub sandbox: bool,
}

impl OAuthConfig {
    pub fn load(sandbox: bool) -> Result<Self> {
        let username = std::env::var("OPENAPI_USERNAME")
            .map_err(|_| anyhow::anyhow!("OPENAPI_USERNAME environment variable not set"))?;

        let key = if sandbox {
            std::env::var("OPENAPI_SANDBOX_KEY").map_err(|_| {
                anyhow::anyhow!("OPENAPI_SANDBOX_KEY environment variable not set")
            })?
        } else {
            std::env::var("OPENAPI_KEY")
                .map_err(|_| anyhow::anyhow!("OPENAPI_KEY environment variable not set"))?
        };

        if username.is_empty() {
            bail!("Username cannot be empty");
        }
        if key.is_empty() {
            bail!("Key cannot be empty");
        }

        Ok(Self {
            username,
            key,
            sandbox,
        })
    }
}
