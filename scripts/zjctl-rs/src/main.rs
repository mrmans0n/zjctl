use clap::Parser;
use zjctl::cli::{Cli, Resource};
use zjctl::commands;
use zjctl::error::ZjctlError;
use zjctl::output::OutputFormat;
use zjctl::zellij::RealZellij;

fn run(cli: Cli) -> Result<(), ZjctlError> {
    let format = if cli.quiet {
        OutputFormat::Quiet
    } else {
        cli.format
    };

    let session = cli
        .session
        .or_else(|| std::env::var("ZELLIJ_SESSION_NAME").ok());

    let zellij = RealZellij {
        session: session.clone(),
    };

    match cli.resource {
        Resource::Sessions { verb } => commands::sessions::run(verb, &zellij, &format),
        Resource::Tabs { verb } => {
            require_session(&session)?;
            commands::tabs::run(verb, &zellij, &format, cli.dry_run)
        }
        Resource::Panes { verb } => {
            require_session(&session)?;
            commands::panes::run(verb, &zellij, &format, cli.dry_run, cli.no_guard)
        }
    }
}

fn require_session(session: &Option<String>) -> Result<(), ZjctlError> {
    if session.is_none() {
        return Err(ZjctlError::not_in_session());
    }
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("{}", e.to_json());
        std::process::exit(e.exit_code);
    }
}
