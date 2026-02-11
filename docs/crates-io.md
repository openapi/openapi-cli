# Openapi® CLI

The official command-line interface to interact with **Openapi® APIs** directly from your terminal.

`openapi` provides structured access to the Openapi Marketplace, including OAuth token management, scope aliases, sandbox support, and service-specific commands.


## Overview

Openapi® CLI allows you to:

* Authenticate and manage OAuth tokens
* Call Openapi® services from the terminal
* Automate workflows in scripts and CI/CD pipelines
* Work with sandbox and production environments
* Use scope aliases for simplified token creation

It is designed to be predictable, script-friendly, and consistent across services.


## Pre-requisites

Before using the CLI, you need:

* An active account at [https://console.openapi.com/](https://console.openapi.com/)
* An API key for sandbox and/or production
* Valid credentials configured via environment variables


## Installation

Install from crates.io:

```bash
cargo install openapi-cli-rs
```

Or add it to your project:

```bash
cargo add openapi-cli-rs
```


## Configuration

The CLI relies on environment variables. There are two credential types depending on the command.


### 1️⃣ OAuth Credentials (Token Management)

Used by `openapi token` commands.

```bash
export OPENAPI_USERNAME="your-username"
export OPENAPI_KEY="your-api-key"
export OPENAPI_SANDBOX_KEY="your-sandbox-key"   # optional
```

These credentials are used for Basic authentication when creating or managing tokens.


### 2️⃣ Bearer Tokens (Service Commands)

Used by service commands like `sms`, `company`, `risk`, etc.

```bash
export OPENAPI_TOKEN="your-bearer-token"
export OPENAPI_SANDBOX_TOKEN="your-sandbox-token"   # optional
```

Tokens are generated using:

```bash
openapi token create --scopes "sms"
```


## Verify Configuration

To check your environment setup:

```bash
openapi info
```

This displays:

* All required environment variables
* Their configured status
* Token scopes (if available)


## Sandbox Mode

Use `-S` or `--sandbox` to target sandbox environments:

```bash
openapi -S sms send --to "+391234567890" --message "Test"
openapi -S token list
```


## Token Management

```bash
# List active tokens
openapi token list

# List all available scopes
openapi token scopes

# Create a new token
openapi token create --scopes "sms,company"

# Filter by HTTP method
openapi token create --scopes "post:sms,get:company"

# Check credit
openapi token credit

# Revoke a token
openapi token revoke --token "token-id"
```

Supported HTTP method filters:

`GET`, `POST`, `PUT`, `PATCH`, `DELETE`

Aliases and method prefixes are case insensitive.

If a scope does not match an alias, it is passed through as a literal scope.


## Available Services

The CLI provides commands for services including:

* `ai`
* `automotive`
* `cadastre`
* `certified-email`
* `chamber-of-commerce`
* `company`
* `docuengine`
* `domains`
* `esignature`
* `exchange-rate`
* `geocoding`
* `invoice`
* `massive-rem`
* `paying-bills`
* `pdf`
* `postal-service`
* `real-estate`
* `risk`
* `sdi`
* `sms`
* `time-stamping`
* `trust`
* `visengine`
* `zip-codes`

Run:

```bash
openapi --help
```

or

```bash
openapi <command> --help
```

for full command details.


## Design Principles

* Explicit over implicit
* Predictable command structure
* Script-friendly output
* Consistent behavior across services
* Environment-based configuration


## License

MIT License.

---

Openapi® CLI — bringing Openapi® to your terminal.
