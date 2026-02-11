#!/bin/bash

cargo run -q -- -S risk verify-cf --fiscal-code "RSSMRA85M01H501Z"
cargo run -q -- -S risk report-persona
cargo run -q -- -S risk report-azienda
