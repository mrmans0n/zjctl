use crate::error::ZjctlError;
use crate::models::ZellijPaneInfo;
use crate::zellij::ZellijRunner;

/// Resolve a tab identifier to a 0-based index.
/// Accepts numeric index or tab name.
pub fn resolve_tab(input: &str, zellij: &dyn ZellijRunner) -> Result<u32, ZjctlError> {
    // Numeric: pass through
    if let Ok(index) = input.parse::<u32>() {
        return Ok(index);
    }

    // Name-based: query tab names
    let tab_names_raw = zellij.run_action(&["query-tab-names"])?;

    for (index, name) in tab_names_raw.lines().enumerate() {
        if name == input {
            return Ok(index as u32);
        }
    }

    Err(ZjctlError::invalid_target(format!(
        "Tab not found: {}",
        input
    )))
}

/// Resolve a pane identifier to a qualified pane ID (terminal_N or plugin_N).
/// Accepts "terminal_N", "plugin_N", bare integer N, or a pane title.
pub fn resolve_pane(input: &str, zellij: &dyn ZellijRunner) -> Result<String, ZjctlError> {
    // Already qualified
    if input.starts_with("terminal_") || input.starts_with("plugin_") {
        return Ok(input.to_string());
    }

    // Bare integer -> terminal_N
    if input.parse::<u32>().is_ok() {
        return Ok(format!("terminal_{}", input));
    }

    // Name/title-based: query list-panes and match
    let panes_raw = zellij.run_action(&["list-panes", "--json", "--all"])?;
    let panes: Vec<ZellijPaneInfo> = serde_json::from_str(&panes_raw)
        .map_err(|e| ZjctlError::parse_error(format!("Failed to parse panes JSON: {}", e)))?;

    let matches: Vec<&ZellijPaneInfo> = panes
        .iter()
        .filter(|p| !p.is_plugin && p.title == input)
        .collect();

    match matches.len() {
        0 => Err(ZjctlError::invalid_target(format!(
            "Pane not found: {}",
            input
        ))),
        1 => Ok(format!("terminal_{}", matches[0].id)),
        _ => Err(ZjctlError::invalid_target(format!(
            "Ambiguous pane name: {} (matches multiple panes)",
            input
        ))),
    }
}
