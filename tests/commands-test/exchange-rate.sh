#!/bin/bash
set -e
cargo run -q -- -S exchange-rate get --from "EUR" --to "USD"
cargo run -q -- -S exchange-rate currencies
cargo run -q -- -S exchange-rate history --from "EUR" --to "USD" --date "2025-01-01"
