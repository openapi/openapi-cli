#!/bin/bash
set -e
cargo run -q -- -S docuengine business-register --tax-code "12345678901" --doc-type "visura"
cargo run -q -- -S docuengine revenue-agency --tax-code "12345678901" --doc-type "certificato"
cargo run -q -- -S docuengine inps --tax-code "12345678901" --doc-type "durc"
cargo run -q -- -S docuengine status --id "test-id-123"
