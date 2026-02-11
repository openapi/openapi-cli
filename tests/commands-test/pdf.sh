#!/bin/bash
set -e
cargo run -q -- -S pdf convert --html "<h1>Hello</h1>" --output "/tmp/test-output.pdf"
cargo run -q -- -S pdf url --url "https://example.com" --output "/tmp/test-output.pdf"
