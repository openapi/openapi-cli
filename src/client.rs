use anyhow::Result;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

use crate::config::Config;

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
            HeaderValue::from_str(&format!("Bearer {}", config.api_key))?,
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
    pub fn base_url(&self, production: &str, sandbox: &str) -> &str {
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

    pub async fn put(&self, url: &str, body: &Value) -> Result<Value> {
        let resp = self.http.put(url).json(body).send().await?;
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
