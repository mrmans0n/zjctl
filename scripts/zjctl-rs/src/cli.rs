use clap::{Parser, Subcommand};

use crate::output::OutputFormat;

#[derive(Parser)]
#[command(
    name = "zjctl",
    about = "JSON-first CLI wrapper around zellij for AI agents"
)]
pub struct Cli {
    #[arg(short = 'f', long, default_value = "json", value_enum)]
    pub format: OutputFormat,

    #[arg(short = 'n', long)]
    pub dry_run: bool,

    #[arg(short = 'q', long)]
    pub quiet: bool,

    #[arg(short = 's', long)]
    pub session: Option<String>,

    #[arg(long)]
    pub no_guard: bool,

    #[command(subcommand)]
    pub resource: Resource,
}

#[derive(Subcommand)]
pub enum Resource {
    /// Session management
    Sessions {
        #[command(subcommand)]
        verb: SessionsVerb,
    },
    /// Tab management
    Tabs {
        #[command(subcommand)]
        verb: TabsVerb,
    },
    /// Pane management
    Panes {
        #[command(subcommand)]
        verb: PanesVerb,
    },
}

#[derive(Subcommand)]
pub enum SessionsVerb {
    /// List all zellij sessions
    List,
}

#[derive(Subcommand)]
pub enum TabsVerb {
    /// List all tabs in the session
    List,
    /// Focus a tab by index or name
    Focus {
        /// Tab index (0-based) or name
        tab: String,
    },
    /// Open a new tab
    Open {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        layout: Option<String>,
        /// Command to run in the new tab
        #[arg(last = true)]
        command: Vec<String>,
    },
}

#[derive(Subcommand)]
pub enum PanesVerb {
    /// List panes, optionally filtered by tab
    List {
        #[arg(long)]
        tab: Option<String>,
    },
    /// Read pane screen content
    Read {
        /// Pane ID (terminal_N, N, or title)
        pane: String,
        #[arg(long)]
        full: bool,
        #[arg(long)]
        ansi: bool,
    },
    /// Write text to a pane
    Write {
        /// Pane ID (terminal_N, N, or title)
        pane: String,
        /// Text to write
        text: String,
    },
    /// Send key sequences to a pane
    SendKeys {
        /// Pane ID (terminal_N, N, or title)
        pane: String,
        /// Key names (e.g., Enter, Ctrl-c)
        keys: Vec<String>,
    },
    /// Focus a pane
    Focus {
        /// Pane ID (terminal_N, N, or title)
        pane: String,
    },
    /// Open a new pane
    Open {
        #[arg(long)]
        direction: Option<String>,
        #[arg(long)]
        floating: bool,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        cwd: Option<String>,
        #[arg(long)]
        tab_id: Option<u32>,
        /// Command to run in the new pane
        #[arg(last = true)]
        command: Vec<String>,
    },
}
