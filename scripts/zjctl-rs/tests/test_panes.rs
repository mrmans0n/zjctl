mod common;

use common::{MockZellij, MOCK_PANES_JSON, MOCK_TAB_NAMES};
use zjctl::commands::panes;

#[test]
fn panes_list_returns_selectable_only() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);

    let result = panes::list(None, &mock).unwrap();
    assert_eq!(result.panes.len(), 3); // 4 total, 1 plugin excluded
}

#[test]
fn panes_list_ids_are_qualified() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);

    let result = panes::list(None, &mock).unwrap();
    assert_eq!(result.panes[0].id, "terminal_0");
}

#[test]
fn panes_list_includes_command_and_cwd() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);

    let result = panes::list(None, &mock).unwrap();
    assert_eq!(result.panes[0].command.as_deref(), Some("openclaw"));
    assert_eq!(result.panes[0].cwd.as_deref(), Some("/home/user"));
}

#[test]
fn panes_list_filters_by_tab_index() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("query-tab-names", MOCK_TAB_NAMES);

    let result = panes::list(Some("1"), &mock).unwrap();
    assert_eq!(result.panes.len(), 1);
    assert_eq!(result.panes[0].tab_name, "code");
}

#[test]
fn panes_list_filters_by_tab_name() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("query-tab-names", MOCK_TAB_NAMES);

    let result = panes::list(Some("code"), &mock).unwrap();
    assert_eq!(result.panes.len(), 1);
}

#[test]
fn panes_read_returns_content() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command(
        "dump-screen",
        "$ git status\nOn branch main\nnothing to commit",
    );

    let result = panes::read("terminal_7", false, false, &mock).unwrap();
    assert_eq!(result.pane_id, "terminal_7");
    assert!(result.content.contains("git status"));
}

#[test]
fn panes_read_accepts_bare_integer() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("dump-screen", "content");

    let result = panes::read("7", false, false, &mock).unwrap();
    assert_eq!(result.pane_id, "terminal_7");
}

#[test]
fn panes_write_succeeds() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("write-chars", "");

    std::env::remove_var("ZELLIJ_PANE_ID");
    let result = panes::write("terminal_0", "hello", &mock, false, false);
    assert!(result.is_ok());
}

#[test]
fn panes_send_keys_succeeds() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("send-keys", "");

    std::env::remove_var("ZELLIJ_PANE_ID");
    let result = panes::send_keys("terminal_0", &["Enter".to_string()], &mock, false, false);
    assert!(result.is_ok());
}

#[test]
fn panes_focus_succeeds() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("focus-pane-id", "");

    let result = panes::focus("terminal_0", &mock, false);
    assert!(result.is_ok());
}

#[test]
fn panes_open_returns_pane_id() {
    let mut mock = MockZellij::new();
    mock.mock_command("new-pane", "terminal_12");

    let result = panes::open_pane(None, false, None, None, None, vec![], &mock, false).unwrap();
    assert_eq!(result, "terminal_12");
}
