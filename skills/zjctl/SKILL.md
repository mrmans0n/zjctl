---
name: zjctl
description: "Use the shipped Rust zjctl CLI to inspect and safely control Zellij sessions, tabs, and panes from agents."
---

# zjctl

Use the shipped Rust `zjctl` CLI to inspect and control Zellij sessions, tabs, and panes safely from agents. Prefer `zjctl` over raw `zellij action` commands when the operation is supported.

## Scope

Use this skill for:
- listing Zellij sessions
- listing, focusing, or opening tabs
- listing, reading, writing, focusing, or opening panes
- sending keys to panes

Do not use this skill to:
- invent unsupported CLI flags — check `--help` first
- bypass safety guards casually — `--no-guard` requires intentional use
- use raw `zellij action` when `zjctl` supports the operation
- assume another implementation exists — the Rust implementation in this repository is canonical

## First checks, always

Before using `zjctl` in a fresh session:

1. Confirm the CLI is available and check its help:
   ```bash
   zjctl --help
   ```
2. If not on `$PATH`, install it from the [zjctl repository](https://github.com/mrmans0n/zjctl):
   ```bash
   cargo install --git https://github.com/mrmans0n/zjctl
   ```
4. Do not assume old flags or commands still exist — verify against `--help` output.

If updating this skill, verify against source files:
- `src/cli.rs` — command and flag surface
- `src/models.rs` — JSON response structs
- `src/commands/` — command behavior and dry-run mapping
- `src/identity.rs` — tab and pane identity resolution
- `src/safety.rs` — self-write guard behavior
- `src/error.rs` — error names, fields, and exit codes

## Global flags

| Flag | Default | Description |
|------|---------|-------------|
| `-f, --format <json\|table\|quiet>` | `json` | Output format |
| `-n, --dry-run` | off | Show command without executing |
| `-q, --quiet` | off | Suppress non-error output |
| `-s, --session <SESSION>` | — | Target session (overrides `ZELLIJ_SESSION_NAME`) |
| `--no-guard` | off | Bypass self-write safety guard |

## Session resolution

Commands under `tabs` and `panes` require a session. Resolution order:
1. `--session <name>` flag
2. `ZELLIJ_SESSION_NAME` environment variable
3. Error: `not_in_session`, exit code `4`

`sessions list` does not require a session.

## Commands

### Sessions

```bash
zjctl sessions list
```

### Tabs

```bash
zjctl tabs list
zjctl tabs focus <index-or-name>
zjctl tabs open [--name NAME] [--layout LAYOUT] [-- COMMAND...]
```

### Panes

```bash
zjctl panes list [--tab <index-or-name>]
zjctl panes read <pane> [--full] [--ansi]
zjctl panes write <pane> <text>
zjctl panes send-keys <pane> <keys...>
zjctl panes focus <pane>
zjctl panes open [--direction DIR] [--floating] [--name NAME] [--cwd CWD] [--tab-id ID] [-- COMMAND...]
```

## Identity resolution

**Tabs** accept:
- Numeric 0-based index (e.g. `0`, `2`)
- Exact tab name from the session (e.g. `"main"`)

**Panes** accept:
- Qualified ID: `terminal_N` or `plugin_N`
- Bare integer `N` — normalized to `terminal_N`
- Exact pane title (must be unambiguous among non-plugin panes)

Ambiguous or missing identifiers return `invalid_target`, exit code `1`.

## JSON contracts

### Success responses

Sessions list:
```json
{"sessions":[{"name":"main"}]}
```

Tabs list:
```json
{"tabs":[{"index":0,"name":"main","active":true,"tab_id":0}]}
```

Panes list (note: `command`, `cwd`, `title` can be `null`):
```json
{"panes":[{"id":"terminal_0","command":"zsh","cwd":"/repo","title":"main","focused":true,"floating":false,"tab_id":0,"tab_name":"main"}]}
```

Pane read:
```json
{"pane_id":"terminal_7","content":"$ echo hello\nhello\n$"}
```

Successful mutation (`write`, `send-keys`, `focus`):
```json
{"ok":true}
```

Tab open:
```json
{"tab_id":1}
```

Pane open:
```json
{"pane_id":"terminal_12"}
```

Dry run (any mutation):
```json
{"dry_run":true,"command":["zellij","action","write-chars","--pane-id","terminal_7","hello"]}
```

### Error responses

Errors are emitted as JSON on stderr:
```json
{"error":"invalid_target","message":"Pane not found: foo","exit_code":1}
```

Known errors:

| Error | Exit | Extra fields | When |
|-------|------|--------------|------|
| `unknown_command` | 1 | — | Unrecognized command |
| `missing_argument` | 1 | — | Required argument missing |
| `invalid_target` | 1 | — | Tab/pane identifier not found or ambiguous |
| `zellij_error` | 2 | `command` | Zellij subprocess failed |
| `self_write_blocked` | 3 | `target`, `self` | Write/send-keys to own pane blocked |
| `not_in_session` | 4 | — | No session resolved |
| `parse_error` | 5 | — | Failed to parse zellij output |

## Safety guards

`panes write` and `panes send-keys` include self-write protection:

1. The target pane is resolved.
2. The resolved pane ID is compared against `ZELLIJ_PANE_ID`.
3. If they match, the command fails with `self_write_blocked` (exit `3`).
4. The guard runs **even during `--dry-run`** — dry-run output is only emitted after the guard passes.
5. `--no-guard` bypasses the protection. Use only when the agent has verified the target is intentional or the user explicitly requests it.
6. If `ZELLIJ_PANE_ID` is not set, the guard is skipped (no self to protect).

Self-write block example:
```json
{"error":"self_write_blocked","message":"Refusing to write to own pane (terminal_8). Use --no-guard to override.","exit_code":3,"target":"terminal_8","self":"terminal_8"}
```

## Usage patterns

### Find a pane by title, run a command, read output

```bash
# 1. Find the target pane
zjctl panes list | jq '.panes[] | select(.title == "build-runner")'
# → {"id":"terminal_5", ...}

# 2. Write a command to it
zjctl panes write terminal_5 "cargo test --quiet"

# 3. Send Enter to execute
zjctl panes send-keys terminal_5 Enter

# 4. Wait, then read output
sleep 3
zjctl panes read terminal_5
# → {"pane_id":"terminal_5","content":"...test results..."}
```

### Focus a tab by name, then list its panes

```bash
# 1. List tabs to confirm the name exists
zjctl tabs list
# → {"tabs":[{"index":0,"name":"editor","active":true,...},{"index":1,"name":"tests",...}]}

# 2. Focus the target tab
zjctl tabs focus tests
# → {"ok":true}

# 3. List panes in that tab
zjctl panes list --tab tests
# → {"panes":[{"id":"terminal_3",...},{"id":"terminal_4",...}]}
```

### Open a pane, capture its ID, run a command

```bash
# 1. Open a floating pane with a working directory
zjctl panes open --floating --name "scratch" --cwd /tmp
# → {"pane_id":"terminal_12"}

# 2. Write a command to the new pane using the returned ID
zjctl panes write terminal_12 "ls -la"
zjctl panes send-keys terminal_12 Enter
```

### Dry-run before writing to a non-current pane

```bash
# 1. Preview what zjctl would execute
zjctl --dry-run panes write terminal_7 "rm -rf build/"
# → {"dry_run":true,"command":["zellij","action","write-chars","--pane-id","terminal_7","rm -rf build/"]}

# 2. If the preview looks correct, run for real
zjctl panes write terminal_7 "rm -rf build/"
# → {"ok":true}
```

### Handle invalid_target by relisting

```bash
# 1. Attempt to read a pane that may have been closed
zjctl panes read terminal_99
# stderr: {"error":"invalid_target","message":"Pane not found: terminal_99","exit_code":1}

# 2. Relist panes to find the correct target — don't retry blindly
zjctl panes list
# → {"panes":[{"id":"terminal_0",...},{"id":"terminal_3",...}]}

# 3. Use the correct ID
zjctl panes read terminal_3
```

## Guardrails

- Prefer `zjctl` over raw `zellij action` when `zjctl` supports the operation.
- Always verify `--help` output in a fresh session before using commands.
- If an operation is not supported by `zjctl`, say so plainly and consider raw `zellij action` only as a last resort.
- Treat `--no-guard` as a deliberate override, not a default workaround.
- The Rust implementation in this repository is canonical.
- When updating this skill, cross-check against the source files listed in "First checks, always".
