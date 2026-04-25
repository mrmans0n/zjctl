# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-25

### Added

- Initial release with core Zellij control commands:
  - `zjctl panes list` — list panes in the current session
  - `zjctl tabs list` — list tabs in the current session
  - `zjctl read --pane <id>` — read pane output
  - `zjctl write --pane <id> --text <text> [--enter]` — send text to a pane
- Structured JSON output support for programmatic consumption
- Cross-platform builds: macOS (Apple Silicon, Intel) and Linux (ARM64, x86_64)
- Homebrew formula published to `mrmans0n/homebrew-tap`

[Unreleased]: https://github.com/mrmans0n/zjctl/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mrmans0n/zjctl/releases/tag/v0.1.0
