mod common;

use common::{MockZellij, MOCK_PANES_JSON, MOCK_TAB_INFO_JSON, MOCK_TAB_NAMES};
use zjctl::commands::tabs;

#[test]
fn tabs_list_returns_both_tabs() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("current-tab-info --json", MOCK_TAB_INFO_JSON);

    let result = tabs::list(&mock).unwrap();
    assert_eq!(result.tabs.len(), 2);
}

#[test]
fn tabs_list_marks_active_tab() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("current-tab-info --json", MOCK_TAB_INFO_JSON);

    let result = tabs::list(&mock).unwrap();
    let active: Vec<_> = result.tabs.iter().filter(|t| t.active).collect();
    assert_eq!(active.len(), 1);
    assert_eq!(active[0].name, "main");
}

#[test]
fn tabs_list_includes_tab_id() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-panes --json --all", MOCK_PANES_JSON);
    mock.mock_command("current-tab-info --json", MOCK_TAB_INFO_JSON);

    let result = tabs::list(&mock).unwrap();
    assert_eq!(result.tabs[1].tab_id, 1);
}

#[test]
fn tabs_focus_accepts_numeric_index() {
    let mut mock = MockZellij::new();
    mock.mock_command("query-tab-names", MOCK_TAB_NAMES);
    mock.mock_command("go-to-tab", "");

    let result = tabs::focus("0", &mock, false);
    assert!(result.is_ok());
}

#[test]
fn tabs_focus_accepts_name() {
    let mut mock = MockZellij::new();
    mock.mock_command("query-tab-names", MOCK_TAB_NAMES);
    mock.mock_command("go-to-tab", "");

    let result = tabs::focus("code", &mock, false);
    assert!(result.is_ok());
}
