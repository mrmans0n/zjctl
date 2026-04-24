#!/usr/bin/env bash
# lib/output.sh — output formatting (json, table, quiet)

# Global format variable, set by main script from --format flag
ZJCTL_FORMAT="${ZJCTL_FORMAT:-json}"

# Emit output in the configured format.
# Usage: emit_output <json_string> [table_header] [table_jq_fields]
#   json_string: the JSON to emit (used directly in json mode)
#   table_header: header line for table mode (e.g., "ID\tCOMMAND\tCWD")
#   table_jq_expr: jq expression to extract array-of-arrays for table rows
emit_output() {
    local json="$1"
    local table_header="${2:-}"
    local table_jq_expr="${3:-}"

    case "$ZJCTL_FORMAT" in
        json)
            echo "$json"
            ;;
        table)
            if [ -n "$table_header" ] && [ -n "$table_jq_expr" ]; then
                {
                    printf '%b\n' "$table_header"
                    echo "$json" | jq -r "$table_jq_expr | @tsv"
                } | column -t -s $'\t'
            else
                # Fallback: pretty-print the JSON
                echo "$json" | jq .
            fi
            ;;
        quiet)
            # No output; exit code carries the meaning
            ;;
        *)
            emit_error "unknown_command" "Unknown format: $ZJCTL_FORMAT"
            ;;
    esac
}

# Emit a simple success/action result (for write/send-keys/focus commands)
emit_action_result() {
    local json="$1"
    emit_output "$json" "" ""
}

# Emit a dry-run result instead of executing
# Usage: emit_dry_run <command_array_json>
emit_dry_run() {
    local cmd_json="$1"
    local json
    json=$(jq -n --argjson cmd "$cmd_json" '{dry_run: true, command: $cmd}')
    emit_output "$json" "DRY_RUN\tCOMMAND" '[.dry_run, (.command | join(" "))]'
}
