#!/bin/bash
set -e
cargo run -q -- -S trust verify-email --email "test@example.com"
cargo run -q -- -S trust verify-phone --phone "+391234567890"
cargo run -q -- -S trust verify-fiscal-code --fiscal-code "RSSMRA85M01H501Z"
cargo run -q -- -S trust verify-vat --vat "12345678901"
