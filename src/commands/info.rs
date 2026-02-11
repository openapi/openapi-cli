use anyhow::Result;
use reqwest::header::{HeaderValue, AUTHORIZATION, CONTENT_TYPE};
use serde_json::Value;

use crate::scopes;

pub async fn execute() -> Result<()> {
    println!("openapi-cli v{}", env!("CARGO_PKG_VERSION"));
    println!();

    let token = std::env::var("OPENAPI_TOKEN").ok();
    let sandbox_token = std::env::var("OPENAPI_SANDBOX_TOKEN").ok();

    let has_token = token.as_ref().is_some_and(|v| !v.is_empty());
    let has_sandbox_token = sandbox_token.as_ref().is_some_and(|v| !v.is_empty());

    // Show variable status
    println!("Environment variables:");
    print_var_status("OPENAPI_TOKEN", has_token, &token);
    print_var_status("OPENAPI_SANDBOX_TOKEN", has_sandbox_token, &sandbox_token);
    println!();

    // Readiness assessment
    if has_token && has_sandbox_token {
        println!("Status: READY");
        println!("  Production and sandbox environments are both available.");
    } else if has_token {
        println!("Status: READY (production only)");
        println!("  Production environment is available.");
        println!("  Set OPENAPI_SANDBOX_TOKEN to enable sandbox mode (-S).");
    } else if has_sandbox_token {
        println!("Status: SANDBOX ONLY");
        println!("  Only the sandbox environment is available (use -S flag).");
        println!("  Set OPENAPI_TOKEN to enable production mode.");
    } else {
        println!("Status: NOT CONFIGURED");
        println!("  The CLI cannot operate. Set the required environment variables:");
        println!("    export OPENAPI_TOKEN=\"your-api-token\"");
        println!("    export OPENAPI_SANDBOX_TOKEN=\"your-sandbox-token\"  (optional)");
    }

    // Show token scopes if tokens are available
    if has_token {
        println!();
        println!("Production token scopes:");
        match fetch_token_scopes(token.as_ref().unwrap(), false).await {
            Ok(token_scopes) if token_scopes.is_empty() => {
                println!("  (no scopes)");
            }
            Ok(token_scopes) => {
                print!("{}", scopes::group_scopes_by_service(&token_scopes, false));
            }
            Err(e) => {
                println!("  Unable to retrieve ({})", e);
            }
        }
    }

    if has_sandbox_token {
        println!();
        println!("Sandbox token scopes:");
        match fetch_token_scopes(sandbox_token.as_ref().unwrap(), true).await {
            Ok(token_scopes) if token_scopes.is_empty() => {
                println!("  (no scopes)");
            }
            Ok(token_scopes) => {
                print!("{}", scopes::group_scopes_by_service(&token_scopes, true));
            }
            Err(e) => {
                println!("  Unable to retrieve ({})", e);
            }
        }
    }

    Ok(())
}

async fn fetch_token_scopes(token: &str, sandbox: bool) -> Result<Vec<String>> {
    let base = if sandbox {
        "https://test.oauth.openapi.it"
    } else {
        "https://oauth.openapi.it"
    };
    let url = format!("{}/token/{}", base, token);

    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()?;

    let resp = client.get(&url).send().await?;
    let status = resp.status();
    let body: Value = resp.json().await?;

    if !status.is_success() {
        let msg = body["message"].as_str().unwrap_or("unknown error");
        anyhow::bail!("{}", msg);
    }

    // Response: { "data": [{ "scopes": [...], "token": "...", "expire": ... }], "success": true }
    let scopes = body["data"]
        .as_array()
        .and_then(|arr| arr.first())
        .and_then(|entry| entry["scopes"].as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    Ok(scopes)
}

fn print_var_status(name: &str, is_set: bool, value: &Option<String>) {
    if is_set {
        let v = value.as_ref().unwrap();
        let masked = if v.len() > 6 {
            format!("{}...{}", &v[..3], &v[v.len() - 3..])
        } else {
            "*".repeat(v.len())
        };
        println!("  {:<30} SET ({})", name, masked);
    } else {
        println!("  {:<30} NOT SET", name);
    }
}
