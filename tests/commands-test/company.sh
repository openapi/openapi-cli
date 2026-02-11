#!/bin/bash

cargo run -q -- -S company search --query "Openapi"
cargo run -q -- -S company start --code "12345678901"
cargo run -q -- -S company legalforms
