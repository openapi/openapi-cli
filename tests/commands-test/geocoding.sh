#!/bin/bash

cargo run -q -- -S geocoding search --query "Piazza del Colosseo, Roma"
cargo run -q -- -S geocoding reverse --lat 41.8902 --lon 12.4922
