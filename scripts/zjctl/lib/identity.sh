#!/usr/bin/env bash
# lib/identity.sh — tab and pane name-to-ID resolution

# Resolve a tab identifier to a 0-based index.
# Input: numeric index, or tab name (resolved via query-tab-names)
# Output: prints the 0-based index to stdout
resolve_tab() {
    local input="$1"

    # Numeric: pass through directly
    case "$input" in
        *[!0-9]*) ;; # contains non-digits, treat as name
        *) echo "$input"; return 0 ;;
    esac

    # Name-based: query tab names and find the matching index
    local tab_names
    tab_names=$(zellij action query-tab-names 2>/dev/null) || {
        emit_error "zellij_error" "Failed to query tab names" 2
    }

    local index=0
    while IFS= read -r name; do
        if [ "$name" = "$input" ]; then
            echo "$index"
            return 0
        fi
        index=$((index + 1))
    done <<< "$tab_names"

    emit_error "invalid_target" "Tab not found: $input"
}

# Resolve a pane identifier to a qualified pane ID (terminal_N or plugin_N).
# Input: "terminal_N", "plugin_N", bare integer N, or a pane title/name
# Output: prints the qualified ID to stdout
resolve_pane() {
    local input="$1"

    # Already qualified
    case "$input" in
        terminal_*|plugin_*) echo "$input"; return 0 ;;
    esac

    # Bare integer -> terminal_N
    case "$input" in
        *[!0-9]*) ;; # contains non-digits, treat as name
        *) echo "terminal_${input}"; return 0 ;;
    esac

    # Name/title-based: query list-panes and match by title
    local panes_json
    panes_json=$(zellij action list-panes --json --all 2>/dev/null) || {
        emit_error "zellij_error" "Failed to list panes for name resolution" 2
    }

    local pane_id
    pane_id=$(echo "$panes_json" | jq -r --arg name "$input" '
        [.[] | select(.is_plugin == false and .title == $name)] |
        if length == 0 then "NOT_FOUND"
        elif length == 1 then .[0] | "terminal_\(.id)"
        else "AMBIGUOUS"
        end
    ')

    case "$pane_id" in
        NOT_FOUND)
            emit_error "invalid_target" "Pane not found: $input"
            ;;
        AMBIGUOUS)
            emit_error "invalid_target" "Ambiguous pane name: $input (matches multiple panes)"
            ;;
        *)
            echo "$pane_id"
            return 0
            ;;
    esac
}
