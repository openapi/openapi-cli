use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

use crate::config::{Config, OAuthConfig};

/// HTTP client for service APIs (Bearer auth).
pub struct ApiClient {
    http: reqwest::Client,
    pub sandbox: bool,
}

impl ApiClient {
    pub fn new(config: Config) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", config.token))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            sandbox: config.sandbox,
        })
    }

    /// Pick the right base URL depending on sandbox mode
    pub fn base_url<'a>(&self, production: &'a str, sandbox: &'a str) -> &'a str {
        if self.sandbox { sandbox } else { production }
    }

    pub async fn get(&self, url: &str) -> Result<Value> {
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }

    pub async fn post(&self, url: &str, body: &Value) -> Result<Value> {
        let resp = self.http.post(url).json(body).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }

}

/// HTTP client for the OAuth management API (Basic auth).
pub struct OAuthClient {
    http: reqwest::Client,
    pub sandbox: bool,
}

impl OAuthClient {
    pub fn new(config: OAuthConfig) -> Result<Self> {
        use base64::Engine;
        let credentials = base64::engine::general_purpose::STANDARD
            .encode(format!("{}:{}", config.username, config.key));

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", credentials))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            sandbox: config.sandbox,
        })
    }

    pub fn base_url(&self) -> &str {
        if self.sandbox {
            "https://test.oauth.openapi.it"
        } else {
            "https://oauth.openapi.it"
        }
    }

    pub async fn get(&self, url: &str) -> Result<Value> {
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }

    pub async fn post(&self, url: &str, body: &Value) -> Result<Value> {
        let resp = self.http.post(url).json(body).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }

    pub async fn delete(&self, url: &str) -> Result<Value> {
        let resp = self.http.delete(url).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }
}

/// HTTP client for the OAuth v2 management API (Basic auth).
pub struct OAuthV2Client {
    http: reqwest::Client,
    pub sandbox: bool,
}

impl OAuthV2Client {
    pub fn new(config: OAuthConfig) -> Result<Self> {
        use base64::Engine;
        let credentials = base64::engine::general_purpose::STANDARD
            .encode(format!("{}:{}", config.username, config.key));

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Basic {}", credentials))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .build()?;

        Ok(Self {
            http,
            sandbox: config.sandbox,
        })
    }

    pub fn base_url(&self) -> &str {
        if self.sandbox {
            "https://test.oauth.openapi.com"
        } else {
            "https://oauth.openapi.com"
        }
    }

    pub async fn get(&self, url: &str) -> Result<Value> {
        let resp = self.http.get(url).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }

    pub async fn get_query(&self, url: &str, query: &[(&str, String)]) -> Result<Value> {
        let resp = self.http.get(url).query(query).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }

    pub async fn post(&self, url: &str, body: &Value) -> Result<Value> {
        let resp = self.http.post(url).json(body).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }

    pub async fn patch(&self, url: &str, body: &Value) -> Result<Value> {
        let resp = self.http.patch(url).json(body).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }

    pub async fn delete(&self, url: &str) -> Result<Value> {
        let resp = self.http.delete(url).send().await?;
        let status = resp.status();
        let body: Value = resp.json().await?;
        if !status.is_success() {
            anyhow::bail!("API error ({}): {}", status, body);
        }
        Ok(body)
    }
}
