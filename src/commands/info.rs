use anyhow::Result;

pub fn execute() -> Result<()> {
    println!("openapi-cli v{}", env!("CARGO_PKG_VERSION"));
    println!();

    let username = std::env::var("OPENAPI_USERNAME").ok();
    let key = std::env::var("OPENAPI_KEY").ok();
    let sandbox_key = std::env::var("OPENAPI_SANDBOX_KEY").ok();

    let has_username = username.as_ref().is_some_and(|v| !v.is_empty());
    let has_key = key.as_ref().is_some_and(|v| !v.is_empty());
    let has_sandbox_key = sandbox_key.as_ref().is_some_and(|v| !v.is_empty());

    // Show variable status
    println!("Environment variables:");
    print_var_status("OPENAPI_USERNAME", has_username, &username);
    print_var_status("OPENAPI_KEY", has_key, &key);
    print_var_status("OPENAPI_SANDBOX_KEY", has_sandbox_key, &sandbox_key);
    println!();

    // Readiness assessment
    if has_username && has_key && has_sandbox_key {
        println!("Status: READY");
        println!("  Production and sandbox environments are both available.");
    } else if has_username && has_key {
        println!("Status: READY (production only)");
        println!("  Production environment is available.");
        println!("  Set OPENAPI_SANDBOX_KEY to enable sandbox mode (-S).");
    } else if has_username && has_sandbox_key {
        println!("Status: SANDBOX ONLY");
        println!("  Only the sandbox environment is available (use -S flag).");
        println!("  Set OPENAPI_KEY to enable production mode.");
    } else {
        println!("Status: NOT CONFIGURED");
        println!("  The CLI cannot operate. Set the required environment variables:");
        if !has_username {
            println!("    export OPENAPI_USERNAME=\"your-username\"");
        }
        if !has_key && !has_sandbox_key {
            println!("    export OPENAPI_KEY=\"your-api-key\"");
            println!("    export OPENAPI_SANDBOX_KEY=\"your-sandbox-key\"  (optional)");
        }
    }

    Ok(())
}

fn print_var_status(name: &str, is_set: bool, value: &Option<String>) {
    if is_set {
        let v = value.as_ref().unwrap();
        let masked = if v.len() > 6 {
            format!("{}...{}", &v[..3], &v[v.len() - 3..])
        } else {
            "*".repeat(v.len())
        };
        println!("  {:<25} SET ({})", name, masked);
    } else {
        println!("  {:<25} NOT SET", name);
    }
}
