#!/bin/bash
set -e
cargo run -q -- -S visengine request --tax-code "12345678901" --doc-type "visura" --source "chamber-of-commerce"
cargo run -q -- -S visengine status --id "test-id-123"
