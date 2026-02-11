#!/bin/bash
set -e
cargo run -q -- -S chamber-of-commerce request --tax-code "12345678901" --doc-type "ordinaria"
cargo run -q -- -S chamber-of-commerce status --id "test-id-123"
cargo run -q -- -S chamber-of-commerce list --limit 5
