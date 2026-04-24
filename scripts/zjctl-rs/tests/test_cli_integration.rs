use assert_cmd::Command;
use predicates::prelude::*;

fn zjctl() -> Command {
    Command::cargo_bin("zjctl").unwrap()
}

#[test]
fn no_args_prints_error_to_stderr() {
    zjctl().assert().failure();
}

#[test]
fn help_flag_shows_usage() {
    zjctl()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("sessions"))
        .stdout(predicate::str::contains("tabs"))
        .stdout(predicate::str::contains("panes"));
}

#[test]
fn tabs_without_session_returns_exit_4() {
    zjctl()
        .args(["tabs", "list"])
        .env_remove("ZELLIJ_SESSION_NAME")
        .assert()
        .code(4)
        .stderr(predicate::str::contains("not_in_session"));
}

#[test]
fn panes_without_session_returns_exit_4() {
    zjctl()
        .args(["panes", "list"])
        .env_remove("ZELLIJ_SESSION_NAME")
        .assert()
        .code(4)
        .stderr(predicate::str::contains("not_in_session"));
}

#[test]
fn sessions_list_without_zellij_returns_zellij_error() {
    zjctl()
        .args(["sessions", "list"])
        .env("PATH", "/nonexistent")
        .assert()
        .code(2)
        .stderr(predicate::str::contains("zellij_error"));
}
