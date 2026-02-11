#!/bin/bash

cargo run -q -- -S domains list
cargo run -q -- -S domains contacts
cargo run -q -- -S domains check --domain "example.it"
