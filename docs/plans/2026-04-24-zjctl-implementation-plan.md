# zjctl Implementation Plan

**Version:** 0.1.0 MVP  
**Date:** 2026-04-24  
**Based on:** [2026-04-24-zjctl-design.md](2026-04-24-zjctl-design.md)

---

## 1. Goals and Scope

### Goals
- Complete the MVP command surface: discovery, read, write, focus, open, send-keys.
- Introduce stable JSON output and structured errors as the primary contract.
- Add identity resolution (pane/tab references) so agents do not hard-code Zellij IDs.
- Add safety guardrails (no self-write, dry-run) so agents cannot accidentally damage the session.
- Refactor the flat `main.rs` into a maintainable module tree without over-engineering.

### MVP Scope (in)
- `sessions list`, `tabs list`, `panes list`
- `read --pane <id> [--full] [--ansi] [--lines N]`
- `write --pane <id> --text <text> [--enter]`
- `send-keys --pane <id> <keys>...`
- `focus --pane <id>` / `focus --tab <id|name>`
- `open tab --name <name>`
- `open pane --command <cmd> [--cwd <path>]`
- Global flags: `--session`, `--json`, `--quiet`, `--dry-run`
- Identity resolution (full ID, numeric shorthand, name match)
- Structured JSON errors + exit codes
- Human-readable tables for TTY, JSON for non-TTY

### Deferred (v1 / later)
- `watch`, `exec`, `close`, `lock` — see design doc §12.
- `--jsonl` output mode.
- Layout operations.
- Custom Zellij plugin.
- Config file support.

---

## 2. Target Module/File Structure

```
src/
├── main.rs              # CLI parser (clap), command routing, exit codes
├── cli.rs               # Clap derive structs (Commands, GlobalFlags, etc.)
├── error.rs             # ZjctlError enum, ExitCode, JSON error serialization
├── output.rs            # OutputMode (Human / Json), format helpers
├── zellij.rs            # Zellij native adapter: run_command, dry-run support
├── resolver.rs          # Pane/tab reference resolution (full, numeric, name)
├── guard.rs             # Safety checks (no self-write, active pane detection)
├── commands/
│   ├── mod.rs           # Re-exports, shared helpers
│   ├── list.rs          # sessions list, tabs list, panes list
│   ├── read.rs          # read pane output
│   ├── write.rs         # write text / send-keys
│   ├── focus.rs         # focus pane / tab
│   └── open.rs          # open tab / pane
```

---

## 3. Ordered Phases

### Phase 0 — Foundation & Refactor
**Goal:** Split the flat `main.rs` into modules and establish the error/output contract.

| # | Task | Files to create/modify |
|---|------|------------------------|
| 0.1 | Create `src/error.rs` — `ZjctlError` enum with exit codes 0–8, implement `serde::Serialize` for JSON errors. | `create` |
| 0.2 | Create `src/output.rs` — `OutputMode` enum (Human/Json), auto-detect from isatty, `--json` override. `println_json` helper. | `create` |
| 0.3 | Create `src/zellij.rs` — Extract `run_zellij` / `run_zellij_os` and `format_args_for_error`. Add `--dry-run` support (print command JSON, skip spawn). | `create` |
| 0.4 | Create `src/cli.rs` — Move all clap structs out of `main.rs`. Add `GlobalFlags` struct for `--session`, `--json`, `--quiet`, `--dry-run`. | `create` |
| 0.5 | Refactor `src/main.rs` — Wire new modules. Keep routing logic only. | `modify` |

