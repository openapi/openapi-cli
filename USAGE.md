# openapi CLI - Usage Guide

## Configuration

The CLI requires API tokens set as environment variables:

```bash
export OPENAPI_TOKEN="your-production-token"
export OPENAPI_SANDBOX_TOKEN="your-sandbox-token"   # optional
```

Check your configuration status:

```bash
openapi info
```

## Sandbox Mode

Use the `-S` (or `--sandbox`) flag to run against sandbox environments:

```bash
openapi -S sms send --to "+391234567890" --message "Test"
```

## Scope Aliases

When creating tokens with `openapi token create --scopes`, you can use **scope aliases** instead of writing full scope strings.

### How it works

Each API service is mapped to an alias name. When you use an alias, it automatically expands to all available scopes for that service.

| Alias | Service | Domain |
|---|---|---|
| `ai` | AI language models | ai.openapi.com |
| `automotive` | Automotive data | automotive.openapi.com |
| `cadastre` | Italian cadastral data | catasto.openapi.it |
| `certified-email` | PEC / Legalmail | pec.openapi.it |
| `chamber-of-commerce` | Chamber of Commerce | visurecamerali.openapi.it |
| `company` | Company data | company.openapi.com |
| `docuengine` | Official documents | docuengine.openapi.com |
| `domains` | .it domain management | domains.altravia.com |
| `esignature` | Electronic signature | esignature.openapi.com |
| `exchange-rate` | Currency exchange rates | exchange.altravia.com |
| `geocoding` | Geocoding | geocoding.openapi.it |
| `invoice` | Electronic invoicing | invoice.openapi.com |
| `massive-rem` | Massive REM | ws.pecmassiva.com |
| `paying-bills` | Bills payment | ws.pagasubito.it |
| `pdf` | HTML to PDF | pdf.openapi.it |
| `postal-service` | Postal mail | ws.ufficiopostale.com |
| `real-estate` | Real estate valuation | realestate.openapi.com |
| `risk` | Risk reports and scoring | risk.openapi.com |
| `sdi` | SDI electronic invoicing | sdi.openapi.it |
| `sms` | SMS messaging | sms.openapi.com |
| `time-stamping` | Time stamping | ws.marchetemporali.com |
| `token` | OAuth token management | oauth.openapi.it |
| `trust` | Trust verification | trust.openapi.com |
| `visengine` | Official documents | visengine2.altravia.com |
| `zip-codes` | Zip codes and municipalities | cap.openapi.it |

### Examples

Create a token with all SMS scopes:

```bash
openapi token create --scopes "sms"
```

Create a token with all SMS and Company scopes:

```bash
openapi token create --scopes "sms,company"
```

### Method filtering

Prefix an alias with an HTTP method to include only scopes for that method:

```bash
# Only POST scopes for SMS
openapi token create --scopes "post:sms"

# Only GET scopes for company
openapi token create --scopes "get:company"

# Mix methods and full aliases
openapi token create --scopes "post:sms,get:company,geocoding"
```

Supported method prefixes: `GET`, `POST`, `PUT`, `PATCH`, `DELETE`.

### Case insensitive

Aliases and method prefixes are **case insensitive**:

```bash
openapi token create --scopes "SMS"
openapi token create --scopes "Post:Sms"
openapi token create --scopes "POST:SMS"
```

All of the above are equivalent.

### Literal scopes

If a term does not match any alias, it is passed through as a literal scope:

```bash
# Mix aliases and literal scopes
openapi token create --scopes "sms,GET:imprese.openapi.it/base"
```

## Token Management

```bash
# List active tokens
openapi token list

# List all available scopes
openapi token scopes

# Check credit
openapi token credit

# Revoke a token
openapi token revoke --token "token-id"
```

## Commands

Run `openapi --help` to see all available commands, or `openapi <command> --help` for subcommand details.
