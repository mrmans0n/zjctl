#!/usr/bin/env bash
# tests/test_panes.sh
set -uo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
source "$SCRIPT_DIR/harness.sh"
source "$PROJECT_ROOT/lib/output.sh"
source "$PROJECT_ROOT/lib/safety.sh"
source "$PROJECT_ROOT/lib/identity.sh"

echo "=== panes tests ==="

export ZELLIJ_SESSION_NAME="test-session"

MOCK_PANES='[
  {"id":0,"is_plugin":false,"is_focused":false,"is_floating":false,"is_suppressed":false,"title":"openclaw","is_selectable":true,"tab_id":0,"tab_position":0,"tab_name":"main","pane_command":"openclaw","pane_cwd":"/home/user"},
  {"id":7,"is_plugin":false,"is_focused":true,"is_floating":false,"is_suppressed":false,"title":"","is_selectable":true,"tab_id":0,"tab_position":0,"tab_name":"main","pane_command":"/bin/zsh","pane_cwd":"/home/user/project"},
  {"id":1,"is_plugin":true,"is_focused":false,"is_floating":false,"is_suppressed":false,"title":"zjstatus","is_selectable":false,"tab_id":0,"tab_position":0,"tab_name":"main"},
  {"id":2,"is_plugin":false,"is_focused":true,"is_floating":false,"is_suppressed":false,"title":"editor","is_selectable":true,"tab_id":1,"tab_position":1,"tab_name":"code","pane_command":"nvim","pane_cwd":"/home/user/code"}
]'

mock_zellij_command "list-panes" "$MOCK_PANES"
mock_zellij_command "query-tab-names" "main
code"
mock_zellij_command "dump-screen" '$ git status
On branch main
nothing to commit'
mock_zellij_command "write-chars" ""
mock_zellij_command "send-keys" ""
mock_zellij_command "focus-pane-id" ""
mock_zellij_command "new-pane" "terminal_12"
install_mock_zellij

# --- panes list ---

describe "panes list returns selectable panes only (no plugins)"
output=$("$PROJECT_ROOT/zjctl" panes list 2>&1)
count=$(echo "$output" | jq '.panes | length')
assert_eq "3" "$count"

describe "panes list pane IDs are qualified"
first_id=$(echo "$output" | jq -r '.panes[0].id')
assert_eq "terminal_0" "$first_id"

describe "panes list includes command and cwd"
cmd=$(echo "$output" | jq -r '.panes[0].command')
assert_eq "openclaw" "$cmd"

describe "panes list --tab filters by tab"
output=$("$PROJECT_ROOT/zjctl" panes list --tab 1 2>&1)
count=$(echo "$output" | jq '.panes | length')
assert_eq "1" "$count"
tab=$(echo "$output" | jq -r '.panes[0].tab_name')
assert_eq "code" "$tab"

describe "panes list --tab by name"
output=$("$PROJECT_ROOT/zjctl" panes list --tab code 2>&1)
count=$(echo "$output" | jq '.panes | length')
assert_eq "1" "$count"

# --- panes read ---

describe "panes read returns content for pane"
output=$("$PROJECT_ROOT/zjctl" panes read terminal_7 2>&1)
pane_id=$(echo "$output" | jq -r '.pane_id')
assert_eq "terminal_7" "$pane_id"
assert_contains "$(echo "$output" | jq -r '.content')" "git status"

describe "panes read accepts bare integer"
output=$("$PROJECT_ROOT/zjctl" panes read 7 2>&1)
pane_id=$(echo "$output" | jq -r '.pane_id')
assert_eq "terminal_7" "$pane_id"

describe "panes read missing arg returns exit 1"
bash -c 'export PATH="'"$MOCK_DIR"':$PATH"; export ZELLIJ_SESSION_NAME=test; "'"$PROJECT_ROOT"'/zjctl" panes read' 2>/dev/null
actual_code=$?
assert_eq "1" "$actual_code"

# --- panes write ---

describe "panes write succeeds for different pane"
export ZELLIJ_PANE_ID=7
output=$("$PROJECT_ROOT/zjctl" panes write terminal_0 "hello" 2>&1)
ok=$(echo "$output" | jq -r '.ok')
assert_eq "true" "$ok"

describe "panes write blocks self-write"
bash -c 'export PATH="'"$MOCK_DIR"':$PATH"; export ZELLIJ_SESSION_NAME=test; export ZELLIJ_PANE_ID=7; "'"$PROJECT_ROOT"'/zjctl" panes write terminal_7 "hello"' 2>/dev/null
actual_code=$?
assert_eq "3" "$actual_code"

describe "panes write with --no-guard allows self-write"
export ZELLIJ_PANE_ID=7
output=$("$PROJECT_ROOT/zjctl" --no-guard panes write terminal_7 "hello" 2>&1)
ok=$(echo "$output" | jq -r '.ok')
assert_eq "true" "$ok"

# --- panes send-keys ---

describe "panes send-keys succeeds"
export ZELLIJ_PANE_ID=7
output=$("$PROJECT_ROOT/zjctl" panes send-keys terminal_0 Enter 2>&1)
ok=$(echo "$output" | jq -r '.ok')
assert_eq "true" "$ok"

describe "panes send-keys blocks self-write"
bash -c 'export PATH="'"$MOCK_DIR"':$PATH"; export ZELLIJ_SESSION_NAME=test; export ZELLIJ_PANE_ID=7; "'"$PROJECT_ROOT"'/zjctl" panes send-keys 7 Enter' 2>/dev/null
actual_code=$?
assert_eq "3" "$actual_code"

# --- panes focus ---

describe "panes focus succeeds"
output=$("$PROJECT_ROOT/zjctl" panes focus terminal_0 2>&1)
ok=$(echo "$output" | jq -r '.ok')
assert_eq "true" "$ok"

# --- panes open ---

describe "panes open returns new pane ID"
output=$("$PROJECT_ROOT/zjctl" panes open 2>&1)
pane_id=$(echo "$output" | jq -r '.pane_id')
assert_eq "terminal_12" "$pane_id"

# --- dry-run ---

describe "panes write dry-run shows command without executing"
export ZELLIJ_PANE_ID=99
output=$("$PROJECT_ROOT/zjctl" --dry-run panes write terminal_0 "hello world" 2>&1)
dry=$(echo "$output" | jq -r '.dry_run')
assert_eq "true" "$dry"
assert_contains "$(echo "$output" | jq -r '.command | join(" ")')" "write-chars"

# --- not_in_session ---

describe "panes list outside session returns exit 4"
bash -c 'export PATH="'"$MOCK_DIR"':$PATH"; unset ZELLIJ_SESSION_NAME; "'"$PROJECT_ROOT"'/zjctl" panes list' 2>/dev/null
actual_code=$?
assert_eq "4" "$actual_code"

report
