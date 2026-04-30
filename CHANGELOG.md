# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- Renamed CLI service commands and scope aliases to use the official Openapi console slugs from `oas/00-list.txt`.
- Replaced legacy aliases such as `token`, `sms`, `cadastre`, `certified-email`, `chamber-of-commerce`, `exchange-rate`, `massive-rem`, `paying-bills`, `postal-service`, `real-estate`, `time-stamping`, and `zip-codes` with `oauth`, `smsv2`, `catasto`, `pec`, `visurecamerali`, `exchange`, `pecmassiva`, `bollettini`, `ufficiopostale`, `realestate`, `marchetemporali`, and `cap`.

### Added

- Added `oauthv2` as a separate command for the new OAuth v2 API on `oauth.openapi.com`, alongside the existing `oauth` command on `oauth.openapi.it`.

---

## [0.1.1] - 2026-04-02

### Added

- **Scope alias `all`** — passing `--scopes all` when creating a token now expands to every available scope returned by the API. The keyword is case-insensitive (`ALL`, `All`, etc. are all valid).

  ```bash
  openapi oauth create --scopes all
  ```

- Global `--who` easter egg powered by the external `billy-ray` crate.

### Changed

- Updated `billy-ray` dependency to `0.1.1`.
- Refined command handling so global flags can work without forcing a subcommand.

---

## [0.1.0] - 2026-02-16

### Added

- First public release of the Openapi CLI
- Token management: `create`, `list`, `revoke`, `scopes`, `credit`
- Scope aliases — use short service names instead of full scope strings when creating tokens
- Method filtering for scope aliases (`post:smsv2`, `get:company`, …)
- Sandbox mode via `-S` / `--sandbox` flag
- Service commands: `ai`, `automotive`, `bollettini`, `cap`, `catasto`, `company`, `docuengine`, `domains`, `esignature`, `exchange`, `geocoding`, `invoice`, `marchetemporali`, `oauth`, `pec`, `pecmassiva`, `pdf`, `realestate`, `risk`, `sdi`, `smsv2`, `trust`, `ufficiopostale`, `visengine`, `visurecamerali`
- `info` command to verify environment variables and token scopes
