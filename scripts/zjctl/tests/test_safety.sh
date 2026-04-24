#!/usr/bin/env bash
# tests/test_safety.sh
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/harness.sh"
source "$PROJECT_ROOT/lib/output.sh"
source "$PROJECT_ROOT/lib/safety.sh"

echo "=== safety.sh tests ==="

# Test: self-write guard blocks when target matches ZELLIJ_PANE_ID
describe "self-write guard blocks matching pane ID"
export ZELLIJ_PANE_ID=8
ZJCTL_NO_GUARD=""
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; source "'"$PROJECT_ROOT"'/lib/safety.sh"; ZELLIJ_PANE_ID=8; ZJCTL_NO_GUARD=""; check_self_write "terminal_8"' 2>&1 || true)
error=$(echo "$output" | jq -r '.error' 2>/dev/null || echo "")
assert_eq "self_write_blocked" "$error"

describe "self-write guard blocks bare integer matching ZELLIJ_PANE_ID"
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; source "'"$PROJECT_ROOT"'/lib/safety.sh"; ZELLIJ_PANE_ID=8; ZJCTL_NO_GUARD=""; check_self_write "8"' 2>&1 || true)
error=$(echo "$output" | jq -r '.error' 2>/dev/null || echo "")
assert_eq "self_write_blocked" "$error"

describe "self-write guard allows different pane"
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; source "'"$PROJECT_ROOT"'/lib/safety.sh"; ZELLIJ_PANE_ID=8; ZJCTL_NO_GUARD=""; check_self_write "terminal_3"' 2>&1)
assert_eq "" "$output"

describe "self-write guard bypassed with --no-guard"
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; source "'"$PROJECT_ROOT"'/lib/safety.sh"; ZELLIJ_PANE_ID=8; ZJCTL_NO_GUARD="true"; check_self_write "terminal_8"' 2>&1)
assert_eq "" "$output"

describe "self-write guard skipped when ZELLIJ_PANE_ID not set"
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; source "'"$PROJECT_ROOT"'/lib/safety.sh"; unset ZELLIJ_PANE_ID; ZJCTL_NO_GUARD=""; check_self_write "terminal_8"' 2>&1)
assert_eq "" "$output"

describe "self-write guard exit code is 3"
bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; source "'"$PROJECT_ROOT"'/lib/safety.sh"; ZELLIJ_PANE_ID=8; ZJCTL_NO_GUARD=""; check_self_write "terminal_8"' 2>/dev/null
actual_code=$?
assert_eq "3" "$actual_code"

report
