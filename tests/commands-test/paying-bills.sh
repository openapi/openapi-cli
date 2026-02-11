#!/bin/bash
set -e
cargo run -q -- -S paying-bills pay --bill-type "postal" --code "123456789" --amount 50.00
cargo run -q -- -S paying-bills status --id "test-id-123"
cargo run -q -- -S paying-bills list --limit 5