**Validation:**
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo run -- panes list   # still works
cargo run -- tabs list    # still works
```

**Testing:** Unit tests for `format_args_for_error` (migrate existing), `OutputMode` detection, `ZjctlError` JSON serialization.

---

### Phase 1 — Discovery Polish
**Goal:** Add `sessions list` and enrich list output with proper JSON schema and human tables.

| # | Task | Files to create/modify |
|---|------|------------------------|
| 1.1 | Add `sessions list` subcommand to `cli.rs` and `commands/list.rs`. | `modify` |
| 1.2 | Implement `SessionsList` in `commands/list.rs`: shell out to `zellij list-sessions`, parse output, emit JSON schema per design doc §4.1. | `modify` |
| 1.3 | Implement `TabsList` and `PanesList` with proper JSON schema (currently raw zellij JSON). Use `serde_json::Value` as intermediate, map to our schema. | `modify` |
| 1.4 | Add human table formatting in `output.rs` for lists (auto-width, tabular). Use a lightweight formatter (manual `println!` with padding; no extra crate). | `modify` |

**Validation:**
```bash
cargo run -- sessions list --json
cargo run -- tabs list --json
cargo run -- panes list --json
cargo run -- sessions list       # human table
cargo run -- tabs list           # human table
```

**Testing:** Snapshot tests for JSON schema shape (use `serde_json::json!` asserts). Mock `zellij` binary calls with a shell wrapper in `tests/`.

---

### Phase 2 — Identity Resolution
**Goal:** Resolve pane/tab references so agents can use `2`, `name:editor`, etc.

| # | Task | Files to create/modify |
|---|------|------------------------|
| 2.1 | Create `src/resolver.rs` — `PaneRef` / `TabRef` enums. `resolve_pane(session, ref)` and `resolve_tab(session, ref)` functions. | `create` |
| 2.2 | Implement resolution rules: exact full ID → numeric shorthand → name match. Error with candidate list on ambiguity. | `modify` |
| 2.3 | Integrate resolver into `read`, `write`, `focus`, `send-keys` commands. Replace raw `--pane` strings with resolved IDs before calling zellij. | `modify` |

**Validation:**
```bash
cargo run -- read --pane 2 --lines 10       # numeric shorthand
cargo run -- write --pane name:nvim --text "hello" --enter
cargo run -- focus --tab editor               # name match
```

**Testing:** Unit tests for resolver with mocked pane/tab lists. Edge cases: ambiguity, not found, exact match vs fuzzy.

---

### Phase 3 — Core Operations
**Goal:** Implement send-keys, focus, open tab, open pane.

| # | Task | Files to create/modify |
|---|------|------------------------|
| 3.1 | Add `send-keys` subcommand in `cli.rs` and `commands/write.rs`. Map key names (`Ctrl+c`, `Enter`, etc.) to zellij `send-keys` syntax. | `modify` |
| 3.2 | Add `focus pane` and `focus tab` in `cli.rs` and `commands/focus.rs`. Shell out to `zellij action focus-pane-id` / `focus-tab`. | `create` / `modify` |
| 3.3 | Add `open tab --name <name>` in `cli.rs` and `commands/open.rs`. Use `zellij action new-tab --name`. | `create` / `modify` |
| 3.4 | Add `open pane --command <cmd> [--cwd <path>]` in `cli.rs` and `commands/open.rs`. Use `zellij action new-pane --command` / `--cwd`. | `modify` |

**Validation:**
```bash
cargo run -- send-keys --pane 2 Ctrl+c
cargo run -- focus --pane terminal_1
cargo run -- focus --tab 1
cargo run -- open tab --name logs
cargo run -- open pane --command "tail -f /var/log/syslog"
```

**Testing:** Command-line parsing tests. Mock zellij invocations to assert correct args are passed.

---

### Phase 4 — Safety & Guards
**Goal:** Prevent self-write accidents and add dry-run / quiet support everywhere.

| # | Task | Files to create/modify |
|---|------|------------------------|
| 4.1 | Create `src/guard.rs` — `check_self_write(pane_id)` and `check_close_active(pane_id)`. Detect current pane via `ZELLIJ_PANE_ID` env var. | `create` |
| 4.2 | Integrate guards into `write`, `send-keys`, `focus`, `close` (if any). Return exit code 8 with structured error when guard triggers. Allow `--force` to bypass. | `modify` |
| 4.3 | Wire `--dry-run` through all commands in `commands/`. When enabled, print the zellij command that would run as JSON and exit 0. | `modify` |
| 4.4 | Wire `--quiet` through all commands. Suppress non-error output in human mode. | `modify` |

**Validation:**
```bash
cargo run -- write --pane $(echo $ZELLIJ_PANE_ID) --text "x"    # should fail
cargo run -- write --pane $(echo $ZELLIJ_PANE_ID) --text "x" --force  # should succeed
cargo run -- open tab --name test --dry-run
cargo run -- sessions list --quiet
```

**Testing:** Unit tests for guard logic with mocked `ZELLIJ_PANE_ID`. Assert correct exit codes.

---

### Phase 5 — Integration & Hardening
**Goal:** End-to-end polish, integration tests, docs.

| # | Task | Files to create/modify |
|---|------|------------------------|
| 5.1 | Add integration tests in `tests/integration.rs` that create a fake `zellij` shell script, set `PATH`, and run `zjctl` commands. | `create` |
| 5.2 | Ensure `cargo fmt`, `cargo clippy`, `cargo test --all-features` pass cleanly. | `modify` |
| 5.3 | Add JSON schema examples to `docs/schemas/` for list, read, write, error outputs. | `create` |
| 5.4 | Update `README.md` with new commands and examples. | `modify` |

**Validation:**
```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

