#!/bin/bash

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEST_DIR="$SCRIPT_DIR/commands-test"

passed=0
failed=0
errors=""

# A test passes if the CLI reaches the API server and gets a response.
# API errors (401, 403, 422...) are expected in sandbox without valid credentials.
# Only connection errors (dns, timeout, panic) count as failures.
run_test() {
    local name="$1"
    local script="$TEST_DIR/$name.sh"
    echo -n "  Testing $name ... "
    local output
    output=$(bash "$script" 2>&1)
    if echo "$output" | grep -qiE "dns error|connection refused|Connection reset|panic|RUST_BACKTRACE"; then
        echo "FAIL (connection error)"
        failed=$((failed + 1))
        errors="$errors\n  - $name"
    else
        echo "OK"
        passed=$((passed + 1))
    fi
}

echo "Running openapi CLI command tests (sandbox mode)"
echo "================================================="
echo "Note: API errors (401/403) are expected without valid credentials."
echo "      Tests verify the CLI reaches each server correctly."
echo ""

run_test "oauth"
run_test "oauthv2"
run_test "company"
run_test "smsv2"
run_test "esignature"
run_test "risk"
run_test "ai"
run_test "trust"
run_test "geocoding"
run_test "invoice"
run_test "automotive"
run_test "docuengine"
run_test "visurecamerali"
run_test "realestate"
run_test "cap"
run_test "visengine"
run_test "catasto"
run_test "ufficiopostale"
run_test "pecmassiva"
run_test "pdf"
run_test "marchetemporali"
run_test "pec"
run_test "bollettini"
run_test "exchange"
run_test "domains"
run_test "sdi"
run_test "info"

echo ""
echo "================================================="
echo "Results: $passed passed, $failed failed"
if [ $failed -gt 0 ]; then
    echo -e "Failed tests:$errors"
    exit 1
fi
