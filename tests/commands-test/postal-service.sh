#!/bin/bash
set -e
cargo run -q -- -S postal-service send --to-name "Mario Rossi" --to-address "Via Roma 1, 00100 Roma" --document "/tmp/test.pdf"
cargo run -q -- -S postal-service status --id "test-id-123"
cargo run -q -- -S postal-service list --limit 5
