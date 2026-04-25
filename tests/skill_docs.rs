use std::fs;
use std::process::Command;

#[test]
fn skill_install_fallback_targets_canonical_subcrate() {
    let skill = fs::read_to_string("skills/zjctl/SKILL.md").expect("read skill docs");

    assert!(
        skill.contains(r#"cargo install --path "$tmpdir/zjctl/scripts/zjctl-rs""#),
        "skill install fallback must install the canonical scripts/zjctl-rs implementation"
    );
    assert!(
        !skill.contains("cargo install --git https://github.com/mrmans0n/zjctl"),
        "cargo install --git installs the repository-root crate, not scripts/zjctl-rs"
    );
}

#[test]
fn packaged_skill_matches_source_skill() {
    let source = fs::read_to_string("skills/zjctl/SKILL.md").expect("read source skill");
    let output = Command::new("unzip")
        .args(["-p", "skills/dist/zjctl.skill", "zjctl/SKILL.md"])
        .output()
        .expect("run unzip");

    assert!(
        output.status.success(),
        "unzip failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let packaged = String::from_utf8(output.stdout).expect("packaged skill is utf-8");
    assert_eq!(packaged, source);
}
