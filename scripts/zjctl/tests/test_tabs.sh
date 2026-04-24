#!/usr/bin/env bash
# tests/test_tabs.sh
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/harness.sh"
source "$PROJECT_ROOT/lib/output.sh"
source "$PROJECT_ROOT/lib/safety.sh"
source "$PROJECT_ROOT/lib/identity.sh"

echo "=== tabs tests ==="

export ZELLIJ_SESSION_NAME="test-session"

# Mock data: list-panes returns panes across tabs
MOCK_PANES='[
  {"id":0,"is_plugin":false,"is_focused":true,"is_floating":false,"is_suppressed":false,"title":"shell","is_selectable":true,"tab_id":0,"tab_position":0,"tab_name":"main","pane_command":"/bin/zsh","pane_cwd":"/home/user"},
  {"id":1,"is_plugin":true,"is_focused":false,"is_floating":false,"is_suppressed":false,"title":"zjstatus","is_selectable":false,"tab_id":0,"tab_position":0,"tab_name":"main"},
  {"id":2,"is_plugin":false,"is_focused":true,"is_floating":false,"is_suppressed":false,"title":"editor","is_selectable":true,"tab_id":1,"tab_position":1,"tab_name":"code","pane_command":"nvim","pane_cwd":"/home/user/project"}
]'

MOCK_TAB_INFO='{
  "position": 0,
  "name": "main",
  "active": true,
  "tab_id": 0
}'

mock_zellij_command "list-panes" "$MOCK_PANES"
mock_zellij_command "current-tab-info" "$MOCK_TAB_INFO"
mock_zellij_command "query-tab-names" "main
code"
mock_zellij_command "go-to-tab" ""
install_mock_zellij

describe "tabs list returns both tabs"
output=$("$PROJECT_ROOT/zjctl" tabs list 2>&1)
count=$(echo "$output" | jq '.tabs | length')
assert_eq "2" "$count"

describe "tabs list marks active tab"
active=$(echo "$output" | jq -r '.tabs[] | select(.active == true) | .name')
assert_eq "main" "$active"

describe "tabs list includes tab_id"
tab_id=$(echo "$output" | jq '.tabs[1].tab_id')
assert_eq "1" "$tab_id"

describe "tabs focus accepts numeric index"
output=$("$PROJECT_ROOT/zjctl" tabs focus 0 2>&1)
ok=$(echo "$output" | jq -r '.ok')
assert_eq "true" "$ok"

describe "tabs focus accepts name"
output=$("$PROJECT_ROOT/zjctl" tabs focus code 2>&1)
ok=$(echo "$output" | jq -r '.ok')
assert_eq "true" "$ok"

describe "tabs focus missing arg returns exit 1"
bash -c 'export PATH="'"$MOCK_DIR"':$PATH"; export ZELLIJ_SESSION_NAME=test; "'"$PROJECT_ROOT"'/zjctl" tabs focus' 2>/dev/null
actual_code=$?
assert_eq "1" "$actual_code"

describe "tabs list dry-run shows command"
output=$("$PROJECT_ROOT/zjctl" --dry-run tabs list 2>&1)
dry=$(echo "$output" | jq -r '.dry_run')
assert_eq "true" "$dry"

report
