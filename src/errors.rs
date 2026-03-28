use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum AppError {
    InvalidSelector(String),
    GroupNotFound(String),
    NoWindowsFound(String),
    DaemonNotRunning(String),
    Ipc(String),
    Config(String),
    Io(std::io::Error),
    Json(serde_json::Error),
    TomlDe(toml::de::Error),
    CommandFailed(String),
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::InvalidSelector(message) => write!(f, "invalid selector: {message}"),
            AppError::GroupNotFound(id) => write!(f, "scratchpad group not found: {id}"),
            AppError::NoWindowsFound(message) => write!(f, "no windows matched selector: {message}"),
            AppError::DaemonNotRunning(message) => write!(f, "daemon is not running: {message}"),
            AppError::Ipc(message) => write!(f, "ipc error: {message}"),
            AppError::Config(message) => write!(f, "config error: {message}"),
            AppError::Io(err) => write!(f, "i/o error: {err}"),
            AppError::Json(err) => write!(f, "json parse error: {err}"),
            AppError::TomlDe(err) => write!(f, "toml parse error: {err}"),
            AppError::CommandFailed(message) => write!(f, "hyprctl command failed: {message}"),
        }
    }
}

impl std::error::Error for AppError {}

impl From<std::io::Error> for AppError {
    fn from(value: std::io::Error) -> Self {
        AppError::Io(value)
    }
}

impl From<serde_json::Error> for AppError {
    fn from(value: serde_json::Error) -> Self {
        AppError::Json(value)
    }
}

impl From<toml::de::Error> for AppError {
    fn from(value: toml::de::Error) -> Self {
        AppError::TomlDe(value)
    }
}
