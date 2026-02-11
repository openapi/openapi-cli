#!/bin/bash
set -e
cargo run -q -- -S invoice status --id "test-id-123"
cargo run -q -- -S invoice list --limit 5
