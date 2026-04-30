#!/bin/bash

cargo run -q -- -S cap regions
cargo run -q -- -S cap provinces
cargo run -q -- -S cap cap --cap "00100"
cargo run -q -- -S cap search-municipalities --query "Roma"
