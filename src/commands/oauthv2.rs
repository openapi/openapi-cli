use anyhow::Result;
use clap::Subcommand;
use serde_json::{json, Map, Value};

use crate::client::OAuthV2Client;
use crate::scopes;

#[derive(Debug, Subcommand)]
pub enum Oauthv2Commands {
    /// Token lifecycle management
    Tokens {
        #[command(subcommand)]
        command: Oauthv2TokenCommands,
    },
    /// Available scopes
    Scopes {
        #[command(subcommand)]
        command: Oauthv2ScopeCommands,
    },
    /// Error logs
    Errors {
        #[command(subcommand)]
        command: Oauthv2ErrorCommands,
    },
    /// Wallet information
    Wallet {
        #[command(subcommand)]
        command: Oauthv2WalletCommands,
    },
    /// Usage statistics
    Stats {
        #[command(subcommand)]
        command: Oauthv2StatsCommands,
    },
    /// Active subscriptions
    Subscriptions {
        #[command(subcommand)]
        command: Oauthv2SubscriptionCommands,
    },
    /// Callback history
    Callbacks {
        #[command(subcommand)]
        command: Oauthv2CallbackCommands,
    },
}

#[derive(Debug, Subcommand)]
pub enum Oauthv2TokenCommands {
    /// Create a new token
    Create {
        /// Comma-separated list of scopes or aliases (e.g. "smsv2,company" or "post:smsv2")
        #[arg(long)]
        scopes: String,
        /// Human-readable token name
        #[arg(long)]
        name: Option<String>,
        /// Token lifetime in seconds
        #[arg(long)]
        ttl: Option<i64>,
        /// Maximum total requests allowed
        #[arg(long = "limit-total-requests")]
        limit_total_requests: Option<i64>,
        /// Maximum paid wallet requests allowed
        #[arg(long = "limit-wallet-requests")]
        limit_wallet_requests: Option<i64>,
        /// Maximum subscription requests allowed
        #[arg(long = "limit-subscription-requests")]
        limit_subscription_requests: Option<i64>,
        /// Maximum wallet amount usable
        #[arg(long = "limit-wallet-amount")]
        limit_wallet_amount: Option<f64>,
        /// Allowed IP address; repeat the flag to add multiple IPs
        #[arg(long = "allow-ip")]
        allow_ip: Vec<String>,
    },
    /// List tokens
    List {
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        skip: Option<u32>,
    },
    /// Get a token by ID
    Get {
        #[arg(long)]
        token: String,
    },
    /// Update a token by ID
    Update {
        #[arg(long)]
        token: String,
        /// Comma-separated list of scopes or aliases
        #[arg(long)]
        scopes: Option<String>,
        /// Human-readable token name
        #[arg(long)]
        name: Option<String>,
        /// Token lifetime in seconds
        #[arg(long)]
        ttl: Option<i64>,
        /// Maximum total requests allowed
        #[arg(long = "limit-total-requests")]
        limit_total_requests: Option<i64>,
        /// Maximum paid wallet requests allowed
        #[arg(long = "limit-wallet-requests")]
        limit_wallet_requests: Option<i64>,
        /// Maximum subscription requests allowed
        #[arg(long = "limit-subscription-requests")]
        limit_subscription_requests: Option<i64>,
        /// Maximum wallet amount usable
        #[arg(long = "limit-wallet-amount")]
        limit_wallet_amount: Option<f64>,
        /// Allowed IP address; repeat the flag to add multiple IPs
        #[arg(long = "allow-ip")]
        allow_ip: Vec<String>,
    },
    /// Delete a token by ID
    Delete {
        #[arg(long)]
        token: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum Oauthv2ScopeCommands {
    /// List available scopes
    List,
    /// Get scope detail by ID
    Get {
        #[arg(long)]
        id: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum Oauthv2ErrorCommands {
    /// List error logs
    List {
        #[arg(long)]
        scopes: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        skip: Option<u32>,
    },
}

#[derive(Debug, Subcommand)]
pub enum Oauthv2WalletCommands {
    /// Get wallet information
    Info,
    /// List wallet transactions
    Transactions {
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        skip: Option<u32>,
    },
}

#[derive(Debug, Subcommand)]
pub enum Oauthv2StatsCommands {
    /// Get overall API usage stats
    Overview {
        #[arg(long)]
        date: Option<String>,
    },
    /// Get stats grouped by API domain
    Apis {
        #[arg(long)]
        date: Option<String>,
    },
    /// Get detailed stats for a specific API domain
    Api {
        #[arg(long)]
        domain: String,
        #[arg(long)]
        date: Option<String>,
    },
    /// Get unique IP addresses
    Ips {
        #[arg(long)]
        date: Option<String>,
    },
}

#[derive(Debug, Subcommand)]
pub enum Oauthv2SubscriptionCommands {
    /// List subscriptions
    List {
        #[arg(long)]
        scopes: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        skip: Option<u32>,
    },
    /// Get subscription detail by ID
    Get {
        #[arg(long)]
        id: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum Oauthv2CallbackCommands {
    /// List callbacks
    List {
        #[arg(long)]
        scopes: Option<String>,
        #[arg(long)]
        limit: Option<u32>,
        #[arg(long)]
        skip: Option<u32>,
    },
    /// Get callback detail by ID
    Get {
        #[arg(long)]
        id: String,
    },
}

pub async fn execute(cmd: &Oauthv2Commands, client: &OAuthV2Client) -> Result<()> {
    let base = client.base_url();

    match cmd {
        Oauthv2Commands::Tokens { command } => match command {
            Oauthv2TokenCommands::Create {
                scopes: scopes_input,
                name,
                ttl,
                limit_total_requests,
                limit_wallet_requests,
                limit_subscription_requests,
                limit_wallet_amount,
                allow_ip,
            } => {
                let expanded = expand_scopes(client, scopes_input).await?;
                if expanded.is_empty() {
                    anyhow::bail!(
                        "No scopes matched. Use 'openapi oauthv2 scopes list' to inspect available scopes."
                    );
                }

                let mut body = Map::new();
                body.insert("scopes".to_string(), json!(expanded));
                if let Some(name) = name {
                    body.insert("name".to_string(), json!(name));
                }
                if let Some(ttl) = ttl {
                    body.insert("ttl".to_string(), json!(ttl));
                }
                if let Some(limits) = build_limits(
                    *limit_total_requests,
                    *limit_wallet_requests,
                    *limit_subscription_requests,
                    *limit_wallet_amount,
                    allow_ip,
                ) {
                    body.insert("limits".to_string(), limits);
                }

                let resp = client.post(&format!("{}/tokens", base), &Value::Object(body)).await?;
                print_json(&resp)?;
            }
            Oauthv2TokenCommands::List { limit, skip } => {
                let params = pagination_params(*limit, *skip);
                let resp = client
                    .get_query(&format!("{}/tokens", base), &params)
                    .await?;
                print_json(&resp)?;
            }
            Oauthv2TokenCommands::Get { token } => {
                let resp = client.get(&format!("{}/tokens/{}", base, token)).await?;
                print_json(&resp)?;
            }
            Oauthv2TokenCommands::Update {
                token,
                scopes,
                name,
                ttl,
                limit_total_requests,
                limit_wallet_requests,
                limit_subscription_requests,
                limit_wallet_amount,
                allow_ip,
            } => {
                let mut body = Map::new();
                if let Some(scopes_input) = scopes {
                    let expanded = expand_scopes(client, scopes_input).await?;
                    if expanded.is_empty() {
                        anyhow::bail!(
                            "No scopes matched. Use 'openapi oauthv2 scopes list' to inspect available scopes."
                        );
                    }
                    body.insert("scopes".to_string(), json!(expanded));
                }
                if let Some(name) = name {
                    body.insert("name".to_string(), json!(name));
                }
                if let Some(ttl) = ttl {
                    body.insert("ttl".to_string(), json!(ttl));
                }
                if let Some(limits) = build_limits(
                    *limit_total_requests,
                    *limit_wallet_requests,
                    *limit_subscription_requests,
                    *limit_wallet_amount,
                    allow_ip,
                ) {
                    body.insert("limits".to_string(), limits);
                }

                if body.is_empty() {
                    anyhow::bail!("Nothing to update. Provide at least one mutable field.");
                }

                let resp = client
                    .patch(&format!("{}/tokens/{}", base, token), &Value::Object(body))
                    .await?;
                print_json(&resp)?;
            }
            Oauthv2TokenCommands::Delete { token } => {
                let resp = client.delete(&format!("{}/tokens/{}", base, token)).await?;
                print_json(&resp)?;
            }
        },
        Oauthv2Commands::Scopes { command } => match command {
            Oauthv2ScopeCommands::List => {
                let resp = client.get(&format!("{}/scopes", base)).await?;
                print_json(&resp)?;
            }
            Oauthv2ScopeCommands::Get { id } => {
                let resp = client.get(&format!("{}/scopes/{}", base, id)).await?;
                print_json(&resp)?;
            }
        },
        Oauthv2Commands::Errors { command } => match command {
            Oauthv2ErrorCommands::List { scopes, limit, skip } => {
                let mut params = pagination_params(*limit, *skip);
                push_optional(&mut params, "scopes", scopes.clone());
                let resp = client
                    .get_query(&format!("{}/errors", base), &params)
                    .await?;
                print_json(&resp)?;
            }
        },
        Oauthv2Commands::Wallet { command } => match command {
            Oauthv2WalletCommands::Info => {
                let resp = client.get(&format!("{}/wallet", base)).await?;
                print_json(&resp)?;
            }
            Oauthv2WalletCommands::Transactions { limit, skip } => {
                let params = pagination_params(*limit, *skip);
                let resp = client
                    .get_query(&format!("{}/wallet/transactions", base), &params)
                    .await?;
                print_json(&resp)?;
            }
        },
        Oauthv2Commands::Stats { command } => match command {
            Oauthv2StatsCommands::Overview { date } => {
                let mut params = Vec::new();
                push_optional(&mut params, "date", date.clone());
                let resp = client.get_query(&format!("{}/stats", base), &params).await?;
                print_json(&resp)?;
            }
            Oauthv2StatsCommands::Apis { date } => {
                let mut params = Vec::new();
                push_optional(&mut params, "date", date.clone());
                let resp = client
                    .get_query(&format!("{}/stats/apis", base), &params)
                    .await?;
                print_json(&resp)?;
            }
            Oauthv2StatsCommands::Api { domain, date } => {
                let mut params = Vec::new();
                push_optional(&mut params, "date", date.clone());
                let resp = client
                    .get_query(&format!("{}/stats/apis/{}", base, domain), &params)
                    .await?;
                print_json(&resp)?;
            }
            Oauthv2StatsCommands::Ips { date } => {
                let mut params = Vec::new();
                push_optional(&mut params, "date", date.clone());
                let resp = client
                    .get_query(&format!("{}/stats/ips", base), &params)
                    .await?;
                print_json(&resp)?;
            }
        },
        Oauthv2Commands::Subscriptions { command } => match command {
            Oauthv2SubscriptionCommands::List { scopes, limit, skip } => {
                let mut params = pagination_params(*limit, *skip);
                push_optional(&mut params, "scopes", scopes.clone());
                let resp = client
                    .get_query(&format!("{}/subscriptions", base), &params)
                    .await?;
                print_json(&resp)?;
            }
            Oauthv2SubscriptionCommands::Get { id } => {
                let resp = client
                    .get(&format!("{}/subscriptions/{}", base, id))
                    .await?;
                print_json(&resp)?;
            }
        },
        Oauthv2Commands::Callbacks { command } => match command {
            Oauthv2CallbackCommands::List { scopes, limit, skip } => {
                let mut params = pagination_params(*limit, *skip);
                push_optional(&mut params, "scopes", scopes.clone());
                let resp = client
                    .get_query(&format!("{}/callbacks", base), &params)
                    .await?;
                print_json(&resp)?;
            }
            Oauthv2CallbackCommands::Get { id } => {
                let resp = client.get(&format!("{}/callbacks/{}", base, id)).await?;
                print_json(&resp)?;
            }
        },
    }

    Ok(())
}

async fn expand_scopes(client: &OAuthV2Client, scopes_input: &str) -> Result<Vec<String>> {
    let all_scopes = fetch_all_scopes(client).await?;
    Ok(scopes::expand_aliases(
        scopes_input,
        &all_scopes,
        client.sandbox,
    ))
}

async fn fetch_all_scopes(client: &OAuthV2Client) -> Result<Vec<String>> {
    let resp = client.get(&format!("{}/scopes", client.base_url())).await?;
    let scopes = resp["data"]
        .as_array()
        .ok_or_else(|| anyhow::anyhow!("Unexpected response format from /scopes endpoint"))?;

    Ok(scopes
        .iter()
        .filter_map(|item| item["scope"].as_str().map(String::from))
        .collect())
}

fn build_limits(
    total_requests: Option<i64>,
    wallet_requests: Option<i64>,
    subscription_requests: Option<i64>,
    wallet_amount: Option<f64>,
    allow_ip: &[String],
) -> Option<Value> {
    let mut limits = Map::new();
    if let Some(value) = total_requests {
        limits.insert("totalRequests".to_string(), json!(value));
    }
    if let Some(value) = wallet_requests {
        limits.insert("walletRequests".to_string(), json!(value));
    }
    if let Some(value) = subscription_requests {
        limits.insert("subscriptionRequests".to_string(), json!(value));
    }
    if let Some(value) = wallet_amount {
        limits.insert("walletAmount".to_string(), json!(value));
    }
    if !allow_ip.is_empty() {
        limits.insert("ip".to_string(), json!(allow_ip));
    }

    if limits.is_empty() {
        None
    } else {
        Some(Value::Object(limits))
    }
}

fn pagination_params(limit: Option<u32>, skip: Option<u32>) -> Vec<(&'static str, String)> {
    let mut params = Vec::new();
    push_optional(&mut params, "limit", limit.map(|v| v.to_string()));
    push_optional(&mut params, "skip", skip.map(|v| v.to_string()));
    params
}

fn push_optional(
    params: &mut Vec<(&'static str, String)>,
    key: &'static str,
    value: Option<String>,
) {
    if let Some(value) = value {
        params.push((key, value));
    }
}

fn print_json(value: &Value) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(value)?);
    Ok(())
}
