# zjctl

`zjctl` is a Rust CLI for programmatic control of [Zellij](https://zellij.dev) panes and tabs.

It is designed as a thin, agent-friendly wrapper around Zellij's native CLI so tools and assistants can:

- list sessions, tabs, and panes in structured JSON
- read pane output and scrollback
- send text and keys into other panes
- open or focus panes/tabs without stealing the whole terminal workflow

## Status

Early bootstrap.

The initial version already includes a small usable control surface:

- `zjctl panes list`
- `zjctl tabs list`
- `zjctl read --pane <pane>`
- `zjctl write --pane <pane> --text <text> [--enter]`

## Why

AI agents running in one pane are normally blind to the rest of a Zellij session.
`zjctl` aims to provide a stable local interface for cross-pane observation and control without requiring MCP.

## Development

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo run -- panes list
```

## Release

Releases are intended to run through GitHub Actions + cargo-dist.
