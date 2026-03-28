mod app;
mod client;
mod cli;
mod config;
mod daemon;
mod domain;
mod errors;
mod hypr;
mod ipc;
mod state;
mod usecases;

use std::process::ExitCode;

use clap::Parser;

use app::App;
use cli::{Cli, Command, ServeCommand};
use ipc::{AppCommand, ControlCommand};

fn to_app_command(command: Command) -> Option<AppCommand> {
    match command {
        Command::Serve { .. } => None,
        Command::Register { id, selector } => Some(AppCommand::Register {
            id,
            class: selector.class,
            title: selector.title,
        }),
        Command::Hide { id, selector } => Some(AppCommand::Hide {
            id,
            class: selector.class,
            title: selector.title,
        }),
        Command::Show { id, selector } => Some(AppCommand::Show {
            id,
            class: selector.class,
            title: selector.title,
        }),
        Command::Toggle { id, selector } => Some(AppCommand::Toggle {
            id,
            class: selector.class,
            title: selector.title,
        }),
        Command::Status { id, selector } => Some(AppCommand::Status {
            id,
            class: selector.class,
            title: selector.title,
        }),
        Command::List => Some(AppCommand::List),
        Command::Unregister { id } => Some(AppCommand::Unregister { id }),
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let outcome = match cli.command {
        Command::Serve { command } => match command {
            ServeCommand::Start { config } => daemon::serve_foreground(config),
            ServeCommand::Stop => client::send_control(ControlCommand::Shutdown),
            ServeCommand::Status => client::send_control(ControlCommand::Status),
            ServeCommand::Reload => client::send_control(ControlCommand::ReloadConfig),
        },
        command => {
            let app_command = match to_app_command(command) {
                Some(command) => command,
                None => {
                    return ExitCode::from(1);
                }
            };

            if cli.local {
                let mut app = App::default();
                app.execute(app_command)
            } else {
                client::send_app_command(app_command)
            }
        }
    };

    match outcome {
        Ok(message) => {
            println!("{message}");
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
            ExitCode::from(1)
        }
    }
}
