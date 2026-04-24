use std::ffi::OsString;
use std::process::Command;

use anyhow::{Context, Result, bail};
use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "zjctl")]
#[command(about = "Agent-friendly programmatic control for Zellij")]
struct Cli {
    /// Optional Zellij session name to target.
    #[arg(long, global = true)]
    session: Option<String>,
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// List panes as JSON.
    Panes(PanesCommand),
    /// List tabs as JSON.
    Tabs(TabsCommand),
    /// Read pane output.
    Read(ReadCommand),
    /// Write text to a pane.
    Write(WriteCommand),
}

#[derive(Args, Debug)]
struct PanesCommand {
    #[command(subcommand)]
    command: PanesSubcommand,
}

#[derive(Subcommand, Debug)]
enum PanesSubcommand {
    /// List panes.
    List,
}

#[derive(Args, Debug)]
struct TabsCommand {
    #[command(subcommand)]
    command: TabsSubcommand,
}

#[derive(Subcommand, Debug)]
enum TabsSubcommand {
    /// List tabs.
    List,
}

#[derive(Args, Debug)]
struct ReadCommand {
    /// Pane ID, eg. terminal_2 or 2.
    #[arg(long)]
    pane: String,
    /// Include full scrollback.
    #[arg(long)]
    full: bool,
    /// Preserve ANSI escape codes.
    #[arg(long)]
    ansi: bool,
}

#[derive(Args, Debug)]
struct WriteCommand {
    /// Pane ID, eg. terminal_2 or 2.
    #[arg(long)]
    pane: String,
    /// Text to paste into the pane.
    #[arg(long)]
    text: String,
    /// Send Enter after pasting text.
    #[arg(long)]
    enter: bool,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Panes(command) => match command.command {
            PanesSubcommand::List => {
                run_zellij(cli.session.as_deref(), &["action", "list-panes", "--json"])
            }
        },
        Commands::Tabs(command) => match command.command {
            TabsSubcommand::List => {
                run_zellij(cli.session.as_deref(), &["action", "list-tabs", "--json"])
            }
        },
        Commands::Read(command) => {
            let mut args = vec![
                OsString::from("action"),
                OsString::from("dump-screen"),
                OsString::from("--pane-id"),
                OsString::from(command.pane),
            ];
            if command.full {
                args.push(OsString::from("--full"));
            }
            if command.ansi {
                args.push(OsString::from("--ansi"));
            }
            run_zellij_os(cli.session.as_deref(), &args)
        }
        Commands::Write(command) => {
            run_zellij_os(
                cli.session.as_deref(),
                &[
                    OsString::from("action"),
                    OsString::from("paste"),
                    OsString::from("--pane-id"),
                    OsString::from(command.pane.clone()),
                    OsString::from(command.text),
                ],
            )?;
            if command.enter {
                run_zellij_os(
                    cli.session.as_deref(),
                    &[
                        OsString::from("action"),
                        OsString::from("send-keys"),
                        OsString::from("--pane-id"),
                        OsString::from(command.pane),
                        OsString::from("Enter"),
                    ],
                )?;
            }
            Ok(())
        }
    }
}

fn run_zellij(session: Option<&str>, args: &[&str]) -> Result<()> {
    let args = args.iter().map(OsString::from).collect::<Vec<_>>();
    run_zellij_os(session, &args)
}

fn run_zellij_os(session: Option<&str>, args: &[OsString]) -> Result<()> {
    let mut command = Command::new("zellij");
    if let Some(session_name) = session {
        command.arg("--session").arg(session_name);
    }
    command.args(args);

    let status = command.status().with_context(|| {
        format!(
            "failed to launch zellij with args {}",
            format_args_for_error(args)
        )
    })?;

    if !status.success() {
        bail!(
            "zellij exited with status {status} while running {}",
            format_args_for_error(args)
        );
    }
    Ok(())
}

fn format_args_for_error(args: &[OsString]) -> String {
    args.iter()
        .map(|arg| arg.to_string_lossy().into_owned())
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn formats_args_for_error_messages() {
        let args = vec![
            OsString::from("action"),
            OsString::from("dump-screen"),
            OsString::from("--pane-id"),
            OsString::from("terminal_2"),
        ];
        assert_eq!(
            format_args_for_error(&args),
            "action dump-screen --pane-id terminal_2"
        );
    }
}
