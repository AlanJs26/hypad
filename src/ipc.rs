use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "kebab-case")]
pub enum AppCommand {
    Register {
        id: String,
        class: Option<String>,
        title: Option<String>,
    },
    Hide {
        id: String,
        class: Option<String>,
        title: Option<String>,
    },
    Show {
        id: String,
        class: Option<String>,
        title: Option<String>,
    },
    Toggle {
        id: String,
        class: Option<String>,
        title: Option<String>,
    },
    Status {
        id: String,
        class: Option<String>,
        title: Option<String>,
    },
    List,
    Unregister {
        id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "control", rename_all = "kebab-case")]
pub enum ControlCommand {
    Status,
    ReloadConfig,
    Shutdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload", rename_all = "kebab-case")]
pub enum Request {
    App(AppCommand),
    Control(ControlCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub success: bool,
    pub code: u16,
    pub message: String,
}

impl Response {
    pub fn ok(message: impl Into<String>) -> Self {
        Self {
            success: true,
            code: 0,
            message: message.into(),
        }
    }

    pub fn err(code: u16, message: impl Into<String>) -> Self {
        Self {
            success: false,
            code,
            message: message.into(),
        }
    }
}

pub fn socket_path() -> PathBuf {
    runtime_dir().join(base_file_name("sock"))
}

pub fn pid_file_path() -> PathBuf {
    runtime_dir().join(base_file_name("pid"))
}

pub fn lock_file_path() -> PathBuf {
    runtime_dir().join(base_file_name("lock"))
}

fn runtime_dir() -> PathBuf {
    if let Ok(path) = std::env::var("XDG_RUNTIME_DIR") {
        return PathBuf::from(path);
    }

    PathBuf::from("/tmp")
}

fn base_file_name(ext: &str) -> String {
    if std::env::var("XDG_RUNTIME_DIR").is_ok() {
        return format!("hypr-scratchpad.{ext}");
    }

    let user = std::env::var("USER").unwrap_or_else(|_| "unknown".to_string());
    format!("hypr-scratchpad-{user}.{ext}")
}
