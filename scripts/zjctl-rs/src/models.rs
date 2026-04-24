use serde::{Deserialize, Serialize};

// === Zellij input types (deserialized from zellij JSON output) ===

#[derive(Debug, Deserialize)]
pub struct ZellijPaneInfo {
    pub id: u32,
    pub is_plugin: bool,
    pub is_selectable: bool,
    pub title: String,
    #[serde(alias = "pane_command", alias = "terminal_command")]
    pub pane_command: Option<String>,
    #[serde(alias = "pane_cwd")]
    pub pane_cwd: Option<String>,
    pub is_focused: bool,
    pub is_floating: bool,
    #[serde(default)]
    pub is_suppressed: bool,
    pub tab_id: u32,
    pub tab_position: u32,
    pub tab_name: String,
}

#[derive(Debug, Deserialize)]
pub struct ZellijTabInfo {
    pub position: u32,
    pub name: String,
    pub active: bool,
    pub tab_id: u32,
}

// === zjctl output types (serialized to JSON for consumers) ===

#[derive(Debug, Serialize)]
pub struct SessionsOutput {
    pub sessions: Vec<Session>,
}

#[derive(Debug, Serialize)]
pub struct Session {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct TabsOutput {
    pub tabs: Vec<Tab>,
}

#[derive(Debug, Serialize)]
pub struct Tab {
    pub index: u32,
    pub name: String,
    pub active: bool,
    pub tab_id: u32,
}

#[derive(Debug, Serialize)]
pub struct PanesOutput {
    pub panes: Vec<Pane>,
}

#[derive(Debug, Serialize)]
pub struct Pane {
    pub id: String,
    pub command: Option<String>,
    pub cwd: Option<String>,
    pub title: Option<String>,
    pub focused: bool,
    pub floating: bool,
    pub tab_id: u32,
    pub tab_name: String,
}

#[derive(Debug, Serialize)]
pub struct PaneContent {
    pub pane_id: String,
    pub content: String,
}
