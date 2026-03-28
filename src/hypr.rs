use std::process::Command;

use serde::Deserialize;

use crate::domain::{WindowInfo, WindowSelector};
use crate::errors::AppError;

pub trait HyprGateway {
    fn list_clients(&self) -> Result<Vec<WindowInfo>, AppError>;
    fn active_workspace_id(&self) -> Result<i32, AppError>;
    fn move_to_workspace(&self, workspace_id: i32, addresses: &[String]) -> Result<(), AppError>;
    fn move_to_workspace_target_silent(&self, workspace_target: &str, addresses: &[String]) -> Result<(), AppError>;
}

#[derive(Debug, Default)]
pub struct HyprctlGateway;

impl HyprctlGateway {
    fn hyprctl_json<T: for<'de> Deserialize<'de>>(&self, subcommand: &str) -> Result<T, AppError> {
        let output = Command::new("hyprctl").args(["-j", subcommand]).output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            return Err(AppError::CommandFailed(stderr));
        }

        serde_json::from_slice(&output.stdout).map_err(AppError::from)
    }

    fn dispatch_move(
        &self,
        verb: &str,
        workspace_target: &str,
        addresses: &[String],
    ) -> Result<(), AppError> {
        for address in addresses {
            let target = format!("{workspace_target},address:{address}");
            let output = Command::new("hyprctl")
                .args(["dispatch", verb, &target])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                return Err(AppError::CommandFailed(stderr));
            }
        }

        Ok(())
    }

    pub fn find_matching_windows(&self, selector: &WindowSelector) -> Result<Vec<WindowInfo>, AppError> {
        let clients = self.list_clients()?;
        let matches = clients
            .into_iter()
            .filter(|window| selector.matches(window))
            .collect::<Vec<_>>();

        Ok(matches)
    }
}

impl HyprGateway for HyprctlGateway {
    fn list_clients(&self) -> Result<Vec<WindowInfo>, AppError> {
        #[derive(Debug, Deserialize)]
        struct ClientWorkspace {
            name: String,
        }

        #[derive(Debug, Deserialize)]
        struct ClientRaw {
            address: String,
            class: String,
            title: String,
            workspace: ClientWorkspace,
        }

        let raw_clients: Vec<ClientRaw> = self.hyprctl_json("clients")?;

        Ok(raw_clients
            .into_iter()
            .map(|raw| WindowInfo {
                address: raw.address,
                class: raw.class,
                title: raw.title,
                workspace_name: raw.workspace.name,
            })
            .collect())
    }

    fn active_workspace_id(&self) -> Result<i32, AppError> {
        #[derive(Debug, Deserialize)]
        struct ActiveWorkspace {
            id: i32,
        }

        let active: ActiveWorkspace = self.hyprctl_json("activeworkspace")?;
        Ok(active.id)
    }

    fn move_to_workspace(&self, workspace_id: i32, addresses: &[String]) -> Result<(), AppError> {
        self.dispatch_move("movetoworkspace", &workspace_id.to_string(), addresses)
    }

    fn move_to_workspace_target_silent(&self, workspace_target: &str, addresses: &[String]) -> Result<(), AppError> {
        self.dispatch_move("movetoworkspacesilent", workspace_target, addresses)
    }
}
