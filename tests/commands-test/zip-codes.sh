#!/bin/bash
set -e
cargo run -q -- -S zip-codes search --cap "00100"
cargo run -q -- -S zip-codes municipality --name "Roma"
cargo run -q -- -S zip-codes province --code "RM"
cargo run -q -- -S zip-codes regions