---

## 4. Testing Strategy Per Phase

| Phase | Test Type | Details |
|-------|-----------|---------|
| 0 | Unit | Error JSON serialization, output mode detection, zellij arg formatting |
| 1 | Unit + Snapshot | JSON schema shape assertions for list outputs; human table formatting |
| 2 | Unit | Resolver logic with injected mock session/tab/pane data |
| 3 | Unit + Mock | Command arg construction; verify correct `zellij action` args are built |
| 4 | Unit | Guard logic with mocked env vars; dry-run output assertions |
| 5 | Integration | Full CLI invocation with fake `zellij` binary; assert stdout/stderr/exit code |

**Mocking approach:** Use a temporary shell script named `zellij` that writes received args to a file and exits 0. Set `PATH` to its directory in tests. This avoids requiring a real Zellij session.

---

## 5. Validation Commands (per phase and final)

After every phase:
```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

Final validation (run in a real Zellij session):
```bash
cargo build --release
./target/release/zjctl sessions list --json
./target/release/zjctl tabs list --json
./target/release/zjctl panes list --json
./target/release/zjctl read --pane 1 --lines 10 --json
./target/release/zjctl write --pane 2 --text "echo hello" --enter
./target/release/zjctl send-keys --pane 2 Ctrl+c
./target/release/zjctl focus --tab 1
./target/release/zjctl open tab --name test
./target/release/zjctl open pane --command "htop"
./target/release/zjctl open tab --name dry --dry-run
```

---

## 6. Risks and Sequencing Notes

| Risk | Mitigation |
|------|------------|
| Zellij native CLI flags change between versions | Parse defensively; test against latest stable; document tested version in README |
| `ZELLIJ_PANE_ID` is not set outside Zellij | Guards gracefully skip when env var is absent (print warning, do not block) |
| Shell-out latency in rapid agent loops | Document limitation; batch reads when possible; no polling loops in MVP |
| Name-match ambiguity confuses agents | Error with candidate list; recommend using full IDs in scripts |
| Refactor breaks existing commands | Phase 0 ends with all existing commands still working unchanged |

**Sequencing critical path:**
- Phase 0 must complete first (modules and contracts).
- Phase 2 (resolver) should complete before Phase 3 (focus/open) so those commands can use shorthand references.
- Phase 4 (guards) can overlap with Phase 3 but should be merged after to avoid conflict churn.

---

## 7. Explicit Deferrals

The following features from the design doc are **explicitly deferred** to v1 or later:

| Feature | Design Doc Section | Rationale |
|---------|-------------------|-----------|
| `watch` — tail pane output | §4.8 | Requires polling or a plugin; not needed for basic agent control loops |
| `exec` — run command and capture exit | §4.7 | Requires heuristic prompt detection or a plugin; complex and brittle |
| `close pane` / `close tab` | §4.6 | Destructive; guardrails must be solid first; lower priority than open/focus |
| `lock` — cooperative file locks | §4.9 / §8 | Single-user local use makes this less urgent; file locks are brittle |
| `--jsonl` output mode | §10 | Only needed for `watch`; defer with it |
| Layout operations | §12 Future | Out of MVP scope; native `zellij` CLI handles basic layout loading |
| Custom Zellij plugin | §13 | Defer until native CLI limits are proven painful |

---

## 8. Recommended Build Order and First Slice

### Recommended Build Order
1. **Phase 0** — Refactor into modules (error, output, zellij adapter, cli). This unblocks everything.
2. **Phase 1** — `sessions list` + polish existing list commands with JSON schema.
3. **Phase 2** — Identity resolver (pane/tab references).
4. **Phase 3** — `send-keys`, `focus`, `open tab`, `open pane`.
5. **Phase 4** — Safety guards, dry-run, quiet mode.
6. **Phase 5** — Integration tests, docs, final hardening.

### Critical Path
`Phase 0 → Phase 2 → Phase 3` is the critical path. The resolver (Phase 2) is a hard dependency for `focus` and `open` to feel complete, because agents will want to use numeric or name references rather than raw Zellij IDs.

### First Implementation Slice
**Phase 0 (Foundation & Refactor)** is the correct first slice because:
- It does not change behavior; it only reorganizes code.
- It establishes the error and output contracts that every subsequent phase depends on.
- It is low risk and can be validated immediately with existing tests.
- Mason can land it in one PR before moving on to new features.
