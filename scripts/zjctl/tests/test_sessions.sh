#!/usr/bin/env bash
# tests/test_sessions.sh
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/harness.sh"
source "$PROJECT_ROOT/lib/output.sh"
source "$PROJECT_ROOT/lib/safety.sh"
source "$PROJECT_ROOT/lib/identity.sh"

echo "=== sessions list tests ==="

describe "sessions list returns JSON with session names"
mock_zellij_command "list-sessions" "didactic-newt
ambrosio
clauded"
install_mock_zellij
export ZELLIJ_SESSION_NAME="ambrosio"
output=$("$PROJECT_ROOT/zjctl" sessions list 2>&1)
count=$(echo "$output" | jq '.sessions | length')
assert_eq "3" "$count"

describe "sessions list first session name is correct"
first=$(echo "$output" | jq -r '.sessions[0].name')
assert_eq "didactic-newt" "$first"

describe "sessions list table format"
output=$("$PROJECT_ROOT/zjctl" --format table sessions list 2>&1)
assert_contains "$output" "ambrosio"

describe "sessions list quiet format produces no output"
output=$("$PROJECT_ROOT/zjctl" --quiet sessions list 2>&1)
assert_eq "" "$output"

describe "unknown resource returns exit code 1"
bash -c 'export PATH="'"$MOCK_DIR"':$PATH"; export ZELLIJ_SESSION_NAME=test; "'"$PROJECT_ROOT"'/zjctl" foobar list' 2>/dev/null
actual_code=$?
assert_eq "1" "$actual_code"

describe "missing verb returns exit code 1"
bash -c 'export PATH="'"$MOCK_DIR"':$PATH"; export ZELLIJ_SESSION_NAME=test; "'"$PROJECT_ROOT"'/zjctl" sessions' 2>/dev/null
actual_code=$?
assert_eq "1" "$actual_code"

report
