use std::fs;

#[test]
fn readme_documents_skill_installation_for_major_harnesses() {
    let readme = read_doc("README.md");

    for section in [
        "## Install the Agent Skill",
        "### Claude Code",
        "### Codex",
        "### Cursor",
        "/plugin marketplace add mrmans0n/zjctl",
        "/plugin install zjctl@mrmans0n-zjctl",
        "npx skills add mrmans0n/zjctl --skill zjctl --agent codex",
        "npx skills add mrmans0n/zjctl --skill zjctl --agent cursor",
    ] {
        assert!(
            readme.contains(section),
            "README.md is missing required documentation: {section}"
        );
    }

    assert!(
        !readme.contains("unzip -o skills/dist/zjctl.skill -d ~/.claude/skills"),
        "Claude Code skill installation should use the plugin marketplace"
    );
}

#[test]
fn operational_guides_are_split_out_of_readme() {
    let readme = read_doc("README.md");
    let developing = read_doc("DEVELOPING.md");
    let releasing = read_doc("RELEASING.md");

    assert!(
        !readme.contains("## Development"),
        "development guide should live in DEVELOPING.md"
    );
    assert!(
        !readme.contains("## Release\n"),
        "release guide should live in RELEASING.md"
    );
    assert!(developing.contains("# Developing"));
    assert!(developing.contains("cargo clippy --all-targets --all-features -- -D warnings"));
    assert!(releasing.contains("# Releasing"));
    assert!(releasing.contains("HOMEBREW_TAP_TOKEN"));
}

fn read_doc(path: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}
