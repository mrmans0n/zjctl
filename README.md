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

## Installation

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

## Development

The Rust crate lives at the repository root, using standard Cargo layout:

- `src/` contains the library and binary source.
- `tests/` contains integration tests.
- `skills/` contains the optional agent skill package.

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo run -- panes list
```

For a local install from a checkout:

```bash
cargo install --path .
```

## Release

Releases are automated through GitHub Actions using [cargo-dist](https://github.com/axodotdev/cargo-dist).

### Release checklist

1. **Before tagging:**
   - Ensure `CHANGELOG.md` is updated with the new version
   - Ensure version in `Cargo.toml` matches the tag you will push
   - Run `cargo fmt --all -- --check`
   - Run `cargo clippy --all-targets --all-features -- -D warnings`
   - Run `cargo test --all-features`
   - Run `cargo build --release`
   - Verify `dist plan` output looks correct
   - Follow the full [release preflight checklist](docs/RELEASE-PREFLIGHT.md)

2. **Create and push the tag:**
   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

3. **Wait for CI:**
   - The [Release workflow](.github/workflows/release.yml) will build binaries for all platforms
   - It will create a GitHub Release and publish the Homebrew formula to [mrmans0n/homebrew-tap](https://github.com/mrmans0n/homebrew-tap)

### Prerequisites

- The GitHub repository must have a `HOMEBREW_TAP_TOKEN` secret configured with a personal access token that has `repo` and `workflow` scopes for `mrmans0n/homebrew-tap`.
- The `mrmans0n/homebrew-tap` repository must exist and be accessible.
