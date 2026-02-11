#!/bin/bash
set -e
cargo run -q -- -S sdi status --id "test-id-123"
cargo run -q -- -S sdi list --limit 5
cargo run -q -- -S sdi notification --id "test-id-123"
