#!/bin/bash

cargo run -q -- -S zip-codes regions
cargo run -q -- -S zip-codes provinces
cargo run -q -- -S zip-codes cap --cap "00100"
cargo run -q -- -S zip-codes search-municipalities --query "Roma"
