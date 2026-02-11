#!/bin/bash
set -e
cargo run -q -- -S massive-rem send --from "test@pec.it" --to "dest@pec.it" --subject "Test" --body "Test body"
cargo run -q -- -S massive-rem status --id "test-id-123"
cargo run -q -- -S massive-rem list --limit 5
