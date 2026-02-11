#!/bin/bash
set -e
cargo run -q -- -S real-estate valuation --address "Via Roma 1" --municipality "Roma"
cargo run -q -- -S real-estate search --municipality "Roma"
cargo run -q -- -S real-estate omi --code "H501"
