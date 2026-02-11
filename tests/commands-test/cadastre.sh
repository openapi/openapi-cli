#!/bin/bash
set -e
cargo run -q -- -S cadastre person --fiscal-code "RSSMRA85M01H501Z"
cargo run -q -- -S cadastre property --municipality-code "H501" --sheet "100" --parcel "200"
cargo run -q -- -S cadastre owners --municipality-code "H501" --sheet "100" --parcel "200"
