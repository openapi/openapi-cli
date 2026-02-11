#!/bin/bash
set -e
cargo run -q -- -S risk report --tax-code "12345678901"
cargo run -q -- -S risk person --fiscal-code "RSSMRA85M01H501Z"
cargo run -q -- -S risk list --limit 5
