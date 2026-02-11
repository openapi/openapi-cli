#!/bin/bash

set -e

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
TEST_DIR="$SCRIPT_DIR/commands-test"

passed=0
failed=0
errors=""

run_test() {
    local name="$1"
    local script="$TEST_DIR/$name.sh"
    echo -n "  Testing $name ... "
    if bash "$script" > /dev/null 2>&1; then
        echo "OK"
        passed=$((passed + 1))
    else
        echo "FAIL"
        failed=$((failed + 1))
        errors="$errors\n  - $name"
    fi
}

echo "Running openapi CLI command tests (sandbox mode)"
echo "================================================="
echo ""

run_test "token"
run_test "company"
run_test "sms"
run_test "esignature"
run_test "risk"
run_test "ai"
run_test "trust"
run_test "geocoding"
run_test "invoice"
run_test "automotive"
run_test "docuengine"
run_test "chamber-of-commerce"
run_test "real-estate"
run_test "zip-codes"
run_test "visengine"
run_test "cadastre"
run_test "postal-service"
run_test "massive-rem"
run_test "pdf"
run_test "time-stamping"
run_test "certified-email"
run_test "paying-bills"
run_test "exchange-rate"
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
