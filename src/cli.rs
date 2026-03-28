use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "hypad")]
#[command(about = "CLI scratchpad manager for Hyprland")]
pub struct Cli {
    #[arg(long)]
    pub local: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Serve {
        #[command(subcommand)]
        command: ServeCommand,
    },
    Register {
        id: String,
        #[command(flatten)]
        selector: SelectorArgs,
    },
    Hide {
        id: String,
        #[command(flatten)]
        selector: SelectorArgs,
    },
    Show {
        id: String,
        #[command(flatten)]
        selector: SelectorArgs,
    },
    Toggle {
        id: String,
        #[command(flatten)]
        selector: SelectorArgs,
    },
    Status {
        id: String,
        #[command(flatten)]
        selector: SelectorArgs,
    },
    List,
    Unregister {
        id: String,
    },
}

#[derive(Debug, Subcommand)]
pub enum ServeCommand {
    Start {
        #[arg(long)]
        config: Option<String>,
    },
    Stop,
    Status,
    Reload,
}

#[derive(Debug, Clone, Args)]
pub struct SelectorArgs {
    #[arg(long)]
    pub class: Option<String>,
    #[arg(long)]
    pub title: Option<String>,
}
