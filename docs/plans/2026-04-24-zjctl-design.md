# zjctl Design Document

**Version:** 0.1.0 (MVP design)  
**Date:** 2026-04-24  
**Status:** Draft — for review before implementation  

---

## 1. Problem Statement & Goals

### Problem
AI agents running inside one Zellij pane are blind to the rest of the session. They cannot:
- discover what other panes/tabs exist,
- read the output of a running build or test,
- send input to another pane safely,
- coordinate actions across multiple panes without human hand-holding.

Zellij's native CLI exposes many actions, but the interface is human-oriented and inconsistent for programmatic consumption.

### Goals
1. **Stable JSON interface** over Zellij's native CLI so agents can inspect and control the session reliably.
2. **Thin wrapper first**: leverage `zellij action` primitives before building custom plugins.
3. **Safety by default**: guardrails that prevent an agent from accidentally destroying a user's session.
4. **Human friendly**: output modes for both machines (JSON/JSONL) and humans (tables/text).
5. **No MCP dependency**: works anywhere Zellij runs, with just a local binary.

### Non-Goals
- Replace Zellij's own CLI for human users.
- Provide a full terminal multiplexer abstraction (tmux compatibility, etc.).
- Real-time pixel-perfect screen scraping.
- Session persistence or remote orchestration across hosts.
- A custom Zellij plugin in the MVP (defer until native CLI is insufficient).

---

## 2. User Personas & Use Cases

### Persona A: AI Agent Operator
- An AI assistant (Claude, Codex, etc.) running inside a Zellij pane.
- Needs to run tests in Pane 2, watch a server in Pane 3, edit files in Pane 4.
- Consumes JSON output and drives `zjctl` via shell calls.

### Persona B: Human Power User
- Writes shell scripts or dev workflows that manipulate Zellij sessions.
- Wants tabulated lists, clear errors, and predictable exit codes.

### Key Use Cases
| # | Use Case | Priority |
|---|----------|----------|
| 1 | List sessions/tabs/panes as structured JSON | MVP |
| 2 | Read the current (or full scrollback) output of a pane | MVP |
| 3 | Write text / send keys to a pane | MVP |
| 4 | Focus a pane or tab without disrupting the user | MVP |
| 5 | Open a new tab or pane with a specific command | MVP |
| 6 | Watch / tail pane output continuously | v1 |
| 7 | Execute a command in a pane and capture its exit status | v1 |
| 8 | Lock a pane/tab to prevent conflicting agent actions | v1 |
| 9 | Close or kill panes/tabs with confirmation guards | v1 |
| 10 | Layout-aware operations (apply named layouts) | Future |

---

## 3. Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│  zjctl CLI (Rust)                                          │
│  ├── Parser (clap derive)                                   │
│  ├── Command Router                                         │
│  ├── Zellij Native Adapter (shells out to `zellij action`) │
│  ├── Output Formatter (JSON / JSONL / Human)               │
│  └── Guardrails / Safety Layer                             │
└─────────────────────────────────────────────────────────────┘
                             │
                             ▼
                    ┌────────────────┐
                    │  zellij binary  │
                    │  (system CLI)   │
                    └────────────────┘
