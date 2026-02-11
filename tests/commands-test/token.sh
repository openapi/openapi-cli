#!/bin/bash
set -e
cargo run -q -- -S token create --scopes "GET:test.imprese.openapi.it/advance"
cargo run -q -- -S token list
