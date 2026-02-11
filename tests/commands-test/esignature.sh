#!/bin/bash
set -e
cargo run -q -- -S esignature create --document "/tmp/test.pdf" --signer-email "test@example.com" --signer-name "Test User"
cargo run -q -- -S esignature status --id "test-id-123"
cargo run -q -- -S esignature list --limit 5
