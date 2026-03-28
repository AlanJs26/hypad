use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::time::Duration;

use crate::errors::AppError;
use crate::ipc::{AppCommand, ControlCommand, Request, Response, socket_path};

fn send_request(request: Request) -> Result<Response, AppError> {
    let socket = socket_path();
    let mut stream = UnixStream::connect(&socket).map_err(|err| {
        AppError::DaemonNotRunning(format!(
            "cannot connect to {} ({err})",
            socket.display()
        ))
    })?;

    stream.set_read_timeout(Some(Duration::from_secs(2)))?;
    stream.set_write_timeout(Some(Duration::from_secs(2)))?;

    let request_json = serde_json::to_string(&request)?;
    stream.write_all(request_json.as_bytes())?;
    stream.write_all(b"\n")?;

    let mut line = String::new();
    let mut reader = BufReader::new(stream);
    let read = reader.read_line(&mut line)?;
    if read == 0 {
        return Err(AppError::Ipc("empty response from daemon".to_string()));
    }

    let response: Response = serde_json::from_str(line.trim_end())?;
    Ok(response)
}

pub fn send_app_command(command: AppCommand) -> Result<String, AppError> {
    let response = send_request(Request::App(command))?;
    if response.success {
        return Ok(response.message);
    }

    Err(AppError::Ipc(response.message))
}

pub fn send_control(command: ControlCommand) -> Result<String, AppError> {
    let response = send_request(Request::Control(command))?;
    if response.success {
        return Ok(response.message);
    }

    Err(AppError::Ipc(response.message))
}
