# zjctl

`zjctl` is a Rust CLI for programmatic control of [Zellij](https://zellij.dev) panes and tabs.

It is designed as a thin, agent-friendly wrapper around Zellij's native CLI so tools and assistants can:

- list sessions, tabs, and panes in structured JSON
- read pane output and scrollback
- send text and keys into other panes
- open or focus panes/tabs without stealing the whole terminal workflow

## Status

Early, usable CLI.

The current control surface includes:

- `zjctl sessions list`
- `zjctl panes list`
- `zjctl tabs list`
- `zjctl panes read <pane>`
- `zjctl panes write <pane> <text>`
- `zjctl panes send-keys <pane> <keys...>`
- `zjctl panes focus <pane>`
- `zjctl panes open [options] [-- COMMAND...]`
- `zjctl tabs focus <tab>`
- `zjctl tabs open [options] [-- COMMAND...]`

## Why

AI agents running in one pane are normally blind to the rest of a Zellij session.
`zjctl` aims to provide a stable local interface for cross-pane observation and control without requiring MCP.

## Install the CLI

Install `zjctl` before installing the agent skill. The skill shells out to this binary.

### Homebrew (recommended)

```bash
brew install mrmans0n/tap/zjctl
```

### Pre-built binaries

Download the archive for your platform from the [GitHub Releases](https://github.com/mrmans0n/zjctl/releases) page and extract the `zjctl` binary to a directory on your `$PATH`.

### From source

```bash
cargo install --path .
```

Verify the install:

```bash
zjctl --help
```

## Install the Agent Skill

The packaged skill lives at `skills/dist/zjctl.skill`. It is a zip archive that expands to `zjctl/SKILL.md`.

Install the CLI first, then install the skill into the directory used by your harness.
The commands below assume you are running them from this repository root; when installing into another project, replace `skills/dist/zjctl.skill` with the archive's absolute path.

### Claude Code

Claude Code loads skills from `~/.claude/skills/<skill-name>/SKILL.md` for personal use and `.claude/skills/<skill-name>/SKILL.md` for project use.

Personal install:

```bash
mkdir -p ~/.claude/skills
unzip -o skills/dist/zjctl.skill -d ~/.claude/skills
```

Project install:

```bash
mkdir -p .claude/skills
unzip -o skills/dist/zjctl.skill -d .claude/skills
git add .claude/skills/zjctl/SKILL.md
```

### Codex

Codex loads user skills from `$CODEX_HOME/skills`; when `CODEX_HOME` is unset, use `~/.codex/skills`.

```bash
CODEX_HOME="${CODEX_HOME:-$HOME/.codex}"
mkdir -p "$CODEX_HOME/skills"
unzip -o skills/dist/zjctl.skill -d "$CODEX_HOME/skills"
```

For a repository-shared Codex setup, commit the skill source under the target repo and reference it from `AGENTS.md` so the harness advertises it to agents:

```bash
mkdir -p skills
unzip -o skills/dist/zjctl.skill -d skills
git add skills/zjctl/SKILL.md AGENTS.md
```

### Cursor

Cursor loads skills from `~/.cursor/skills/<skill-name>/SKILL.md` for personal use and `.cursor/skills/<skill-name>/SKILL.md` for project use.

Personal install:

```bash
mkdir -p ~/.cursor/skills
unzip -o skills/dist/zjctl.skill -d ~/.cursor/skills
```

Project install:

```bash
mkdir -p .cursor/skills
unzip -o skills/dist/zjctl.skill -d .cursor/skills
git add .cursor/skills/zjctl/SKILL.md
```

## Documentation

- [Developing](DEVELOPING.md)
- [Releasing](RELEASING.md)
- [Release preflight checklist](docs/RELEASE-PREFLIGHT.md)
