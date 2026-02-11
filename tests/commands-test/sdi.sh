#!/bin/bash

cargo run -q -- -S sdi invoices
cargo run -q -- -S sdi configurations
cargo run -q -- -S sdi api-configurations
