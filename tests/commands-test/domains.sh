#!/bin/bash
set -e
cargo run -q -- -S domains check --domain "example.it"
cargo run -q -- -S domains info --domain "example.it"
cargo run -q -- -S domains list --limit 5
