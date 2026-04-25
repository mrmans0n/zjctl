use std::process::Command;

use crate::error::ZjctlError;

pub trait ZellijRunner {
    fn run_action(&self, args: &[&str]) -> Result<String, ZjctlError>;
    fn run_toplevel(&self, args: &[&str]) -> Result<String, ZjctlError>;
}

pub struct RealZellij {
    pub session: Option<String>,
}

impl RealZellij {
    fn run_command(&self, cmd_args: &[&str]) -> Result<String, ZjctlError> {
        let mut cmd = Command::new("zellij");
        cmd.args(cmd_args);

        let output = cmd.output().map_err(|e| {
            ZjctlError::zellij_error(
                format!("Failed to execute zellij: {}", e),
                format!("zellij {}", cmd_args.join(" ")),
            )
        })?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(ZjctlError::zellij_error(
                stderr,
                format!("zellij {}", cmd_args.join(" ")),
            ));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

impl ZellijRunner for RealZellij {
    fn run_action(&self, args: &[&str]) -> Result<String, ZjctlError> {
        let mut cmd_args: Vec<&str> = Vec::new();
        if let Some(ref s) = self.session {
            cmd_args.extend_from_slice(&["-s", s]);
        }
        cmd_args.push("action");
        cmd_args.extend_from_slice(args);
        self.run_command(&cmd_args)
    }

    fn run_toplevel(&self, args: &[&str]) -> Result<String, ZjctlError> {
        let mut cmd_args: Vec<&str> = Vec::new();
        if let Some(ref s) = self.session {
            cmd_args.extend_from_slice(&["-s", s]);
        }
        cmd_args.extend_from_slice(args);
        self.run_command(&cmd_args)
    }
}
