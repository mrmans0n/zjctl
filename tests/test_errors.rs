use zjctl::error::ZjctlError;

#[test]
fn unknown_command_has_exit_code_1() {
    let err = ZjctlError::unknown_command("bad command");
    assert_eq!(err.exit_code, 1);
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert_eq!(json["error"], "unknown_command");
    assert_eq!(json["message"], "bad command");
    assert_eq!(json["exit_code"], 1);
}

#[test]
fn missing_argument_has_exit_code_1() {
    let err = ZjctlError::missing_argument("need pane id");
    assert_eq!(err.exit_code, 1);
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert_eq!(json["message"], "need pane id");
}

#[test]
fn invalid_target_has_exit_code_1() {
    let err = ZjctlError::invalid_target("Pane not found: foo");
    assert_eq!(err.exit_code, 1);
}

#[test]
fn zellij_error_has_exit_code_2_and_command_field() {
    let err = ZjctlError::zellij_error("failed", "zellij action write-chars foo");
    assert_eq!(err.exit_code, 2);
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert_eq!(json["error"], "zellij_error");
    assert_eq!(json["command"], "zellij action write-chars foo");
}

#[test]
fn self_write_blocked_has_exit_code_3_with_target_and_self() {
    let err = ZjctlError::self_write_blocked("terminal_8".into(), "terminal_8".into());
    assert_eq!(err.exit_code, 3);
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert_eq!(json["error"], "self_write_blocked");
    assert_eq!(json["target"], "terminal_8");
    assert_eq!(json["self"], "terminal_8");
}

#[test]
fn not_in_session_has_exit_code_4() {
    let err = ZjctlError::not_in_session();
    assert_eq!(err.exit_code, 4);
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert_eq!(json["error"], "not_in_session");
}

#[test]
fn parse_error_has_exit_code_5() {
    let err = ZjctlError::parse_error("bad json");
    assert_eq!(err.exit_code, 5);
}

#[test]
fn error_json_omits_none_fields() {
    let err = ZjctlError::unknown_command("test");
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert!(json.get("command").is_none());
    assert!(json.get("target").is_none());
    assert!(json.get("self").is_none());
}
