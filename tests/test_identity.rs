mod common;

use common::MockZellij;
use zjctl::identity::{resolve_pane, resolve_tab};

#[test]
fn resolve_tab_passes_through_numeric_index() {
    let mock = MockZellij::new();
    let result = resolve_tab("2", &mock).unwrap();
    assert_eq!(result, 2);
}

#[test]
fn resolve_tab_resolves_name() {
    let mut mock = MockZellij::new();
    mock.mock_command("query-tab-names", "openclaw\nclaude-remote\nopencode");
    let result = resolve_tab("opencode", &mock).unwrap();
    assert_eq!(result, 2);
}

#[test]
fn resolve_tab_resolves_first_tab_by_name() {
    let mut mock = MockZellij::new();
    mock.mock_command("query-tab-names", "openclaw\nclaude-remote\nopencode");
    let result = resolve_tab("openclaw", &mock).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn resolve_tab_errors_on_unknown_name() {
    let mut mock = MockZellij::new();
    mock.mock_command("query-tab-names", "openclaw\nclaude-remote\nopencode");
    let result = resolve_tab("nonexistent", &mock);
    let err = result.unwrap_err();
    assert_eq!(err.exit_code, 1);
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert_eq!(json["error"], "invalid_target");
}

#[test]
fn resolve_pane_passes_through_terminal_n() {
    let mock = MockZellij::new();
    let result = resolve_pane("terminal_8", &mock).unwrap();
    assert_eq!(result, "terminal_8");
}

#[test]
fn resolve_pane_passes_through_plugin_n() {
    let mock = MockZellij::new();
    let result = resolve_pane("plugin_2", &mock).unwrap();
    assert_eq!(result, "plugin_2");
}

#[test]
fn resolve_pane_converts_bare_integer() {
    let mock = MockZellij::new();
    let result = resolve_pane("8", &mock).unwrap();
    assert_eq!(result, "terminal_8");
}

#[test]
fn resolve_pane_resolves_by_title() {
    let mut mock = MockZellij::new();
    mock.mock_command(
        "list-panes --json --all",
        r#"[
            {"id": 0, "is_plugin": false, "title": "openclaw", "tab_id": 0, "is_selectable": true, "is_focused": false, "is_floating": false, "is_suppressed": false, "tab_position": 0, "tab_name": "main"},
            {"id": 7, "is_plugin": false, "title": "my-shell", "tab_id": 0, "is_selectable": true, "is_focused": false, "is_floating": false, "is_suppressed": false, "tab_position": 0, "tab_name": "main"},
            {"id": 2, "is_plugin": true, "title": "zjstatus", "tab_id": 0, "is_selectable": false, "is_focused": false, "is_floating": false, "is_suppressed": false, "tab_position": 0, "tab_name": "main"}
        ]"#,
    );
    let result = resolve_pane("my-shell", &mock).unwrap();
    assert_eq!(result, "terminal_7");
}

#[test]
fn resolve_pane_errors_on_unknown_name() {
    let mut mock = MockZellij::new();
    mock.mock_command(
        "list-panes --json --all",
        r#"[
            {"id": 0, "is_plugin": false, "title": "openclaw", "tab_id": 0, "is_selectable": true, "is_focused": false, "is_floating": false, "is_suppressed": false, "tab_position": 0, "tab_name": "main"}
        ]"#,
    );
    let result = resolve_pane("nonexistent", &mock);
    let err = result.unwrap_err();
    let json: serde_json::Value = serde_json::from_str(&err.to_json()).unwrap();
    assert_eq!(json["error"], "invalid_target");
}
