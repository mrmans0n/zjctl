# AGENTS.md

This repository contains `zjctl`, a Rust CLI for programmatic Zellij control.

## Standards

All committed code must:

1. include tests when behavior changes
2. pass `cargo fmt --all -- --check`
3. pass `cargo clippy --all-targets --all-features -- -D warnings`
4. pass `cargo test --all-features`

## Scope

Prefer a thin wrapper around Zellij's native CLI before adding custom plugin complexity.
Only add a Zellij plugin when the native CLI cannot provide the needed control surface reliably.

## Language

Code, comments, logs, and user-visible strings must be in English.

## Architecture bias

- Keep `zjctl` agent-friendly: JSON in/out first, human text second.
- Prefer explicit subcommands over hidden magic.
- Keep session/tab/pane references stable and easy to resolve.
- Avoid shell-specific behavior when a direct process call is possible.

## Validation

Before finishing work, run:

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```
