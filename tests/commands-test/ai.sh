#!/bin/bash
set -e
cargo run -q -- -S ai query --prompt "What is Openapi?"
cargo run -q -- -S ai models
