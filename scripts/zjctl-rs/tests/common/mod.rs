use std::collections::HashMap;
use zjctl::error::ZjctlError;
use zjctl::zellij::ZellijRunner;

pub struct MockZellij {
    fixtures: HashMap<String, MockResponse>,
}

struct MockResponse {
    output: String,
    exit_code: i32,
}

impl MockZellij {
    pub fn new() -> Self {
        Self {
            fixtures: HashMap::new(),
        }
    }

    pub fn mock_command(&mut self, key: &str, output: &str) {
        self.fixtures.insert(
            key.to_string(),
            MockResponse {
                output: output.to_string(),
                exit_code: 0,
            },
        );
    }

    #[allow(dead_code)]
    pub fn mock_command_fail(&mut self, key: &str, output: &str, exit_code: i32) {
        self.fixtures.insert(
            key.to_string(),
            MockResponse {
                output: output.to_string(),
                exit_code,
            },
        );
    }

    fn lookup(&self, args: &[&str]) -> Result<String, ZjctlError> {
        // Try full key first, then progressively shorter suffixes
        let full_key = args.join(" ");
        if let Some(resp) = self.fixtures.get(&full_key) {
            return if resp.exit_code == 0 {
                Ok(resp.output.clone())
            } else {
                Err(ZjctlError::zellij_error(&resp.output, &full_key))
            };
        }

        // Try individual args as keys (matches bash mock behavior)
        for arg in args {
            if let Some(resp) = self.fixtures.get(*arg) {
                return if resp.exit_code == 0 {
                    Ok(resp.output.clone())
                } else {
                    Err(ZjctlError::zellij_error(
                        &resp.output,
                        &format!("zellij {}", full_key),
                    ))
                };
            }
        }

        panic!(
            "MockZellij: unhandled command: {}. Available keys: {:?}",
            full_key,
            self.fixtures.keys().collect::<Vec<_>>()
        );
    }
}

impl ZellijRunner for MockZellij {
    fn run_action(&self, args: &[&str]) -> Result<String, ZjctlError> {
        self.lookup(args)
    }

    fn run_toplevel(&self, args: &[&str]) -> Result<String, ZjctlError> {
        self.lookup(args)
    }
}

// === Shared test fixtures ===

#[allow(dead_code)]
pub const MOCK_PANES_JSON: &str = r#"[
  {"id":0,"is_plugin":false,"is_focused":false,"is_floating":false,"is_suppressed":false,"title":"openclaw","is_selectable":true,"tab_id":0,"tab_position":0,"tab_name":"main","pane_command":"openclaw","pane_cwd":"/home/user"},
  {"id":7,"is_plugin":false,"is_focused":true,"is_floating":false,"is_suppressed":false,"title":"","is_selectable":true,"tab_id":0,"tab_position":0,"tab_name":"main","pane_command":"/bin/zsh","pane_cwd":"/home/user/project"},
  {"id":1,"is_plugin":true,"is_focused":false,"is_floating":false,"is_suppressed":false,"title":"zjstatus","is_selectable":false,"tab_id":0,"tab_position":0,"tab_name":"main"},
  {"id":2,"is_plugin":false,"is_focused":true,"is_floating":false,"is_suppressed":false,"title":"editor","is_selectable":true,"tab_id":1,"tab_position":1,"tab_name":"code","pane_command":"nvim","pane_cwd":"/home/user/code"}
]"#;

#[allow(dead_code)]
pub const MOCK_TAB_NAMES: &str = "main\ncode";

#[allow(dead_code)]
pub const MOCK_TAB_INFO_JSON: &str = r#"{
  "position": 0,
  "name": "main",
  "active": true,
  "tab_id": 0
}"#;
