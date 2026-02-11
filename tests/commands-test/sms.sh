#!/bin/bash
set -e
cargo run -q -- -S sms send --to "+391234567890" --message "Test from CLI"
cargo run -q -- -S sms status --id "test-id-123"
cargo run -q -- -S sms history --limit 5
