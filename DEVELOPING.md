# Developing

Use the commands below from the repository root.

## Validation

```bash
cargo fmt --all
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-features
```

## Local Runs

```bash
cargo run -- panes list
cargo run -- tabs list
cargo run -- sessions list
```

## Rebuild the Skill Archive

The distributed skill archive must match `skills/zjctl/SKILL.md`.

```bash
cd skills
zip -r dist/zjctl.skill zjctl/SKILL.md
```

Verify the archive:

```bash
unzip -l dist/zjctl.skill
```
