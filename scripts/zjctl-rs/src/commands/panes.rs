use crate::cli::PanesVerb;
use crate::error::ZjctlError;
use crate::identity::{resolve_pane, resolve_tab};
use crate::models::{Pane, PaneContent, PanesOutput, ZellijPaneInfo};
use crate::output::{emit, emit_dry_run, emit_ok, OutputFormat};
use crate::safety::check_self_write;
use crate::zellij::ZellijRunner;

pub fn run(
    verb: PanesVerb,
    zellij: &dyn ZellijRunner,
    format: &OutputFormat,
    dry_run: bool,
    no_guard: bool,
) -> Result<(), ZjctlError> {
    match verb {
        PanesVerb::List { tab } => {
            if dry_run {
                emit_dry_run(
                    &["zellij", "action", "list-panes", "--json", "--all"],
                    format,
                );
                return Ok(());
            }
            let output = list(tab.as_deref(), zellij)?;
            emit(&output, format, |o| {
                println!(
                    "{:<14}{:<12}{:<20}{:<10}{:<10}{:<10}TAB",
                    "ID", "COMMAND", "CWD", "TITLE", "FOCUSED", "FLOATING"
                );
                for p in &o.panes {
                    println!(
                        "{:<14}{:<12}{:<20}{:<10}{:<10}{:<10}{}",
                        p.id,
                        p.command.as_deref().unwrap_or("-"),
                        p.cwd.as_deref().unwrap_or("-"),
                        p.title.as_deref().unwrap_or("-"),
                        p.focused,
                        p.floating,
                        p.tab_name,
                    );
                }
            });
            Ok(())
        }
        PanesVerb::Read { pane, full, ansi } => {
            if dry_run {
                let pane_ref = pane.as_str();
                let mut cmd = vec!["zellij", "action", "dump-screen", "--pane-id", pane_ref];
                if full {
                    cmd.push("--full");
                }
                if ansi {
                    cmd.push("--ansi");
                }
                emit_dry_run(&cmd, format);
                return Ok(());
            }
            let content = read(&pane, full, ansi, zellij)?;
            emit(&content, format, |c| {
                println!("{}\t{}", c.pane_id, c.content);
            });
            Ok(())
        }
        PanesVerb::Write { pane, text } => {
            write(&pane, &text, zellij, dry_run, no_guard)?;
            emit_ok(format);
            Ok(())
        }
        PanesVerb::SendKeys { pane, keys } => {
            send_keys(&pane, &keys, zellij, dry_run, no_guard)?;
            emit_ok(format);
            Ok(())
        }
        PanesVerb::Focus { pane } => {
            focus(&pane, zellij, dry_run)?;
            emit_ok(format);
            Ok(())
        }
        PanesVerb::Open {
            direction,
            floating,
            name,
            cwd,
            tab_id,
            command,
        } => {
            let pane_id = open_pane(
                direction, floating, name, cwd, tab_id, command, zellij, dry_run,
            )?;
            let result = serde_json::json!({"pane_id": pane_id});
            emit(&result, format, |_| {});
            Ok(())
        }
    }
}

pub fn list(
    tab_filter: Option<&str>,
    zellij: &dyn ZellijRunner,
) -> Result<PanesOutput, ZjctlError> {
    let panes_raw = zellij.run_action(&["list-panes", "--json", "--all"])?;
    let panes: Vec<ZellijPaneInfo> = serde_json::from_str(&panes_raw)
        .map_err(|e| ZjctlError::parse_error(format!("Failed to parse panes: {}", e)))?;

    let tab_position_filter = match tab_filter {
        Some(filter) => Some(resolve_tab(filter, zellij)?),
        None => None,
    };

    let output_panes: Vec<Pane> = panes
        .iter()
        .filter(|p| p.is_selectable)
        .filter(|p| match tab_position_filter {
            Some(pos) => p.tab_position == pos,
            None => true,
        })
        .map(|p| Pane {
            id: if p.is_plugin {
                format!("plugin_{}", p.id)
            } else {
                format!("terminal_{}", p.id)
            },
            command: p.pane_command.clone(),
            cwd: p.pane_cwd.clone(),
            title: if p.title.is_empty() {
                None
            } else {
                Some(p.title.clone())
            },
            focused: p.is_focused,
            floating: p.is_floating,
            tab_id: p.tab_id,
            tab_name: p.tab_name.clone(),
        })
        .collect();

    Ok(PanesOutput {
        panes: output_panes,
    })
}

