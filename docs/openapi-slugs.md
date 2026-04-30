# OpenAPI Slugs

This repository treats an API slug as official only when it resolves on the Openapi console:

```text
https://console.openapi.com/apis/<openapi-slug>/info
```

If that URL returns `404`, the slug is not official and must not be exposed as a top-level CLI command.

## Source of Truth

The local source of truth is [`oas/00-list.txt`](/home/francesco/Develop/Openapi/openapi-cli/oas/00-list.txt), which downloads specs from:

```text
https://console.openapi.com/oas/en/<openapi-slug>.openapi.json
```

The command surface of this CLI must therefore use the official slug, not a descriptive alias based on marketing copy, translated names, or server domains.

`oauthv2` is an additional official slug and does not replace `oauth`.

## Legacy To Official Mapping

| Legacy command | Official slug | Notes |
|---|---|---|
| `token` | `oauth` | Token management belongs to the OAuth API slug |
| `sms` | `smsv2` | Current CLI targets SMS v2 |
| `cadastre` | `catasto` | Use the official Italian slug |
| `certified-email` | `pec` | Official slug is PEC |
| `chamber-of-commerce` | `visurecamerali` | Use registry document slug |
| `exchange-rate` | `exchange` | Console slug is `exchange` |
| `massive-rem` | `pecmassiva` | Console slug is `pecmassiva` |
| `paying-bills` | `bollettini` | Console slug is `bollettini` |
| `postal-service` | `ufficiopostale` | Console slug is `ufficiopostale` |
| `real-estate` | `realestate` | Console slug has no dash |
| `time-stamping` | `marchetemporali` | Use official Italian slug |
| `zip-codes` | `cap` | Console slug is `cap` |

These commands already matched the official slug and remain unchanged:

- `ai`
- `automotive`
- `company`
- `docuengine`
- `domains`
- `esignature`
- `geocoding`
- `invoice`
- `oauthv2`
- `pdf`
- `risk`
- `sdi`
- `trust`
- `visengine`

## Rule For New Commands

Before adding a new command:

1. Verify the slug resolves on `https://console.openapi.com/apis/<slug>/info`.
2. Verify the spec is downloadable from `https://console.openapi.com/oas/en/<slug>.openapi.json`.
3. Use that exact slug as the CLI command name and as the scope alias key.

If only a non-official alias exists, the command should not be added.
