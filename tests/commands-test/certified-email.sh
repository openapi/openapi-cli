#!/bin/bash
set -e
cargo run -q -- -S certified-email send --from "test@pec.it" --to "dest@pec.it" --subject "Test" --body "Test body"
cargo run -q -- -S certified-email status --id "test-id-123"
cargo run -q -- -S certified-email list --limit 5