pub fn read(
    target: &str,
    full: bool,
    ansi: bool,
    zellij: &dyn ZellijRunner,
) -> Result<PaneContent, ZjctlError> {
    let pane_id = resolve_pane(target, zellij)?;

    let mut args = vec!["dump-screen", "--pane-id"];
    let pane_id_ref = pane_id.as_str();
    args.push(pane_id_ref);
    if full {
        args.push("--full");
    }
    if ansi {
        args.push("--ansi");
    }

    let content = zellij.run_action(&args)?;

    Ok(PaneContent { pane_id, content })
}

pub fn write(
    target: &str,
    text: &str,
    zellij: &dyn ZellijRunner,
    dry_run: bool,
    no_guard: bool,
) -> Result<(), ZjctlError> {
    let pane_id = resolve_pane(target, zellij)?;
    check_self_write(&pane_id, no_guard)?;

    if dry_run {
        return Ok(());
    }

    zellij.run_action(&["write-chars", "--pane-id", &pane_id, text])?;
    Ok(())
}

pub fn send_keys(
    target: &str,
    keys: &[String],
    zellij: &dyn ZellijRunner,
    dry_run: bool,
    no_guard: bool,
) -> Result<(), ZjctlError> {
    let pane_id = resolve_pane(target, zellij)?;
    check_self_write(&pane_id, no_guard)?;

    if dry_run {
        return Ok(());
    }

    let mut args = vec!["send-keys", "--pane-id", &pane_id];
    let key_refs: Vec<&str> = keys.iter().map(|k| k.as_str()).collect();
    args.extend(&key_refs);
    zellij.run_action(&args)?;
    Ok(())
}

pub fn focus(target: &str, zellij: &dyn ZellijRunner, dry_run: bool) -> Result<(), ZjctlError> {
    let pane_id = resolve_pane(target, zellij)?;

    if dry_run {
        return Ok(());
    }

    zellij.run_action(&["focus-pane-id", &pane_id])?;
    Ok(())
}

#[allow(clippy::too_many_arguments)]
pub fn open_pane(
    direction: Option<String>,
    floating: bool,
    name: Option<String>,
    cwd: Option<String>,
    tab_id: Option<u32>,
    command: Vec<String>,
    zellij: &dyn ZellijRunner,
    dry_run: bool,
) -> Result<String, ZjctlError> {
    let mut args: Vec<String> = vec!["new-pane".to_string()];
    if let Some(ref d) = direction {
        args.push("--direction".to_string());
        args.push(d.clone());
    }
    if floating {
        args.push("--floating".to_string());
    }
    if let Some(ref n) = name {
        args.push("--name".to_string());
        args.push(n.clone());
    }
    if let Some(ref c) = cwd {
        args.push("--cwd".to_string());
        args.push(c.clone());
    }
    if let Some(id) = tab_id {
        args.push("--tab-id".to_string());
        args.push(id.to_string());
    }
    if !command.is_empty() {
        args.push("--".to_string());
        args.extend(command);
    }

    let args_refs: Vec<&str> = args.iter().map(|s| s.as_str()).collect();

    if dry_run {
        return Ok("dry_run".to_string());
    }

    let output = zellij.run_action(&args_refs)?;
    let pane_id = output.trim().to_string();
    Ok(pane_id)
}
