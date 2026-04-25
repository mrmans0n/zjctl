use assert_cmd::Command;
use std::fs;

fn zjctl() -> Command {
    Command::cargo_bin("zjctl").unwrap()
}

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
    assert!(
        !readme.contains("cargo install --path scripts/zjctl-rs"),
        "source installation should target the root crate shipped by releases"
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

#[test]
fn documented_control_surface_matches_cli_help() {
    let readme = read_doc("README.md");

    for command in [
        "zjctl sessions list",
        "zjctl panes list",
        "zjctl tabs list",
        "zjctl panes read <pane>",
        "zjctl panes write <pane> <text>",
        "zjctl panes send-keys <pane> <keys...>",
        "zjctl panes focus <pane>",
        "zjctl panes open [options] [-- COMMAND...]",
        "zjctl tabs focus <tab>",
        "zjctl tabs open [options] [-- COMMAND...]",
    ] {
        assert!(
            readme.contains(command),
            "README.md is missing documented command: {command}"
        );
    }

    for args in [
        &["sessions", "list", "--help"][..],
        &["panes", "list", "--help"][..],
        &["tabs", "list", "--help"][..],
        &["panes", "read", "--help"][..],
        &["panes", "write", "--help"][..],
        &["panes", "send-keys", "--help"][..],
        &["panes", "focus", "--help"][..],
        &["panes", "open", "--help"][..],
        &["tabs", "focus", "--help"][..],
        &["tabs", "open", "--help"][..],
    ] {
        zjctl().args(args).assert().success();
    }

    for stale_command in ["zjctl read --pane", "zjctl write --pane"] {
        assert!(
            !readme.contains(stale_command),
            "README.md should not document stale top-level command: {stale_command}"
        );
    }
}

fn read_doc(path: &str) -> String {
    fs::read_to_string(format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path))
        .unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}
