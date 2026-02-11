#!/bin/bash
set -e
cargo run -q -- -S geocoding search --query "Piazza del Colosseo, Roma" --limit 5
cargo run -q -- -S geocoding reverse --lat 41.8902 --lon 12.4922
