mod common;

use common::MockZellij;
use zjctl::commands::sessions;
#[test]
fn sessions_list_returns_json_with_session_names() {
    let mut mock = MockZellij::new();
    mock.mock_command("list-sessions", "didactic-newt\nambrosio\nclauded");

    let result = sessions::list(&mock).unwrap();
    assert_eq!(result.sessions.len(), 3);
    assert_eq!(result.sessions[0].name, "didactic-newt");
    assert_eq!(result.sessions[1].name, "ambrosio");
    assert_eq!(result.sessions[2].name, "clauded");
}
