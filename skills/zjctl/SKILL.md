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
- rely on the legacy bash `zjctl` — the Rust implementation at `/Volumes/Ambrosio/clawd/scripts/zjctl-rs/` is canonical

## First checks, always

Before using `zjctl` in a fresh session:

1. Confirm the CLI is available and check its help:
   ```bash
   zjctl --help
   ```
2. If not on `$PATH`, use the release binary:
   ```bash
   /Volumes/Ambrosio/clawd/scripts/zjctl-rs/target/release/zjctl --help
   ```
3. If the binary is not built, build and run:
   ```bash
   cargo run --manifest-path /Volumes/Ambrosio/clawd/scripts/zjctl-rs/Cargo.toml -- --help
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

## Guardrails

- Prefer `zjctl` over raw `zellij action` when `zjctl` supports the operation.
- Always verify `--help` output in a fresh session before using commands.
- If an operation is not supported by `zjctl`, say so plainly and consider raw `zellij action` only as a last resort.
- Treat `--no-guard` as a deliberate override, not a default workaround.
- The Rust implementation at `/Volumes/Ambrosio/clawd/scripts/zjctl-rs/` is canonical. The legacy bash `zjctl` under `scripts/zjctl/` is historical and should not be relied upon.
- When updating this skill, cross-check against the source files listed in "First checks, always".
