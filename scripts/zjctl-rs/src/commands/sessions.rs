use crate::cli::SessionsVerb;
use crate::error::ZjctlError;
use crate::models::{Session, SessionsOutput};
use crate::output::{emit, OutputFormat};
use crate::zellij::ZellijRunner;

pub fn run(
    verb: SessionsVerb,
    zellij: &dyn ZellijRunner,
    format: &OutputFormat,
) -> Result<(), ZjctlError> {
    match verb {
        SessionsVerb::List => {
            let output = list(zellij, format)?;
            emit(&output, format, |o| {
                for session in &o.sessions {
                    println!("{}", session.name);
                }
            });
            Ok(())
        }
    }
}

pub fn list(
    zellij: &dyn ZellijRunner,
    _format: &OutputFormat,
) -> Result<SessionsOutput, ZjctlError> {
    let raw = zellij.run_toplevel(&["list-sessions", "-n", "-s"])?;

    let sessions: Vec<Session> = raw
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| Session {
            name: line.to_string(),
        })
        .collect();

    Ok(SessionsOutput { sessions })
}
