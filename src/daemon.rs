use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::{UnixListener, UnixStream};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::app::App;
use crate::config;
use crate::errors::AppError;
use crate::ipc::{ControlCommand, Request, Response, lock_file_path, pid_file_path, socket_path};

struct RuntimeFilesGuard {
    socket_path: PathBuf,
    pid_path: PathBuf,
    lock_path: PathBuf,
}

impl RuntimeFilesGuard {
    fn cleanup(&self) {
        let _ = std::fs::remove_file(&self.socket_path);
        let _ = std::fs::remove_file(&self.pid_path);
        let _ = std::fs::remove_file(&self.lock_path);
    }
}

impl Drop for RuntimeFilesGuard {
    fn drop(&mut self) {
        self.cleanup();
    }
}

fn process_alive(pid: u32) -> bool {
    PathBuf::from(format!("/proc/{pid}")).exists()
}

fn read_pid_file(path: &PathBuf) -> Result<Option<u32>, AppError> {
    if !path.exists() {
        return Ok(None);
    }

    let text = std::fs::read_to_string(path)?;
    let pid = text.trim().parse::<u32>().map_err(|err| {
        AppError::Ipc(format!("invalid pid file format in {}: {err}", path.display()))
    })?;
    Ok(Some(pid))
}

fn remove_if_exists(path: &PathBuf) -> Result<(), AppError> {
    if path.exists() {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

fn ensure_single_instance(
    socket_path: &PathBuf,
    pid_path: &PathBuf,
    lock_path: &PathBuf,
) -> Result<(), AppError> {
    if let Some(pid) = read_pid_file(pid_path)? {
        if process_alive(pid) {
            return Err(AppError::Ipc(format!(
                "daemon already running with pid {pid}"
            )));
        }

        // Stale files from a crashed daemon.
        remove_if_exists(pid_path)?;
        remove_if_exists(lock_path)?;
        remove_if_exists(socket_path)?;
        return Ok(());
    }

    if socket_path.exists() {
        match UnixStream::connect(socket_path) {
            Ok(_) => {
                return Err(AppError::Ipc(format!(
                    "daemon already running on {}",
                    socket_path.display()
                )));
            }
            Err(_) => {
                remove_if_exists(socket_path)?;
            }
        }
    }

    if lock_path.exists() {
        remove_if_exists(lock_path)?;
    }

    Ok(())
}

fn create_runtime_files(pid_path: &PathBuf, lock_path: &PathBuf) -> Result<(), AppError> {
    let pid = std::process::id();
    std::fs::write(pid_path, format!("{pid}\n"))?;

    let started_at = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|err| AppError::Ipc(format!("failed to read system clock: {err}")))?
        .as_secs();
    let lock_content = format!("pid={pid}\nstarted_at={started_at}\n");
    std::fs::write(lock_path, lock_content)?;

    Ok(())
}

fn map_error_code(err: &AppError) -> u16 {
    match err {
        AppError::InvalidSelector(_) | AppError::Config(_) => 400,
        AppError::GroupNotFound(_) => 404,
        AppError::NoWindowsFound(_) => 409,
        _ => 500,
    }
}

fn handle_request(
    app: &mut App,
    request: Request,
    config_path: &Option<PathBuf>,
    keep_running: &mut bool,
) -> Response {
    match request {
        Request::App(command) => match app.execute(command) {
            Ok(message) => Response::ok(message),
            Err(err) => Response::err(map_error_code(&err), err.to_string()),
        },
        Request::Control(command) => match command {
            ControlCommand::Status => {
                let stats = app.state_stats();
                Response::ok(format!(
                    "running: total_groups={} cli_groups={} config_groups={} config_path={}",
                    stats.0,
                    stats.1,
                    stats.2,
                    config_path
                        .as_ref()
                        .map(|p| p.display().to_string())
                        .unwrap_or_else(|| "none".to_string())
                ))
            }
            ControlCommand::ReloadConfig => {
                let Some(path) = config_path.as_ref() else {
                    return Response::err(400, "no config file set for daemon");
                };

                match config::load_groups(path) {
                    Ok(groups) => {
                        app.apply_config_groups(groups);
                        Response::ok(format!("config reloaded from {}", path.display()))
                    }
                    Err(err) => Response::err(map_error_code(&err), err.to_string()),
                }
            }
            ControlCommand::Shutdown => {
                *keep_running = false;
                Response::ok("daemon shutting down")
            }
        },
    }
}

fn process_stream(
    stream: UnixStream,
    app: &mut App,
    config_path: &Option<PathBuf>,
    keep_running: &mut bool,
) -> Result<(), AppError> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut line = String::new();
    let read = reader.read_line(&mut line)?;
    if read == 0 {
        return Ok(());
    }

    let response = match serde_json::from_str::<Request>(line.trim_end()) {
        Ok(request) => handle_request(app, request, config_path, keep_running),
        Err(err) => Response::err(400, format!("invalid request: {err}")),
    };

    let mut stream = stream;
    let payload = serde_json::to_string(&response)?;
    stream.write_all(payload.as_bytes())?;
    stream.write_all(b"\n")?;
    Ok(())
}

pub fn serve_foreground(config_path: Option<String>) -> Result<String, AppError> {
    let socket = socket_path();
    let pid_path = pid_file_path();
    let lock_path = lock_file_path();

    ensure_single_instance(&socket, &pid_path, &lock_path)?;

    let listener = UnixListener::bind(&socket)?;
    create_runtime_files(&pid_path, &lock_path)?;
    let runtime_guard = RuntimeFilesGuard {
        socket_path: socket.clone(),
        pid_path,
        lock_path,
    };

    let mut app = App::default();
    let resolved_config_path = config_path.map(PathBuf::from);

    if let Some(path) = resolved_config_path.as_ref() {
        let groups = config::load_groups(path)?;
        app.apply_config_groups(groups);
    }

    let mut keep_running = true;
    while keep_running {
        let (stream, _) = listener.accept()?;
        process_stream(stream, &mut app, &resolved_config_path, &mut keep_running)?;
    }

    runtime_guard.cleanup();
    Ok("daemon stopped".to_string())
}
