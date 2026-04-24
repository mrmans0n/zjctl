#!/usr/bin/env bash
# tests/test_error.sh
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/harness.sh"

echo "=== error.sh tests ==="

# Test: emit_error produces valid JSON with correct fields
describe "emit_error produces valid JSON for unknown_command"
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; emit_error "unknown_command" "bad command"' 2>&1 || true)
error_field=$(echo "$output" | jq -r '.error')
assert_eq "unknown_command" "$error_field"

describe "emit_error default exit code for unknown_command is 1"
bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; emit_error "unknown_command" "bad"' 2>/dev/null
actual_code=$?
assert_eq "1" "$actual_code"

describe "emit_error default exit code for zellij_error is 2"
bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; emit_error "zellij_error" "fail"' 2>/dev/null
actual_code=$?
assert_eq "2" "$actual_code"

describe "emit_error default exit code for self_write_blocked is 3"
bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; emit_error "self_write_blocked" "nope"' 2>/dev/null
actual_code=$?
assert_eq "3" "$actual_code"

describe "emit_error default exit code for not_in_session is 4"
bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; emit_error "not_in_session" "no session"' 2>/dev/null
actual_code=$?
assert_eq "4" "$actual_code"

describe "emit_error includes message field"
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; emit_error "missing_argument" "need pane id"' 2>&1 || true)
msg=$(echo "$output" | jq -r '.message')
assert_eq "need pane id" "$msg"

describe "emit_error merges extra JSON fields"
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; emit_error "self_write_blocked" "blocked" "" "{\"target\": \"terminal_8\", \"self\": \"terminal_8\"}"' 2>&1 || true)
target=$(echo "$output" | jq -r '.target')
assert_eq "terminal_8" "$target"

describe "emit_zellij_error includes command field"
output=$(bash -c 'source "'"$PROJECT_ROOT"'/lib/error.sh"; emit_zellij_error "failed" "zellij action write-chars foo"' 2>&1 || true)
cmd=$(echo "$output" | jq -r '.command')
assert_eq "zellij action write-chars foo" "$cmd"

report
