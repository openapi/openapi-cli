#!/bin/bash
set -e
cargo run -q -- -S automotive plate --plate "AA000BB"
cargo run -q -- -S automotive chassis --vin "WVWZZZ3CZWE123456"
cargo run -q -- -S automotive insurance --plate "AA000BB"
