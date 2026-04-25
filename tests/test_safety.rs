use zjctl::safety::check_self_write;

#[test]
fn blocks_matching_qualified_pane_id() {
    let result = check_self_write("terminal_8", false, Some("8"));
    let err = result.unwrap_err();
    assert_eq!(err.exit_code, 3);
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert_eq!(json["error"], "self_write_blocked");
    assert_eq!(json["target"], "terminal_8");
    assert_eq!(json["self"], "terminal_8");
}

#[test]
fn blocks_bare_integer_matching_pane_id() {
    let result = check_self_write("8", false, Some("8"));
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().exit_code, 3);
}

#[test]
fn allows_different_pane() {
    let result = check_self_write("terminal_3", false, Some("8"));
    assert!(result.is_ok());
}

#[test]
fn bypassed_with_no_guard() {
    let result = check_self_write("terminal_8", true, Some("8"));
    assert!(result.is_ok());
}

#[test]
fn skipped_when_pane_id_not_set() {
    let result = check_self_write("terminal_8", false, None);
    assert!(result.is_ok());
}
