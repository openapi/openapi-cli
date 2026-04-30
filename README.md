<div align="center">
  <a href="https://openapi.com/">
    <img alt="Openapi CLI" src=".github/assets/images/repo-header-a4.png" >
  </a>

  <h1>🔐 Openapi® CLI</h1>
  <h4>The command-line interface to interact with <a href="https://openapi.com/">Openapi®</a> APIs directly from your terminal</h4>

[![Build Status](https://github.com/openapi/openapi-cli/actions/workflows/rust.yml/badge.svg)](https://github.com/openapi/openapi-cli/actions/workflows/rust.yml)
[![Crates.io](https://img.shields.io/crates/v/openapi-cli-rs.svg)](https://crates.io/crates/openapi-cli-rs)
[![License](https://img.shields.io/github/license/openapi/openapi-cli?ts=1771243284)](LICENSE)
[![Rust Version](https://img.shields.io/badge/rust-2024_edition-orange.svg)](https://www.rust-lang.org/)
<br>
[![Linux Foundation Member](https://img.shields.io/badge/Linux%20Foundation-Silver%20Member-003778?logo=linux-foundation&logoColor=white)](https://www.linuxfoundation.org/about/members)
</div>

## Overview

A command-line client for the APIs available at [Openapi®](https://openapi.com).
It provides direct terminal access to the Openapi® Marketplace, including OAuth and OAuth v2 token management, scope aliases, and sandbox support.
With this CLI you can quickly interact with hundreds of certified APIs and accelerate your digital transformation projects.

## Pre-requisites

Before using the Openapi CLI, you will need an account at [Openapi®](https://console.openapi.com/) and an API key to the sandbox and/or production environment.

## What you can do

With the Openapi CLI, you can easily interact with a variety of services in the Openapi Marketplace. For example, you can:

- Send SMS messages with delivery reports and custom sender IDs
- Process bills and payments in real time via API
- Send electronic invoices securely to the Italian Revenue Agency
- Generate PDFs from HTML content, including JavaScript rendering
- Manage certified emails and legal communications via Italian Legalmail
- Query company data, risk reports, automotive records, and more

For a complete list of all available services, check out the [Openapi® Marketplace](https://console.openapi.com/).
There is also a small terminal easter egg available through `openapi --who`.

## Installation

### From crates.io

```bash
cargo install openapi-cli-rs
```

### From source

```bash
git clone https://github.com/openapi/openapi-cli.git
cd openapi-cli
cargo install --path .
```

Or using make:

```bash
make install
```

The binary `openapi` will be installed in `~/.cargo/bin/`.

### Build without installing

```bash
cargo build --release
```

The binary will be available at `target/release/openapi`.

## Configuration

The CLI uses two sets of environment variables depending on the type of command.

### OAuth credentials (for token management)

Used by `openapi oauth` and `openapi oauthv2` commands. These are your Openapi account credentials (Basic auth).

```bash
export OPENAPI_USERNAME="your-username"
export OPENAPI_KEY="your-api-key"
export OPENAPI_SANDBOX_KEY="your-sandbox-key"        # optional
```

### Bearer tokens (for service API commands)

Used by all service commands (`smsv2`, `company`, `risk`, etc.). These are tokens generated via `openapi oauth create` or `openapi oauthv2 tokens create`.

```bash
export OPENAPI_TOKEN="your-bearer-token"
export OPENAPI_SANDBOX_TOKEN="your-sandbox-token"    # optional
```

### Verify your setup

```bash
openapi info
```

This shows all 5 environment variables, their status, and the scopes attached to your tokens.

## Sandbox mode

Use the `-S` (or `--sandbox`) flag to run against sandbox environments:

```bash
openapi -S smsv2 send --to "+391234567890" --message "Test"
openapi -S oauth list
```

## Token management

```bash
# List active tokens
openapi oauth list

# List all available scopes
openapi oauth scopes

# Create a new token
openapi oauth create --scopes "smsv2"

# Check credit
openapi oauth credit

# Revoke a token
openapi oauth revoke --token "token-id"

# OAuth v2 token creation
openapi oauthv2 tokens create --scopes "smsv2"

# OAuth v2 scopes
openapi oauthv2 scopes list
```

## Scope aliases

When creating tokens with `openapi oauth create --scopes`, you can use **scope aliases** instead of writing full scope strings.

### How it works

Each API service is mapped to an alias name. When you use an alias, it automatically expands to all available scopes for that service.

| Alias | Service | Domain |
|---|---|---|
| `all` | **All services** _(special keyword)_ | — |
| `ai` | AI language models | ai.openapi.com |
| `automotive` | Automotive data | automotive.openapi.com |
| `bollettini` | Bills payment | ws.pagasubito.it |
| `cap` | Zip codes and municipalities | cap.openapi.it |
| `catasto` | Italian cadastral data | catasto.openapi.it |
| `company` | Company data | company.openapi.com |
| `docuengine` | Official documents | docuengine.openapi.com |
| `domains` | .it domain management | domains.altravia.com |
| `esignature` | Electronic signature | esignature.openapi.com |
| `exchange` | Currency exchange rates | exchange.altravia.com |
| `geocoding` | Geocoding | geocoding.openapi.it |
| `invoice` | Electronic invoicing | invoice.openapi.com |
| `marchetemporali` | Time stamping | ws.marchetemporali.com |
| `oauth` | OAuth token management | oauth.openapi.it |
| `oauthv2` | OAuth v2 token management | oauth.openapi.com |
| `pec` | PEC / Legalmail | pec.openapi.it |
| `pecmassiva` | Massive REM | ws.pecmassiva.com |
| `pdf` | HTML to PDF | pdf.openapi.it |
| `realestate` | Real estate valuation | realestate.openapi.com |
| `risk` | Risk reports and scoring | risk.openapi.com |
| `sdi` | SDI electronic invoicing | sdi.openapi.it |
| `smsv2` | SMS messaging | sms.openapi.com |
| `trust` | Trust verification | trust.openapi.com |
| `ufficiopostale` | Postal mail | ws.ufficiopostale.com |
| `visengine` | Official documents | visengine2.altravia.com |
| `visurecamerali` | Chamber of Commerce | visurecamerali.openapi.it |

### Examples

```bash
# All SMS scopes
openapi oauth create --scopes "smsv2"

# Multiple services
openapi oauth create --scopes "smsv2,company"

# All available scopes (special keyword)
openapi oauth create --scopes "all"
```

### Method filtering

Prefix an alias with an HTTP method to include only scopes for that method:

```bash
# Only POST scopes for SMS
openapi oauth create --scopes "post:smsv2"

# Only GET scopes for company
openapi oauth create --scopes "get:company"

# Mix methods and full aliases
openapi oauth create --scopes "post:smsv2,get:company,geocoding"
```

Supported method prefixes: `GET`, `POST`, `PUT`, `PATCH`, `DELETE`.

### Case insensitive

Aliases and method prefixes are case insensitive:

```bash
openapi oauth create --scopes "SMSV2"
openapi oauth create --scopes "Post:SmsV2"
openapi oauth create --scopes "POST:SMSV2"
```

All of the above are equivalent.

### Literal scopes

If a term does not match any alias, it is passed through as a literal scope:

```bash
openapi oauth create --scopes "smsv2,GET:imprese.openapi.it/base"
```

## Available commands

| Command | Description |
|---|---|
| `info` | Show configuration status and readiness |
| `oauth` | OAuth token management |
| `ai` | AI language models |
| `automotive` | Automotive data (vehicles, insurance) |
| `bollettini` | Bills payment |
| `cap` | Zip codes, municipalities, provinces, regions |
| `catasto` | Italian cadastral data |
| `company` | Company data and information |
| `docuengine` | Official documents (Business Register, Revenue Agency, INPS) |
| `domains` | .it domain management |
| `esignature` | Electronic signature |
| `exchange` | Foreign currency exchange rates |
| `geocoding` | Geocoding and reverse geocoding |
| `invoice` | Electronic invoicing |
| `marchetemporali` | Document time stamping |
| `pec` | Italian certified email (PEC / Legalmail) |
| `pecmassiva` | Massive Registered Electronic Mail |
| `pdf` | HTML to PDF conversion |
| `realestate` | Real estate valuation data |
| `risk` | Risk reports and scoring |
| `sdi` | SDI electronic invoicing |
| `smsv2` | SMS messaging (v2) |
| `oauthv2` | OAuth v2 token management and analytics |
| `trust` | Trust verification services |
| `ufficiopostale` | Postal mail service |
| `visengine` | Official documents (Chamber of Commerce, INPS, Tax Agency) |
| `visurecamerali` | Chamber of Commerce documents |

Run `openapi --help` for the full list, or `openapi <command> --help` for subcommand details.

## Development

```bash
# Build
make build

# Run tests
make test-commands

# Download/update OpenAPI specs
make oas-download
```

## Contributing

Contributions are always welcome! Whether you want to report bugs, suggest new features, improve documentation, or contribute code, your help is appreciated.

## Authors

- Francesco Bianco ([@francescobianco](https://www.github.com/francescobianco))
- Openapi Team ([@openapi-it](https://github.com/openapi-it))

## Partners

Meet our partners using Openapi or contributing to this project:

- [Blank](https://www.blank.app/)
- [Credit Safe](https://www.creditsafe.com/)
- [Deliveroo](https://deliveroo.it/)
- [Gruppo MOL](https://molgroupitaly.it/it/)
- [Jakala](https://www.jakala.com/)
- [Octotelematics](https://www.octotelematics.com/)
- [OTOQI](https://otoqi.com/)
- [PWC](https://www.pwc.com/)
- [QOMODO S.R.L.](https://www.qomodo.me/)
- [SOUNDREEF S.P.A.](https://www.soundreef.com/)

## Our Commitments

We believe in open source and we act on that belief. We became Silver Members 
of the Linux Foundation because we wanted to formally support the ecosystem 
we build on every day. Open standards, open collaboration, and open governance 
are part of how we work and how we think about software.

## License

This project is licensed under the [MIT License](LICENSE).

The MIT License is a permissive open-source license that allows you to freely use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the software, provided that the original copyright notice and this permission notice are included in all copies or substantial portions of the software.

For more details, see the full license text at the [MIT License page](https://choosealicense.com/licenses/mit/).
