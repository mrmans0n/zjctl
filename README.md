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

The skill source lives at `skills/zjctl/SKILL.md`.
Install the CLI first, then install the skill through the package manager used by your harness.

### Claude Code

Claude Code installs agent skills through plugins. Add the `zjctl` marketplace from inside Claude Code, then install the plugin:

```text
/plugin marketplace add mrmans0n/zjctl
/plugin install zjctl@mrmans0n-zjctl
```

For a repository-shared install, use the Claude Code CLI with project scope:

```bash
claude plugin install zjctl@mrmans0n-zjctl --scope project
```

### Codex

Install with the open Agent Skills CLI:

```bash
npx skills add mrmans0n/zjctl --skill zjctl --agent codex
```

Add `--global` to install into your user Codex skills directory instead of the current project:

```bash
npx skills add mrmans0n/zjctl --skill zjctl --agent codex --global
```

### Cursor

Install with the open Agent Skills CLI:

```bash
npx skills add mrmans0n/zjctl --skill zjctl --agent cursor
```

Add `--global` to install into your user Cursor skills directory instead of the current project:

```bash
npx skills add mrmans0n/zjctl --skill zjctl --agent cursor --global
```

## Documentation

- [Developing](DEVELOPING.md)
- [Releasing](RELEASING.md)
- [Release preflight checklist](docs/RELEASE-PREFLIGHT.md)
