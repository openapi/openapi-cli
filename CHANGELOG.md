# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - 0.2.0

### Added

- **Scope alias `all`** — passing `--scopes all` when creating a token now expands to every available scope returned by the API. The keyword is case-insensitive (`ALL`, `All`, etc. are all valid).

  ```bash
  openapi token create --scopes all
  ```

---

## [0.1.0] - 2026-02-16

### Added

- First public release of the Openapi CLI
- Token management: `create`, `list`, `revoke`, `scopes`, `credit`
- Scope aliases — use short service names instead of full scope strings when creating tokens
- Method filtering for scope aliases (`post:sms`, `get:company`, …)
- Sandbox mode via `-S` / `--sandbox` flag
- Service commands: `ai`, `automotive`, `cadastre`, `certified-email`, `chamber-of-commerce`, `company`, `docuengine`, `domains`, `esignature`, `exchange-rate`, `geocoding`, `invoice`, `massive-rem`, `paying-bills`, `pdf`, `postal-service`, `real-estate`, `risk`, `sdi`, `sms`, `time-stamping`, `trust`, `visengine`, `zip-codes`
- `info` command to verify environment variables and token scopes
