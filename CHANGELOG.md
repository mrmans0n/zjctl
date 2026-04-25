# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-04-25

### Added

- Initial release with core Zellij control commands:
  - `zjctl sessions list` — list Zellij sessions
  - `zjctl panes list` — list panes in the current session
  - `zjctl tabs list` — list tabs in the current session
  - `zjctl panes read <pane>` — read pane output
  - `zjctl panes write <pane> <text>` — send text to a pane
  - `zjctl panes send-keys <pane> <keys...>` — send key sequences to a pane
  - `zjctl panes focus <pane>` — focus a pane
  - `zjctl panes open [options] [-- COMMAND...]` — open a pane
  - `zjctl tabs focus <tab>` — focus a tab
  - `zjctl tabs open [options] [-- COMMAND...]` — open a tab
- Structured JSON output support for programmatic consumption
- Cross-platform builds: macOS (Apple Silicon, Intel) and Linux (ARM64, x86_64)
- Homebrew formula published to `mrmans0n/homebrew-tap`

[Unreleased]: https://github.com/mrmans0n/zjctl/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/mrmans0n/zjctl/releases/tag/v0.1.0
