# zjctl AgentSkill

Agent skill for inspecting and controlling Zellij sessions, tabs, and panes via the Rust `zjctl` CLI.

## Purpose

Provides a structured interface for AI agents to interact with Zellij terminal multiplexer sessions. Agents use the skill to discover `zjctl` commands, understand JSON contracts, and follow safety rules (self-write guards, dry-run previews).

## Prerequisites

- **Zellij** running with at least one session
- **Rust `zjctl` binary** on `$PATH` (see [Install the CLI](#install-the-cli))
- Agent platform that supports skill discovery via `SKILL.md` frontmatter

## Install the CLI

Build and install the binary to Cargo's bin directory:

```bash
cargo install --path scripts/zjctl-rs
```

Verify the install:

```bash
which zjctl
zjctl --help
```

### Alternative: symlink the release binary

If you prefer not to use `cargo install`:

```bash
cargo build --release --manifest-path scripts/zjctl-rs/Cargo.toml
ln -sf "$(pwd)/scripts/zjctl-rs/target/release/zjctl" /usr/local/bin/zjctl
```

## Package the Skill

The skill is distributed as a `.skill` archive (zip containing `zjctl/SKILL.md`).

To rebuild the archive after editing `SKILL.md`:

```bash
cd /path/to/zjctl/skills
zip -r dist/zjctl.skill zjctl/SKILL.md
```

Verify:

```bash
unzip -l /path/to/zjctl/skills/dist/zjctl.skill
# Should show: zjctl/SKILL.md
```

## Consume From Another Repo

### Option A: Unzip the archive

```bash
cp /path/to/zjctl/skills/dist/zjctl.skill /path/to/target-repo/skills/
cd /path/to/target-repo/skills
unzip zjctl.skill
```

This creates `skills/zjctl/SKILL.md` in the target repo.

### Option B: Symlink the source directory

For repos on the same machine:

```bash
ln -s /path/to/zjctl/skills/zjctl /path/to/target-repo/skills/zjctl
```

## Agent Discovery

Agents discover skills by scanning `skills/*/SKILL.md` for YAML frontmatter. The zjctl skill declares:

```yaml
---
name: zjctl
description: "Use the shipped Rust zjctl CLI to inspect and safely control Zellij sessions, tabs, and panes from agents."
---
```

Once installed in a repo's `skills/` directory, any agent that scans for `SKILL.md` files will pick it up automatically.

## Validation Flow

Run this walkthrough to confirm the skill and CLI work end-to-end. Requires a running Zellij session with at least two panes.

### 1. Verify CLI availability

```bash
zjctl --help
```

Expected: usage text with `sessions`, `tabs`, `panes` subcommands.

### 2. List sessions

```bash
zjctl sessions list
```

Expected: `{"sessions":[{"name":"..."}]}` with at least one session.

### 3. List panes

```bash
zjctl panes list
```

Expected: JSON array of panes with `id`, `command`, `cwd`, `title`, `focused`, `tab_name` fields.

### 4. Pick a non-current pane and dry-run a write

Choose a pane ID from step 3 that is **not** your current pane (to avoid `self_write_blocked`).

```bash
zjctl --dry-run panes write <pane_id> "echo zjctl-skill-ok"
```

Expected: `{"dry_run":true,"command":["zellij","action","write-chars","--pane-id","<pane_id>","echo zjctl-skill-ok"]}`

### 5. Write and execute

```bash
zjctl panes write <pane_id> "echo zjctl-skill-ok"
zjctl panes send-keys <pane_id> Enter
```

Expected: `{"ok":true}` for each command.

### 6. Read back output

```bash
zjctl panes read <pane_id>
```

Expected: `content` field includes `zjctl-skill-ok`.

### Success criteria

- All commands return valid JSON
- The written command executed in the target pane
- Output confirms `zjctl-skill-ok` was echoed
- The `.skill` archive unzips to `zjctl/SKILL.md` with valid frontmatter

## Troubleshooting

| Symptom | Cause | Fix |
|---------|-------|-----|
| `zjctl: command not found` | Binary not on `$PATH` | Run `cargo install --path scripts/zjctl-rs` from the repository root |
| `not_in_session` (exit 4) | No Zellij session detected | Run from inside a Zellij session, or use `--session <name>` |
| `self_write_blocked` (exit 3) | Targeting your own pane | Pick a different pane, or use `--no-guard` if intentional |
| `invalid_target` (exit 1) | Pane/tab ID doesn't exist | Run `zjctl panes list` to find valid IDs |
| Stale `.skill` archive | `SKILL.md` was edited after packaging | Re-run the zip command from [Package the Skill](#package-the-skill) |
