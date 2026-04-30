#!/bin/bash

cargo run -q -- -S oauthv2 scopes list
cargo run -q -- -S oauthv2 tokens list
cargo run -q -- -S oauthv2 wallet info
