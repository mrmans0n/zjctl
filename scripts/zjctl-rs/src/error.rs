use serde::Serialize;
use std::fmt;

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    UnknownCommand,
    MissingArgument,
    InvalidTarget,
    ZellijError,
    SelfWriteBlocked,
    NotInSession,
    ParseError,
}

#[derive(Debug, Clone, Serialize)]
pub struct ZjctlError {
    pub error: ErrorCode,
    pub message: String,
    pub exit_code: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(rename = "self", skip_serializing_if = "Option::is_none")]
    pub self_pane: Option<String>,
}

impl ZjctlError {
    pub fn new(error: ErrorCode, message: impl Into<String>) -> Self {
        let exit_code = match &error {
            ErrorCode::UnknownCommand | ErrorCode::MissingArgument | ErrorCode::InvalidTarget => 1,
            ErrorCode::ZellijError => 2,
            ErrorCode::SelfWriteBlocked => 3,
            ErrorCode::NotInSession => 4,
            ErrorCode::ParseError => 5,
        };
        Self {
            error,
            message: message.into(),
            exit_code,
            command: None,
            target: None,
            self_pane: None,
        }
    }

    pub fn unknown_command(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::UnknownCommand, message)
    }

    pub fn missing_argument(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::MissingArgument, message)
    }

    pub fn invalid_target(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::InvalidTarget, message)
    }

    pub fn zellij_error(message: impl Into<String>, command: impl Into<String>) -> Self {
        let mut err = Self::new(ErrorCode::ZellijError, message);
        err.command = Some(command.into());
        err
    }

    pub fn self_write_blocked(target: String, self_pane: String) -> Self {
        let message = format!(
            "Refusing to write to own pane ({}). Use --no-guard to override.",
            self_pane
        );
        let mut err = Self::new(ErrorCode::SelfWriteBlocked, message);
        err.target = Some(target);
        err.self_pane = Some(self_pane);
        err
    }

    pub fn not_in_session() -> Self {
        Self::new(
            ErrorCode::NotInSession,
            "Not in a zellij session. Set ZELLIJ_SESSION_NAME or use --session.",
        )
    }

    pub fn parse_error(message: impl Into<String>) -> Self {
        Self::new(ErrorCode::ParseError, message)
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            format!(
                r#"{{"error":"{}","message":"serialization failed","exit_code":{}}}"#,
                "parse_error", self.exit_code
            )
        })
    }
}

impl fmt::Display for ZjctlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_json())
    }
}

impl std::error::Error for ZjctlError {}
