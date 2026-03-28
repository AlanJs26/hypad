# hypad

Scratchpad CLI for Hyprland using one dedicated hidden `special workspace`.

The project hides windows in a single dedicated special workspace (`special:scratchpad-hidden`) and lets you control window groups by identifier (`id`) using class and/or title selectors (regex).

## Status

Working project in CLI + daemon (`serve`) mode with local IPC.

Implemented features:
- `register`, `hide`, `show`, `toggle`, `status`, `list`, `unregister`
- resident daemon mode: `serve start|status|reload|stop`
- load groups from TOML file
- lock/pid files to prevent multiple daemon instances
- single hidden special workspace strategy

## Requirements

- Linux
- Hyprland running
- `hyprctl` available in `PATH`
- Rust toolchain (cargo + rustc)

## Build

```bash
cargo build
```

Run the CLI during development:

```bash
cargo run -- <command>
```

## CLI Commands

Operational commands:

```bash
hypad register <id> --class <regex> [--title <regex>]
hypad hide <id> [--class <regex>] [--title <regex>]
hypad show <id> [--class <regex>] [--title <regex>]
hypad toggle <id> [--class <regex>] [--title <regex>]
hypad status <id> [--class <regex>] [--title <regex>]
hypad list
hypad unregister <id>
```

Daemon commands:

```bash
hypad serve start [--config hypad.toml]
hypad serve status
hypad serve reload
hypad serve stop
```

Local mode (without daemon):

```bash
hypad --local <operational-command>
```

Notes:
- without `--local`, operational commands try to talk to the daemon through the socket.
- if the daemon is not running, the CLI returns an error suggesting `serve start`.

## Recommended Flow

1. Start the daemon:

```bash
cargo run -- serve start --config hypad.toml
```

2. In another terminal, run commands:

```bash
cargo run -- list
cargo run -- register notes --class 'obsidian|code'
cargo run -- toggle notes
```

3. Stop the daemon when needed:

```bash
cargo run -- serve stop
```

## TOML Configuration

Example (`hypad.toml`):

```toml
version = 1

[groups.calc]
class = "qalculate-gtk"

[groups.browser]
class = "firefox|chromium"

[groups.editor]
class = "code|nvim"
title = "work|project"
```

Current schema rules:
- `version` is optional, but when present it must be `1`
- each group is defined under `[groups.<id>]`
- each group must have at least one selector: `class` and/or `title`
- `class` and `title` are validated as regex on load

When using TOML:
- `serve start --config <file>` loads groups on startup
- `serve reload` reloads the same configuration file

## Lock/PID and Socket

The daemon creates runtime files on startup:
- socket IPC: `hypad.sock`
- pid file: `hypad.pid`
- lock file: `hypad.lock`

Path resolution:
- if `XDG_RUNTIME_DIR` exists: `${XDG_RUNTIME_DIR}/hypad.{sock|pid|lock}`
- fallback: `/tmp/hypad-$USER.{sock|pid|lock}`

Behavior:
- if a live PID exists, a second daemon instance is blocked
- if stale files are detected, they are cleaned on startup
- on graceful shutdown (`serve stop`), runtime files are removed

## How It Works (Summary)

- `hide`: moves group windows to `special:scratchpad-hidden` using `movetoworkspacesilent`
- `show`: moves group windows to the active workspace
- `toggle`: switches between `hide/show` based on current window state
- window matching uses regex on `class` and `title` via `hyprctl -j clients`

## Project Structure

```text
src/
  main.rs
  app.rs
  cli.rs
  client.rs
  config.rs
  daemon.rs
  domain.rs
  errors.rs
  hypr.rs
  ipc.rs
  state.rs
  usecases.rs
```

## Current Limitations

- daemon runs in foreground (no `start --background` yet)
- no disk persistence for dynamic CLI state (daemon memory only)
- no automatic TOML file watch (`reload` is manual)
- all groups share the same hidden special workspace while hidden

## Troubleshooting

Daemon is not responding:

```bash
cargo run -- serve status
```

If it is not running, start it:

```bash
cargo run -- serve start --config hypad.toml
```

Invalid selector error:
- review regex in `--class`, `--title`, or TOML

No windows found:
- check `hyprctl -j clients` and adjust regex to real `class/title` values

## License

Not defined in this repository yet.
