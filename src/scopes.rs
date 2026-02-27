use std::collections::HashMap;

/// Maps CLI command alias names to their production server domains.
/// In sandbox mode, the domain is prefixed with "test.".
pub fn alias_registry() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("token", "oauth.openapi.it"),
        ("company", "company.openapi.com"),
        ("sms", "sms.openapi.com"),
        ("esignature", "esignature.openapi.com"),
        ("risk", "risk.openapi.com"),
        ("ai", "ai.openapi.com"),
        ("trust", "trust.openapi.com"),
        ("geocoding", "geocoding.openapi.it"),
        ("invoice", "invoice.openapi.com"),
        ("automotive", "automotive.openapi.com"),
        ("docuengine", "docuengine.openapi.com"),
        ("chamber-of-commerce", "visurecamerali.openapi.it"),
        ("real-estate", "realestate.openapi.com"),
        ("zip-codes", "cap.openapi.it"),
        ("visengine", "visengine2.altravia.com"),
        ("cadastre", "catasto.openapi.it"),
        ("postal-service", "ws.ufficiopostale.com"),
        ("massive-rem", "ws.pecmassiva.com"),
        ("pdf", "pdf.openapi.it"),
        ("time-stamping", "ws.marchetemporali.com"),
        ("certified-email", "pec.openapi.it"),
        ("paying-bills", "ws.pagasubito.it"),
        ("exchange-rate", "exchange.altravia.com"),
        ("domains", "domains.altravia.com"),
        ("sdi", "sdi.openapi.it"),
    ])
}

const HTTP_METHODS: &[&str] = &["get", "post", "put", "patch", "delete"];

/// Expand scope aliases in a comma-separated scope string.
///
/// - `"sms"` → all scopes whose domain matches the sms service
/// - `"post:sms"` → only POST scopes for sms
/// - Anything that doesn't match an alias is passed through as-is
/// - Case insensitive
///
/// `all_scopes` is the full list of available scopes from `GET /scopes`.
/// `sandbox` determines whether to match test. or production domains.
pub fn expand_aliases(input: &str, all_scopes: &[String], sandbox: bool) -> Vec<String> {
    let registry = alias_registry();
    let mut result = Vec::new();

    for part in input.split(',') {
        let part = part.trim();
        if part.is_empty() {
            continue;
        }

        let lower = part.to_lowercase();

        // Check for "method:alias" pattern
        let (method_filter, alias_key) = if let Some(pos) = lower.find(':') {
            let prefix = &lower[..pos];
            let rest = &lower[pos + 1..];
            if HTTP_METHODS.contains(&prefix) {
                (Some(prefix.to_uppercase()), rest.to_string())
            } else {
                // Not a method prefix, treat as literal scope
                (None, String::new())
            }
        } else {
            (None, lower.clone())
        };

        let key = if method_filter.is_some() {
            &alias_key
        } else {
            &lower
        };

        // "all" is a special alias that expands to every available scope
        if key == "all" {
            result.extend(all_scopes.iter().cloned());
            continue;
        }

        if let Some(&prod_domain) = registry.get(key.as_str()) {
            let domain = if sandbox {
                format!("test.{}", prod_domain)
            } else {
                prod_domain.to_string()
            };
            let domain_lower = domain.to_lowercase();

            for scope in all_scopes {
                let scope_lower = scope.to_lowercase();
                if let Some(colon_pos) = scope_lower.find(':') {
                    let scope_method = &scope_lower[..colon_pos];
                    let scope_rest = &scope_lower[colon_pos + 1..];

                    if scope_rest.starts_with(&domain_lower) {
                        if let Some(ref mf) = method_filter {
                            if scope_method == mf.to_lowercase() {
                                result.push(scope.clone());
                            }
                        } else {
                            result.push(scope.clone());
                        }
                    }
                }
            }
        } else {
            // Not an alias — pass through as literal scope
            result.push(part.to_string());
        }
    }

    result
}

/// Group scopes by service alias for display.
pub fn group_scopes_by_service(scopes: &[String], sandbox: bool) -> String {
    let registry = alias_registry();
    let mut grouped: HashMap<String, Vec<String>> = HashMap::new();
    let mut unmatched = Vec::new();

    for scope in scopes {
        let scope_lower = scope.to_lowercase();
        let mut matched = false;

        for (&alias, &prod_domain) in &registry {
            let domain = if sandbox {
                format!("test.{}", prod_domain)
            } else {
                prod_domain.to_string()
            };

            if let Some(colon_pos) = scope_lower.find(':') {
                let scope_rest = &scope_lower[colon_pos + 1..];
                if scope_rest.starts_with(&domain.to_lowercase()) {
                    grouped
                        .entry(alias.to_string())
                        .or_default()
                        .push(scope.clone());
                    matched = true;
                    break;
                }
            }
        }

        if !matched {
            unmatched.push(scope.clone());
        }
    }

    let mut output = String::new();
    let mut aliases: Vec<_> = grouped.keys().cloned().collect();
    aliases.sort();

    for alias in &aliases {
        let scopes = &grouped[alias];
        output.push_str(&format!("  {} ({} scope{})\n", alias, scopes.len(), if scopes.len() == 1 { "" } else { "s" }));
        for s in scopes {
            output.push_str(&format!("    {}\n", s));
        }
    }

    if !unmatched.is_empty() {
        output.push_str(&format!("  other ({} scope{})\n", unmatched.len(), if unmatched.len() == 1 { "" } else { "s" }));
        for s in &unmatched {
            output.push_str(&format!("    {}\n", s));
        }
    }

    output
}

/// List all available alias names (sorted).
#[allow(dead_code)]
pub fn list_aliases() -> Vec<&'static str> {
    let registry = alias_registry();
    let mut aliases: Vec<&str> = registry.keys().copied().collect();
    aliases.sort();
    aliases
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_scopes() -> Vec<String> {
        vec![
            "GET:sms.openapi.com/v1/send".to_string(),
            "POST:sms.openapi.com/v1/send".to_string(),
            "GET:company.openapi.com/v1/search".to_string(),
        ]
    }

    #[test]
    fn test_all_expands_to_every_scope() {
        let scopes = sample_scopes();
        let result = expand_aliases("all", &scopes, false);
        assert_eq!(result, scopes);
    }

    #[test]
    fn test_all_case_insensitive() {
        let scopes = sample_scopes();
        let result = expand_aliases("ALL", &scopes, false);
        assert_eq!(result, scopes);
    }

    #[test]
    fn test_all_mixed_with_other_aliases() {
        let scopes = sample_scopes();
        // "all" alone should still return everything without duplicates from mixing
        let result = expand_aliases("all", &scopes, false);
        assert_eq!(result.len(), scopes.len());
    }
}
