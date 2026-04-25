use std::collections::BTreeMap;

use crate::cli::TabsVerb;
use crate::error::ZjctlError;
use crate::identity::resolve_tab;
use crate::models::{Tab, TabsOutput, ZellijPaneInfo, ZellijTabInfo};
use crate::output::{emit, emit_dry_run, emit_ok, OutputFormat};
use crate::zellij::ZellijRunner;

pub fn run(
    verb: TabsVerb,
    zellij: &dyn ZellijRunner,
    format: &OutputFormat,
    dry_run: bool,
) -> Result<(), ZjctlError> {
    match verb {
        TabsVerb::List => {
            if dry_run {
                emit_dry_run(
                    &["zellij", "action", "list-panes", "--json", "--all"],
                    format,
                );
                return Ok(());
            }
            let output = list(zellij)?;
            emit(&output, format, |o| {
                println!("{:<8}{:<16}{:<8}TAB_ID", "INDEX", "NAME", "ACTIVE");
                for tab in &o.tabs {
                    let active = if tab.active { "*" } else { "" };
                    println!(
                        "{:<8}{:<16}{:<8}{}",
                        tab.index, tab.name, active, tab.tab_id
                    );
                }
            });
            Ok(())
        }
        TabsVerb::Focus { tab } => {
            focus(&tab, zellij, format, dry_run)?;
            if !dry_run {
                emit_ok(format);
            }
            Ok(())
        }
        TabsVerb::Open {
            name,
            layout,
            command,
        } => open(name, layout, command, zellij, format, dry_run),
    }
}

pub fn list(zellij: &dyn ZellijRunner) -> Result<TabsOutput, ZjctlError> {
    let panes_raw = zellij.run_action(&["list-panes", "--json", "--all"])?;
    let panes: Vec<ZellijPaneInfo> = serde_json::from_str(&panes_raw)
        .map_err(|e| ZjctlError::parse_error(format!("Failed to parse panes: {}", e)))?;

    let tab_info_raw = zellij.run_action(&["current-tab-info", "--json"])?;
    let tab_info: ZellijTabInfo = serde_json::from_str(&tab_info_raw)
        .map_err(|e| ZjctlError::parse_error(format!("Failed to parse tab info: {}", e)))?;

    // Group panes by tab_id, extract tab info
    let mut tab_map: BTreeMap<u32, (u32, String)> = BTreeMap::new();
    for pane in &panes {
        tab_map
            .entry(pane.tab_id)
            .or_insert((pane.tab_position, pane.tab_name.clone()));
    }

    let mut tabs: Vec<Tab> = tab_map
        .into_iter()
        .map(|(tab_id, (position, name))| Tab {
            index: position,
            name,
            active: tab_id == tab_info.tab_id,
            tab_id,
        })
        .collect();

    tabs.sort_by_key(|t| t.index);

    Ok(TabsOutput { tabs })
}

pub fn focus(
    target: &str,
    zellij: &dyn ZellijRunner,
    format: &OutputFormat,
    dry_run: bool,
) -> Result<(), ZjctlError> {
    let index = resolve_tab(target, zellij)?;
    let one_based = (index + 1).to_string();

    if dry_run {
        emit_dry_run(&["zellij", "action", "go-to-tab", &one_based], format);
        return Ok(());
    }

    zellij.run_action(&["go-to-tab", &one_based])?;
    Ok(())
}

fn open(
    name: Option<String>,
    layout: Option<String>,
    command: Vec<String>,
    zellij: &dyn ZellijRunner,
    format: &OutputFormat,
    dry_run: bool,
) -> Result<(), ZjctlError> {
    let mut args: Vec<String> = vec!["new-tab".to_string()];
    if let Some(ref n) = name {
        args.push("--name".to_string());
        args.push(n.clone());
    }
    if let Some(ref l) = layout {
        args.push("--layout".to_string());
        args.push(l.clone());
    }
    if !command.is_empty() {
        args.push("--".to_string());
        args.extend(command);
    }

    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    if dry_run {
        let mut full_cmd = vec!["zellij", "action"];
        full_cmd.extend(&args_refs);
        emit_dry_run(&full_cmd, format);
        return Ok(());
    }

    zellij.run_action(&args_refs)?;

    // Re-query to find the new tab ID.
    let panes_raw = zellij.run_action(&["list-panes", "--json", "--all"])?;
    let panes: Vec<ZellijPaneInfo> = serde_json::from_str(&panes_raw)
        .map_err(|e| ZjctlError::parse_error(format!("Failed to parse panes: {}", e)))?;

    let max_tab_id = panes.iter().map(|p| p.tab_id).max().unwrap_or(0);

    let result = serde_json::json!({"tab_id": max_tab_id});
    emit(&result, format, |_| {});
    Ok(())
}
