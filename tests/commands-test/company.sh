#!/bin/bash
set -e
cargo run -q -- -S company search --query "Openapi Srl"
cargo run -q -- -S company get --tax-code "12345678901"
cargo run -q -- -S company balance --tax-code "12345678901"
cargo run -q -- -S company pec --tax-code "12345678901"
