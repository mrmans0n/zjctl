#!/usr/bin/env bash
# tests/test_identity.sh
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/harness.sh"
source "$PROJECT_ROOT/lib/output.sh"
source "$PROJECT_ROOT/lib/safety.sh"
source "$PROJECT_ROOT/lib/identity.sh"

echo "=== identity.sh tests ==="

# --- Tab resolution ---

describe "resolve_tab passes through numeric index"
result=$(resolve_tab "2")
assert_eq "2" "$result"

describe "resolve_tab resolves name via query-tab-names"
mock_zellij_command "query-tab-names" "openclaw
claude-remote
opencode"
install_mock_zellij
result=$(resolve_tab "opencode")
assert_eq "2" "$result"

describe "resolve_tab resolves first tab by name"
result=$(resolve_tab "openclaw")
assert_eq "0" "$result"

describe "resolve_tab errors on unknown name"
output=$(bash -c '
    export PATH="'"$MOCK_DIR"':$PATH"
    source "'"$PROJECT_ROOT"'/lib/error.sh"
    source "'"$PROJECT_ROOT"'/lib/output.sh"
    source "'"$PROJECT_ROOT"'/lib/safety.sh"
    source "'"$PROJECT_ROOT"'/lib/identity.sh"
    resolve_tab "nonexistent"
' 2>&1 || true)
error=$(echo "$output" | jq -r '.error' 2>/dev/null || echo "")
assert_eq "invalid_target" "$error"

# --- Pane resolution ---

describe "resolve_pane passes through terminal_N"
result=$(resolve_pane "terminal_8")
assert_eq "terminal_8" "$result"

describe "resolve_pane passes through plugin_N"
result=$(resolve_pane "plugin_2")
assert_eq "plugin_2" "$result"

describe "resolve_pane converts bare integer to terminal_N"
result=$(resolve_pane "8")
assert_eq "terminal_8" "$result"

describe "resolve_pane resolves by title"
mock_zellij_command "list-panes" '[
    {"id": 0, "is_plugin": false, "title": "openclaw", "tab_id": 0},
    {"id": 7, "is_plugin": false, "title": "my-shell", "tab_id": 0},
    {"id": 2, "is_plugin": true, "title": "zjstatus", "tab_id": 0}
]'
install_mock_zellij
result=$(resolve_pane "my-shell")
assert_eq "terminal_7" "$result"

describe "resolve_pane errors on unknown name"
output=$(bash -c '
    export PATH="'"$MOCK_DIR"':$PATH"
    source "'"$PROJECT_ROOT"'/lib/error.sh"
    source "'"$PROJECT_ROOT"'/lib/output.sh"
    source "'"$PROJECT_ROOT"'/lib/safety.sh"
    source "'"$PROJECT_ROOT"'/lib/identity.sh"
    resolve_pane "nonexistent"
' 2>&1 || true)
error=$(echo "$output" | jq -r '.error' 2>/dev/null || echo "")
assert_eq "invalid_target" "$error"

report