```

### Design Principles
1. **Adapter, not re-implementation**: `zjctl` translates agent-friendly commands into `zellij action` calls.
2. **JSON is the primary contract**: every command that produces data must have a `--json` mode.
3. **Fail fast, explain clearly**: structured errors, distinct exit codes, human-readable messages.
4. **Session-scoped by default**: target the current Zellij session unless `--session` is given.

---

## 4. Command Taxonomy

### Global Flags
```
--session <name>   Target a specific Zellij session (default: current)
--json             Force JSON output even for human-oriented commands
--jsonl            Use JSON Lines for streaming/list output
--quiet            Suppress non-error output
--dry-run          Print the zellij commands that would run, but do not execute
```

### 4.1 Discovery (`list`)

#### `zjctl sessions list`
List all Zellij sessions.

**Example:**
```bash
$ zjctl sessions list --json
```

**JSON output:**
```json
{
  "sessions": [
    {
      "name": "dev",
      "tabs": 3,
      "panes": 7,
      "attached": true,
      "created_at": "2026-04-24T18:00:00Z"
    }
  ]
}
```

#### `zjctl tabs list [--session <name>]`
List tabs in the target session.

**JSON output:**
```json
{
  "session": "dev",
  "tabs": [
    {
      "id": 1,
      "name": "editor",
      "active": true,
      "panes": 2,
      "layout": "default"
    }
  ]
}
```

#### `zjctl panes list [--tab <id|name>] [--session <name>]`
List panes. Optionally filtered by tab.

**JSON output:**
```json
{
  "session": "dev",
  "tab": 1,
  "panes": [
    {
      "id": "terminal_1",
      "type": "terminal",
      "title": "nvim",
      "cwd": "/home/user/project",
      "command": "nvim src/main.rs",
      "is_active": true,
      "is_floating": false,
      "size": { "rows": 40, "cols": 120 }
    }
  ]
}
```

### 4.2 Read (`read`)

#### `zjctl read --pane <id> [--full] [--ansi] [--lines <n>]`
Read pane output. Without `--full`, returns only the visible viewport.

**Example:**
```bash
$ zjctl read --pane terminal_2 --lines 50 --json
```

**JSON output:**
```json
{
  "pane": "terminal_2",
  "lines": 50,
  "content": "Line 1\nLine 2\n...",
  "truncated": false,
  "ansi": false
}
```

### 4.3 Write (`write`)

#### `zjctl write --pane <id> --text <text> [--enter] [--no-newline]`
Paste text into a pane. `--enter` sends a Return keystroke after the text.

**Example:**
```bash
$ zjctl write --pane terminal_3 --text "cargo test" --enter
```

**JSON output (with `--json`):**
```json
{
  "pane": "terminal_3",
  "action": "write",
  "bytes_sent": 10,
  "enter": true
}
```

#### `zjctl send-keys --pane <id> <key>...`
Send raw key sequences (Ctrl, Alt, special keys).

**Example:**
```bash
$ zjctl send-keys --pane terminal_1 Ctrl+c
```

### 4.4 Focus (`focus`)

#### `zjctl focus --pane <id>`
Focus a specific pane without switching tabs.

#### `zjctl focus --tab <id|name>`
Focus a tab.

### 4.5 Open (`open`)

#### `zjctl open tab --name <name> [--layout <layout>] [--cwd <path>]`
Create a new tab.

#### `zjctl open pane --command <cmd> [--cwd <path>] [--floating] [--name <name>]`
Open a new pane running a command.

### 4.6 Close (`close`)

#### `zjctl close pane --pane <id> [--force]`
Close a pane. Requires `--force` if the pane is running a process (safety guard).

#### `zjctl close tab --tab <id|name> [--force]`
Close a tab.

### 4.7 Execute (`exec`) — v1

#### `zjctl exec --pane <id> --command <cmd> [--timeout <secs>] [--json]`
Run a command in a pane and wait for it to finish, capturing exit status.

**Semantics:**
1. Write the command to the pane with `--enter`.
2. Poll pane output for a sentinel (e.g., prompt return) or use a timeout.
3. Return exit code and final output.

> **Note:** True exit-code capture requires either a cooperating shell prompt or a Zellij plugin. In v1, we document the limitation and provide best-effort heuristics.

**JSON output:**
```json
{
  "pane": "terminal_4",
  "command": "cargo build",
  "exit_code": 0,
  "output": "Compiling...\nFinished dev [unoptimized]",
  "duration_ms": 12400
}
```

### 4.8 Watch (`watch`) — v1

#### `zjctl watch --pane <id> [--lines <n>] [--interval <ms>] [--jsonl]`
Stream pane output changes as JSON Lines.

> **Limitation:** Zellij native CLI does not support streaming. v1 implementation will poll `dump-screen` and emit deltas. A future plugin can provide push-based updates.

**JSONL output:**
```jsonl
{"timestamp":"2026-04-24T18:01:00Z","pane":"terminal_2","lines":["new line 1","new line 2"]}
{"timestamp":"2026-04-24T18:01:05Z","pane":"terminal_2","lines":["new line 3"]}
```

### 4.9 Lock (`lock`) — v1

#### `zjctl lock pane --pane <id> --reason <msg> [--ttl <secs>]`
Mark a pane as locked by the agent. Prevents other `zjctl` processes from writing to it.

#### `zjctl lock release --pane <id>`
Release a lock.

#### `zjctl lock status [--pane <id>]`
Show active locks.

> **Implementation:** File-based lock in `/tmp/zjctl-locks/` with TTL and process PID. Not cryptographically secure, but sufficient for local single-user coordination.

---

## 5. Identity Model

### Pane References
Zellij uses `terminal_2`, `plugin_1`, etc. `zjctl` accepts:
- Full ID: `terminal_2`
- Numeric shorthand: `2` (resolved against terminal panes)
- Title match: `name:editor` (resolved by fuzzy title match, error if ambiguous)

**Resolution rules:**
1. If exact full ID match → use it.
2. If numeric and unique among terminals → `terminal_<n>`.
3. If `name:<text>` and exactly one match → use it.
4. Otherwise → error with list of candidates.

### Tab References
- Numeric ID: `1`, `2`
- Name match: `editor` (exact) or `name:ed` (fuzzy)

### Session References
- Always by exact name. No fuzzy matching (too risky).

---

## 6. Read / Watch / Write / Exec Semantics

### Read
- **Viewport read** (default): returns visible screen content. Fast, no scrollback.
- **Full read** (`--full`): returns entire scrollback buffer. Can be large; respect `--lines` to cap.
- **ANSI** (`--ansi`): include escape codes when the user asks for them; default stripped.

### Write
- **Paste** (`zjctl write`): simulates typing. Subject to shell interpretation in the target pane.
- **Send-keys** (`zjctl send-keys`): literal key events. Use for control sequences.
- **Safety**: never write to the current pane (the one running `zjctl`) unless `--force` is passed.

### Watch
- **Polling strategy** (v1): diff consecutive `dump-screen` outputs.
- **Deduplication**: suppress unchanged lines.
- **Backpressure**: if the consumer is slow, drop intermediate frames and emit a catch-up event.

### Exec
- **Fire-and-write**: send command, return immediately.
- **Wait-for-completion** (`--wait`): poll for shell prompt return. Best-effort; requires known prompt or sentinel.
- **Timeout**: default 30s, configurable.

---

## 7. Safety Model & Guardrails

### Default Protections
| Guard | Behavior |
|-------|----------|
| No self-write | Refuse to `write` to the pane running `zjctl` unless `--force` |
| No close active | Refuse to `close` the active pane/tab unless `--force` |
| Confirm close running | `close` a pane with a running process requires `--force` |
| Dry-run available | `--dry-run` prints commands without executing |
| JSON by default for scripts | `--json` is implied when stdout is not a TTY (overridable) |

### Session Boundaries
- `zjctl` only touches the current Zellij session unless `--session` is explicit.
- Cross-session operations require explicit `--session` for every command.

### What `zjctl` Cannot Protect Against
- Writing dangerous commands to another pane (the shell in that pane will execute them).
- Race conditions between multiple agents (mitigated by locks in v1).
- Zellij crashes or session detachment.

---

## 8. Locking & Concurrency Model

### v1: Cooperative File Locks
- Lock directory: `/tmp/zjctl-locks/<session>/<pane>.lock`
- Lock file contains JSON: `{ "pid": 1234, "since": "ISO8601", "ttl": 300, "reason": "..." }`
- TTL default: 5 minutes. Expired locks are ignored and cleaned up.
- `zjctl lock` creates the file; `zjctl lock release` removes it.
- Write/close/exec on a locked pane fail unless `--force` or the lock is owned by the same PID.

### Future: Zellij Plugin
- A plugin can hold authoritative state inside the Zellij server, avoiding stale locks when `zjctl` crashes.
- Defer until file-lock pain is demonstrated.

---

## 9. Error Model & Exit Codes

| Exit Code | Meaning |
|-----------|---------|
| 0 | Success |
| 1 | General error (I/O, unexpected) |
| 2 | Invalid arguments / usage |
| 3 | Zellij not found or not running |
| 4 | Target session/tab/pane not found |
| 5 | Target is locked by another process |
| 6 | Zellij command failed (non-zero exit) |
| 7 | Timeout (exec/watch) |
| 8 | Safety guard triggered (needs `--force`) |

### Error JSON
When `--json` is active, errors are emitted as:
```json
{
  "error": {
    "code": 5,
    "message": "Pane terminal_2 is locked by PID 1234 since 2026-04-24T18:00:00Z",
    "target": "terminal_2",
    "hint": "Use 'zjctl lock release --pane terminal_2' or wait for TTL expiry"
  }
}
```

---

## 10. Output Modes

### JSON (default for non-TTY)
Structured, stable schema. All list/read/write/exec commands support it.

### JSONL (`--jsonl`)
One JSON object per line. Used for `watch` and streaming list operations.

### Human (default for TTY)
- Tables for lists (auto-width, truncated if narrow).
- Plain text for `read`.
- One-line confirmations for `write`/`close`.

### Switching
- `--json` forces JSON.
- `--jsonl` forces JSONL.
- `--human` forces human format.
- Auto-detect: JSON if stdout is not a TTY, unless overridden.

---

## 11. Observability & Debug Strategy

### Logging
- `RUST_LOG=zjctl=debug` uses `env_logger`.
- Debug logs include the exact `zellij action` commands being spawned and their exit codes.
- Never log pane content at `info` or higher (privacy).

### Dry Run
- `--dry-run` prints the native Zellij command array as JSON:
```json
{
  "dry_run": true,
  "command": ["zellij", "action", "dump-screen", "--pane-id", "terminal_2"]
}
```

### Tracing
- Future: add `--trace` to emit a structured trace of all operations (timings, targets, outcomes).

---

## 12. Phased Roadmap

### MVP (0.1.x) — Now
- [ ] `sessions list`
- [ ] `tabs list`
- [ ] `panes list`
- [ ] `read --pane <id> [--full] [--ansi] [--lines N]`
- [ ] `write --pane <id> --text <text> [--enter]`
- [ ] `send-keys --pane <id> <keys>...`
- [ ] `focus --pane <id>` / `focus --tab <id>`
- [ ] `open tab --name <name>`
- [ ] `open pane --command <cmd>`
- [ ] Global flags: `--session`, `--json`, `--quiet`, `--dry-run`
- [ ] Error model with structured JSON errors
- [ ] Identity resolution (full ID, numeric, name)
- [ ] Safety guards (no self-write, no close active)
- [ ] Human + JSON output modes

### v1 (0.2.x) — Next
- [ ] `watch --pane <id> [--jsonl]` (polling-based)
- [ ] `exec --pane <id> --command <cmd> [--wait] [--timeout]`
- [ ] `close pane --pane <id> [--force]` / `close tab --tab <id> [--force]`
- [ ] `lock pane` / `lock release` / `lock status` (file-based)
- [ ] `--jsonl` for list commands
- [ ] `--human` override
- [ ] Exit-code capture heuristics for exec

### Future (0.3.x+)
- [ ] Custom Zellij plugin for push-based watch and authoritative locks
- [ ] Layout operations (`zjctl layout apply <name>`)
- [ ] Session creation / attachment
- [ ] Configuration file support (`~/.config/zjctl/config.toml`)
- [ ] Plugin API for third-party extensions

---

## 13. Alternatives Considered

| Alternative | Pros | Cons | Decision |
|-------------|------|------|----------|
| **Zellij plugin (now)** | Push events, fast watches, no polling | More complex build, requires loading into Zellij, harder to iterate | **Defer** |
| **Zellij native CLI only** | Zero extra code | No stable JSON contracts, no safety guards, no identity resolution | **Insufficient** |
| **MCP server** | Rich protocol, IDE integration | Requires MCP client, extra setup, overkill for local use | **Out of scope** |
| **Tmux equivalent (`tmuxctl`)** | Broader user base | User chose Zellij; tmux is different architecture | **Not doing** |
| **Daemon / long-running process** | Holds state, fast | More complexity, crash recovery, lifecycle mgmt | **Defer** |

---

## 14. Trade-offs

1. **Polling vs. Plugin for watch**: Polling is simpler but higher CPU and latency. Accept for v1; plugin when pain is real.
2. **File locks vs. Plugin locks**: File locks are brittle (PID reuse, stale locks). Accept because they are trivial and good enough for single-user local use.
3. **Shell-out vs. Zellij Rust API**: Zellij does not expose a stable Rust crate API. Shell-out is the only supported integration.
4. **Heuristic exec vs. True exec**: True process exec requires PTY control we don't have. Heuristic exec is the pragmatic boundary.

---

## 15. TODO (for implementer)

1. Add `Sessions`, `Tabs`, `Panes`, `Read`, `Write`, `SendKeys`, `Focus`, `Open` subcommands to `main.rs`.
2. Implement identity resolution module (`resolve_pane`, `resolve_tab`).
3. Add `OutputMode` enum and formatter module.
4. Add structured error type implementing `serde::Serialize`.
5. Add guardrail checks before destructive operations.
6. Write integration tests that mock `zellij` binary calls.
7. Document JSON schemas in `docs/schemas/`.

---

## Summary & Recommendation

**Build first:**
1. The discovery surface (`sessions list`, `tabs list`, `panes list`) — this unlocks everything else.
2. `read` and `write` with identity resolution and safety guards — the core agent loop.
3. `focus` and `open` — basic session manipulation.
4. Structured errors and JSON output — contract stability.

**Biggest risks:**
1. Zellij native CLI output schema changes between versions. Mitigate: parse defensively, validate with JSON Schema, pin tested versions.
2. Shell-out latency for rapid agent loops. Mitigate: batch operations, cache list results briefly.
3. Self-write accidents. Mitigate: the "no self-write" guard is mandatory and must be well-tested.

**Bottom line:** A thin, well-structured wrapper around `zellij action` is the right MVP. It delivers immediate value, stays maintainable, and leaves a clear path to a plugin when native CLI limits become painful.
