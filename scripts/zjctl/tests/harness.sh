#!/usr/bin/env bash
# tests/harness.sh — minimal test framework for zjctl

TESTS_RUN=0
TESTS_PASSED=0
TESTS_FAILED=0
CURRENT_TEST=""

# Directory of this harness file
HARNESS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$HARNESS_DIR/.." && pwd)"

# Create a temp dir for mocks; cleaned up on exit
MOCK_DIR="$(mktemp -d)"
trap 'rm -rf "$MOCK_DIR"' EXIT

# Put mock dir first in PATH so mock binaries shadow real ones
export PATH="$MOCK_DIR:$PATH"

# Source library modules
source "$PROJECT_ROOT/lib/error.sh"

describe() {
    CURRENT_TEST="$1"
    TESTS_RUN=$((TESTS_RUN + 1))
}

assert_eq() {
    local expected="$1"
    local actual="$2"
    local msg="${3:-$CURRENT_TEST}"
    if [ "$expected" = "$actual" ]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        printf "  PASS: %s\n" "$msg"
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        printf "  FAIL: %s\n" "$msg"
        printf "    expected: %s\n" "$expected"
        printf "    actual:   %s\n" "$actual"
    fi
}

assert_contains() {
    local haystack="$1"
    local needle="$2"
    local msg="${3:-$CURRENT_TEST}"
    if echo "$haystack" | grep -qF "$needle"; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        printf "  PASS: %s\n" "$msg"
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        printf "  FAIL: %s\n" "$msg"
        printf "    expected to contain: %s\n" "$needle"
        printf "    actual: %s\n" "$haystack"
    fi
}

assert_exit() {
    local expected_code="$1"
    shift
    "$@" >/dev/null 2>/dev/null
    local actual_code=$?
    assert_eq "$expected_code" "$actual_code" "$CURRENT_TEST (exit code)"
}

# Create a mock zellij binary that reads from fixture files
# Usage: mock_zellij_command "list-sessions" "output text"
mock_zellij_command() {
    local subcmd="$1"
    local output="$2"
    local exit_code="${3:-0}"
    # Store fixture data keyed by subcommand
    printf '%s' "$output" > "$MOCK_DIR/zellij_fixture_${subcmd}"
    printf '%s' "$exit_code" > "$MOCK_DIR/zellij_exit_${subcmd}"
}

# Write the mock zellij script (call once after setting up fixtures)
install_mock_zellij() {
    cat > "$MOCK_DIR/zellij" << 'MOCK_EOF'
#!/usr/bin/env bash
MOCK_DIR="$(cd "$(dirname "$0")" && pwd)"
# Flatten args to find the subcommand
# Handle: zellij list-sessions, zellij action list-panes, etc.
args="$*"
# Try progressively shorter suffixes to match
# e.g., "action list-panes --json --all" -> try "action-list-panes---json---all", then shorter
# Simplify: join all args with dash, look for fixture
key=$(echo "$args" | tr ' ' '-')
if [ -f "$MOCK_DIR/zellij_fixture_${key}" ]; then
    cat "$MOCK_DIR/zellij_fixture_${key}"
    exit "$(cat "$MOCK_DIR/zellij_exit_${key}" 2>/dev/null || echo 0)"
fi
# Try just the subcommand (skip "action" prefix)
for arg in "$@"; do
    if [ -f "$MOCK_DIR/zellij_fixture_${arg}" ]; then
        cat "$MOCK_DIR/zellij_fixture_${arg}"
        exit "$(cat "$MOCK_DIR/zellij_exit_${arg}" 2>/dev/null || echo 0)"
    fi
done
echo "mock zellij: unhandled command: $*" >&2
exit 99
MOCK_EOF
    chmod +x "$MOCK_DIR/zellij"
}

report() {
    echo ""
    echo "Results: $TESTS_PASSED/$TESTS_RUN passed, $TESTS_FAILED failed"
    [ "$TESTS_FAILED" -eq 0 ] && exit 0 || exit 1
}
