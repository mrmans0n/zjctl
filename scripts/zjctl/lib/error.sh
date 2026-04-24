#!/usr/bin/env bash
# lib/error.sh — structured JSON error construction

# Emit a structured JSON error to stderr and exit.
# Usage: emit_error <error_code> <message> [exit_code] [extra_json_fields]
#   error_code: one of unknown_command, missing_argument, invalid_target,
#               zellij_error, self_write_blocked, not_in_session, parse_error
#   message: human-readable description
#   exit_code: defaults based on error_code (see mapping below)
#   extra_json_fields: optional jq expression to merge, e.g. '{target: "terminal_8"}'
emit_error() {
    local error_code="$1"
    local message="$2"
    local exit_code="${3:-}"
    local extra="${4:-}"

    # Default exit codes per error_code
    if [ -z "$exit_code" ]; then
        case "$error_code" in
            unknown_command|missing_argument|invalid_target) exit_code=1 ;;
            zellij_error)       exit_code=2 ;;
            self_write_blocked) exit_code=3 ;;
            not_in_session)     exit_code=4 ;;
            parse_error)        exit_code=5 ;;
            *)                  exit_code=1 ;;
        esac
    fi

    local json
    json=$(jq -n \
        --arg error "$error_code" \
        --arg message "$message" \
        --argjson exit_code "$exit_code" \
        '{error: $error, message: $message, exit_code: $exit_code}')

    # Merge extra fields if provided
    if [ -n "$extra" ]; then
        json=$(echo "$json" | jq ". + $extra")
    fi

    echo "$json" >&2
    exit "$exit_code"
}

# Emit a zellij_error with the failed command included
# Usage: emit_zellij_error <message> <command_string>
emit_zellij_error() {
    local message="$1"
    local command="$2"
    emit_error "zellij_error" "$message" 2 "{\"command\": $(echo "$command" | jq -R .)}"
}
