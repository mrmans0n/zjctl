# Release Preflight

This document is a step-by-step checklist to run before cutting a new `zjctl` release. Following it reduces the chance of a failed or broken release.

## 1. Version alignment

- [ ] `Cargo.toml` `version` field matches the intended release tag (e.g., `0.1.0` → tag `v0.1.0`)
- [ ] `CHANGELOG.md` has an entry for this version
- [ ] No `[Unreleased]` changes are left undocumented

## 2. Build and test

Run these locally before tagging:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
cargo build --release
```

All must pass.

## 3. Verify cargo-dist plan

```bash
dist plan
```

Check that:
- [ ] The version shown matches the intended release
- [ ] All four targets are listed:
  - `aarch64-apple-darwin`
  - `aarch64-unknown-linux-gnu`
  - `x86_64-apple-darwin`
  - `x86_64-unknown-linux-gnu`
- [ ] Homebrew installer is listed under the build plan
- [ ] The formula name is `zjctl.rb`

## 4. Verify dist profile exists

```bash
grep -q '^\[profile.dist\]' Cargo.toml
```

This must return 0. Without this profile, the release build will fail in CI.

## 5. Verify CHANGELOG.md exists

```bash
test -f CHANGELOG.md
```

cargo-dist uses `CHANGELOG.md` to populate the GitHub Release notes. If it is missing, the release body will be empty.

## 6. External prerequisites (one-time setup)

- [ ] GitHub secret `HOMEBREW_TAP_TOKEN` is configured in the `zjctl` repo settings with access to `mrmans0n/homebrew-tap`
- [ ] The `mrmans0n/homebrew-tap` repository exists on GitHub and is not archived
- [ ] The token has at minimum `contents:write` and `workflow` scopes for the tap repo

If any of these are missing, the Homebrew publish job will fail even if the rest of the release succeeds.

## 7. Tag and push

```bash
git tag v0.1.0        # use the actual version
git push origin v0.1.0
```

Do not push tags in batches. cargo-dist processes one tag at a time, and GitHub limits tag triggers to 3 per commit.

## 8. Monitor the release workflow

After pushing the tag, open the [Actions tab](https://github.com/mrmans0n/zjctl/actions) and watch the `Release` workflow.

Stages to verify:
1. **plan** — should succeed quickly; check the manifest output for correctness
2. **build-local-artifacts** — four parallel jobs, one per target
3. **build-global-artifacts** — generates the Homebrew formula and source tarball
4. **host** — creates the GitHub Release, uploads binaries
5. **publish-homebrew-formula** — commits the updated formula to `mrmans0n/homebrew-tap`
6. **announce** — final verification step

If any stage fails, investigate the logs before retrying. Do not re-push the same tag; use a new patch version if needed.

## 9. Post-release smoke test

Once the workflow completes:

```bash
# Verify the release exists on GitHub
curl -s https://api.github.com/repos/mrmans0n/zjctl/releases/latest | jq '.tag_name, .name'

# Verify the Homebrew formula was published
curl -s https://raw.githubusercontent.com/mrmans0n/homebrew-tap/main/Formula/zjctl.rb | head -5
```

Optional — install via Homebrew on a clean machine:

```bash
brew install mrmans0n/tap/zjctl
zjctl --version
```
