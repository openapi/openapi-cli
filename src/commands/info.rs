use anyhow::Result;

use crate::client::OAuthClient;
use crate::config::OAuthConfig;
use crate::scopes;

pub async fn execute() -> Result<()> {
    println!("openapi-cli v{}", env!("CARGO_PKG_VERSION"));
    println!();

    // OAuth credentials (Basic auth — for token management)
    let username = std::env::var("OPENAPI_USERNAME").ok();
    let key = std::env::var("OPENAPI_KEY").ok();
    let sandbox_key = std::env::var("OPENAPI_SANDBOX_KEY").ok();

    // Bearer tokens (for service API commands)
    let token = std::env::var("OPENAPI_TOKEN").ok();
    let sandbox_token = std::env::var("OPENAPI_SANDBOX_TOKEN").ok();

    let has_username = username.as_ref().is_some_and(|v| !v.is_empty());
    let has_key = key.as_ref().is_some_and(|v| !v.is_empty());
    let has_sandbox_key = sandbox_key.as_ref().is_some_and(|v| !v.is_empty());
    let has_token = token.as_ref().is_some_and(|v| !v.is_empty());
    let has_sandbox_token = sandbox_token.as_ref().is_some_and(|v| !v.is_empty());

    // Show variable status
    println!("Environment variables:");
    println!("  OAuth credentials (token management):");
    print_var_status("  OPENAPI_USERNAME", has_username, &username);
    print_var_status("  OPENAPI_KEY", has_key, &key);
    print_var_status("  OPENAPI_SANDBOX_KEY", has_sandbox_key, &sandbox_key);
    println!("  Bearer tokens (service API commands):");
    print_var_status("  OPENAPI_TOKEN", has_token, &token);
    print_var_status("  OPENAPI_SANDBOX_TOKEN", has_sandbox_token, &sandbox_token);
    println!();

    // Readiness for token management (Basic auth)
    let oauth_ready = has_username && has_key;
    let oauth_sandbox_ready = has_username && has_sandbox_key;

    if oauth_ready || oauth_sandbox_ready {
        print!("Token management: ");
        if oauth_ready && oauth_sandbox_ready {
            println!("READY (production + sandbox)");
        } else if oauth_ready {
            println!("READY (production only)");
        } else {
            println!("READY (sandbox only)");
        }
    } else {
        println!("Token management: NOT CONFIGURED");
        println!("  Set OPENAPI_USERNAME + OPENAPI_KEY to manage tokens.");
    }

    // Readiness for service commands (Bearer auth)
    if has_token || has_sandbox_token {
        print!("Service commands:  ");
        if has_token && has_sandbox_token {
            println!("READY (production + sandbox)");
        } else if has_token {
            println!("READY (production only)");
        } else {
            println!("READY (sandbox only)");
        }
    } else {
        println!("Service commands:  NOT CONFIGURED");
        println!("  Set OPENAPI_TOKEN to use service commands (sms, company, etc.).");
    }

    // Show token scopes using OAuth Basic auth
    if oauth_ready {
        if has_token {
            println!();
            println!("Production token scopes:");
            match fetch_token_scopes_via_oauth(false, token.as_ref().unwrap()).await {
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
    }

    if oauth_sandbox_ready {
        if has_sandbox_token {
            println!();
            println!("Sandbox token scopes:");
            match fetch_token_scopes_via_oauth(true, sandbox_token.as_ref().unwrap()).await {
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
    }

    Ok(())
}

/// Fetch scopes for a specific token using OAuth Basic auth.
async fn fetch_token_scopes_via_oauth(sandbox: bool, token: &str) -> Result<Vec<String>> {
    let oauth_config = OAuthConfig::load(sandbox)?;
    let oauth_client = OAuthClient::new(oauth_config)?;
    let base = oauth_client.base_url();
    let url = format!("{}/token/{}", base, token);

    let resp = oauth_client.get(&url).await?;

    // Response: { "data": [{ "scopes": [...], "token": "...", "expire": ... }], "success": true }
    let scopes = resp["data"]
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
