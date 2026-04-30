# Openapi® CLI

The official command-line interface to interact with **Openapi® APIs** directly from your terminal.

`openapi` provides structured access to the Openapi Marketplace, including OAuth and OAuth v2 token management, scope aliases, sandbox support, and service-specific commands.


## Overview

Openapi® CLI allows you to:

* Authenticate and manage OAuth and OAuth v2 tokens
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

Used by `openapi oauth` and `openapi oauthv2` commands.

```bash
export OPENAPI_USERNAME="your-username"
export OPENAPI_KEY="your-api-key"
export OPENAPI_SANDBOX_KEY="your-sandbox-key"   # optional
```

These credentials are used for Basic authentication when creating or managing OAuth and OAuth v2 tokens.


### 2️⃣ Bearer Tokens (Service Commands)

Used by service commands like `smsv2`, `company`, `risk`, etc.

```bash
export OPENAPI_TOKEN="your-bearer-token"
export OPENAPI_SANDBOX_TOKEN="your-sandbox-token"   # optional
```

Tokens are generated using:

```bash
openapi oauth create --scopes "smsv2"
```

or

```bash
openapi oauthv2 tokens create --scopes "smsv2"
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
openapi -S smsv2 send --to "+391234567890" --message "Test"
openapi -S oauth list
```


## Token Management

```bash
# List active tokens
openapi oauth list

# List all available scopes
openapi oauth scopes

# Create a new token
openapi oauth create --scopes "smsv2,company"

# Filter by HTTP method
openapi oauth create --scopes "post:smsv2,get:company"

# Check credit
openapi oauth credit

# Revoke a token
openapi oauth revoke --token "token-id"

# OAuth v2 token creation
openapi oauthv2 tokens create --scopes "smsv2"

# OAuth v2 scopes
openapi oauthv2 scopes list
```

Supported HTTP method filters:

`GET`, `POST`, `PUT`, `PATCH`, `DELETE`

Aliases and method prefixes are case insensitive.

If a scope does not match an alias, it is passed through as a literal scope.


## Available Services

The CLI provides commands for services including:

* `ai`
* `automotive`
* `bollettini`
* `cap`
* `catasto`
* `company`
* `docuengine`
* `domains`
* `esignature`
* `exchange`
* `geocoding`
* `invoice`
* `marchetemporali`
* `oauth`
* `oauthv2`
* `pec`
* `pecmassiva`
* `pdf`
* `realestate`
* `risk`
* `sdi`
* `smsv2`
* `trust`
* `ufficiopostale`
* `visengine`
* `visurecamerali`

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
