# Releasing

Releases are automated through GitHub Actions using [cargo-dist](https://github.com/axodotdev/cargo-dist).

## Release Checklist

1. Before tagging:
   - Ensure `CHANGELOG.md` is updated with the new version.
   - Ensure version in `Cargo.toml` matches the tag you will push.
   - Run `cargo fmt --all -- --check`.
   - Run `cargo clippy --all-targets --all-features -- -D warnings`.
   - Run `cargo test --all-features`.
   - Run `cargo build --release`.
   - Verify `dist plan` output looks correct.
   - Follow the full [release preflight checklist](docs/RELEASE-PREFLIGHT.md).

2. Create and push the tag:

   ```bash
   git tag v0.1.0
   git push origin v0.1.0
   ```

3. Wait for CI:
   - The [Release workflow](.github/workflows/release.yml) will build binaries for all platforms.
   - It will create a GitHub Release and publish the Homebrew formula to [mrmans0n/homebrew-tap](https://github.com/mrmans0n/homebrew-tap).

## Prerequisites

- The GitHub repository must have a `HOMEBREW_TAP_TOKEN` secret configured with a personal access token that has `repo` and `workflow` scopes for `mrmans0n/homebrew-tap`.
- The `mrmans0n/homebrew-tap` repository must exist and be accessible.
