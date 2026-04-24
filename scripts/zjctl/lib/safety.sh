#!/usr/bin/env bash
# lib/safety.sh — no-self-write guard and dry-run helpers

# Check if writing to the target pane would be a self-write.
# Exits with code 3 if the guard triggers.
# Usage: check_self_write <resolved_pane_id>
check_self_write() {
    local target="$1"

    # Guard disabled
    if [ "${ZJCTL_NO_GUARD:-}" = "true" ]; then
        return 0
    fi

    # Can't check if ZELLIJ_PANE_ID isn't set
    if [ -z "${ZELLIJ_PANE_ID:-}" ]; then
        return 0
    fi

    local self_id="terminal_${ZELLIJ_PANE_ID}"

    # Normalize target: bare integer -> terminal_N
    case "$target" in
        terminal_*|plugin_*) ;; # already qualified
        *[!0-9]*) return 0 ;; # not numeric, can't match
        *) target="terminal_${target}" ;; # bare integer
    esac

    if [ "$target" = "$self_id" ]; then
        emit_error "self_write_blocked" \
            "Refusing to write to own pane ($self_id). Use --no-guard to override." \
            3 \
            "{\"target\": \"$self_id\", \"self\": \"$self_id\"}"
    fi
}

# Run a zellij action command, or emit dry-run output.
# Usage: run_zellij_action [args...]
run_zellij_action() {
    local cmd=("zellij" "action" "$@")

    if [ "${ZJCTL_DRY_RUN:-}" = "true" ]; then
        local cmd_json
        cmd_json=$(printf '%s\n' "${cmd[@]}" | jq -R . | jq -s .)
        emit_dry_run "$cmd_json"
        return 0
    fi

    local output
    local exit_code=0
    output=$("${cmd[@]}" 2>&1) || exit_code=$?

    if [ "$exit_code" -ne 0 ]; then
        emit_zellij_error "$output" "${cmd[*]}"
    fi

    echo "$output"
}

# Run a top-level zellij command (not action), e.g., list-sessions
# Usage: run_zellij [args...]
run_zellij() {
    local cmd=("zellij" "$@")

    if [ "${ZJCTL_DRY_RUN:-}" = "true" ]; then
        local cmd_json
        cmd_json=$(printf '%s\n' "${cmd[@]}" | jq -R . | jq -s .)
        emit_dry_run "$cmd_json"
        return 0
    fi

    local output
    local exit_code=0
    output=$("${cmd[@]}" 2>&1) || exit_code=$?

    if [ "$exit_code" -ne 0 ]; then
        emit_zellij_error "$output" "${cmd[*]}"
    fi

    echo "$output"
}
