use crate::domain::{WindowInfo, WindowSelector};
use crate::errors::AppError;
use crate::hypr::{HyprGateway, HyprctlGateway};
use crate::state::RuntimeState;
use std::process::{Command, Stdio};

pub const HIDDEN_SPECIAL_WORKSPACE: &str = "special:scratchpad-hidden";

fn is_hidden_workspace(workspace_name: &str) -> bool {
    workspace_name == HIDDEN_SPECIAL_WORKSPACE
}

fn resolve_selector(
    state: &RuntimeState,
    id: &str,
    selector: Option<WindowSelector>,
) -> Result<WindowSelector, AppError> {
    if let Some(selector) = selector {
        return Ok(selector);
    }

    state
        .get(id)
        .map(|group| group.selector.clone())
        .ok_or_else(|| AppError::GroupNotFound(id.to_string()))
}

fn resolve_windows(
    gateway: &HyprctlGateway,
    state: &RuntimeState,
    id: &str,
    selector: Option<WindowSelector>,
) -> Result<(WindowSelector, Vec<WindowInfo>), AppError> {
    let selector = resolve_selector(state, id, selector)?;
    let windows = gateway.find_matching_windows(&selector)?;
    Ok((selector, windows))
}

fn no_windows_detail(id: &str, selector: &WindowSelector) -> String {
    format!(
        "id={id}, class={:?}, title={:?}",
        selector.class_pattern, selector.title_pattern
    )
}

fn spawn_on_no_match(id: &str, action: &str, selector: &WindowSelector) -> Result<Option<String>, AppError> {
    let Some(command) = selector.on_no_match.as_ref() else {
        return Ok(None);
    };

    Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .map_err(|err| {
            AppError::CommandFailed(format!(
                "failed to spawn on-no-match command for '{id}' ({action}): {err}"
            ))
        })?;

    Ok(Some(format!(
        "no windows matched for '{id}' during '{action}', launched on-no-match command in background"
    )))
}

pub fn register_group(state: &mut RuntimeState, id: String, selector: WindowSelector) -> Result<String, AppError> {
    state.upsert_cli_group(id.clone(), selector);

    Ok(format!(
        "registered '{id}' (hidden target: {HIDDEN_SPECIAL_WORKSPACE})"
    ))
}

pub fn unregister_group(state: &mut RuntimeState, id: &str) -> Result<String, AppError> {
    if !state.remove(id) {
        return Err(AppError::GroupNotFound(id.to_string()));
    }

    Ok(format!("unregistered '{id}'"))
}

pub fn list_groups(state: &RuntimeState) -> String {
    let mut lines = vec![format!(
        "registered groups (runtime only, hidden target: {HIDDEN_SPECIAL_WORKSPACE}):"
    )];

    for group in state.list() {
        lines.push(format!(
            "- {} -> source={:?} class={:?} title={:?} on_no_match={:?}",
            group.id,
            group.source,
            group.selector.class_pattern,
            group.selector.title_pattern,
            group.selector.on_no_match
        ));
    }

    if lines.len() == 1 {
        lines.push("- (none)".to_string());
    }

    lines.join("\n")
}

pub fn hide_group(
    gateway: &HyprctlGateway,
    state: &RuntimeState,
    id: &str,
    selector: Option<WindowSelector>,
) -> Result<String, AppError> {
    let (selector, windows) = resolve_windows(gateway, state, id, selector)?;
    if windows.is_empty() {
        return Err(AppError::NoWindowsFound(no_windows_detail(id, &selector)));
    }

    let addresses = windows.into_iter().map(|w| w.address).collect::<Vec<_>>();

    gateway.move_to_workspace_target_silent(HIDDEN_SPECIAL_WORKSPACE, &addresses)?;
    Ok(format!(
        "hidden '{}' ({} window(s)) to {}",
        id,
        addresses.len(),
        HIDDEN_SPECIAL_WORKSPACE
    ))
}

pub fn show_group(
    gateway: &HyprctlGateway,
    state: &RuntimeState,
    id: &str,
    selector: Option<WindowSelector>,
) -> Result<String, AppError> {
    let (selector, windows) = resolve_windows(gateway, state, id, selector)?;
    if windows.is_empty() {
        if let Some(message) = spawn_on_no_match(id, "show", &selector)? {
            return Ok(message);
        }

        return Err(AppError::NoWindowsFound(no_windows_detail(id, &selector)));
    }

    let active_workspace = gateway.active_workspace_id()?;
    let addresses = windows.into_iter().map(|w| w.address).collect::<Vec<_>>();

    gateway.move_to_workspace(active_workspace, &addresses)?;
    Ok(format!(
        "shown '{}' ({} window(s)) on active workspace {}",
        id,
        addresses.len(),
        active_workspace
    ))
}

pub fn toggle_group(
    gateway: &HyprctlGateway,
    state: &RuntimeState,
    id: &str,
    selector: Option<WindowSelector>,
) -> Result<String, AppError> {
    let (selector, windows) = resolve_windows(gateway, state, id, selector)?;
    if windows.is_empty() {
        if let Some(message) = spawn_on_no_match(id, "toggle", &selector)? {
            return Ok(message);
        }

        return Err(AppError::NoWindowsFound(no_windows_detail(id, &selector)));
    }

    let addresses = windows.iter().map(|w| w.address.clone()).collect::<Vec<_>>();

    let all_hidden = windows
        .iter()
        .all(|window| is_hidden_workspace(&window.workspace_name));

    if all_hidden {
        let active_workspace = gateway.active_workspace_id()?;
        gateway.move_to_workspace(active_workspace, &addresses)?;
        Ok(format!(
            "toggled '{}' to visible ({} window(s)) on workspace {}",
            id,
            addresses.len(),
            active_workspace
        ))
    } else {
        gateway.move_to_workspace_target_silent(HIDDEN_SPECIAL_WORKSPACE, &addresses)?;
        Ok(format!(
            "toggled '{}' to hidden ({} window(s)) on {}",
            id,
            addresses.len(),
            HIDDEN_SPECIAL_WORKSPACE
        ))
    }
}

pub fn status_group(
    gateway: &HyprctlGateway,
    state: &RuntimeState,
    id: &str,
    selector: Option<WindowSelector>,
) -> Result<String, AppError> {
    let (selector, windows) = resolve_windows(gateway, state, id, selector)?;
    if windows.is_empty() {
        return Err(AppError::NoWindowsFound(no_windows_detail(id, &selector)));
    }

    let hidden = windows
        .iter()
        .filter(|window| is_hidden_workspace(&window.workspace_name))
        .count();

    let visible = windows.len().saturating_sub(hidden);

    Ok(format!(
        "status '{}' -> total={} visible={} hidden={} hidden_target={} class={:?} title={:?} on_no_match={:?}",
        id,
        windows.len(),
        visible,
        hidden,
        HIDDEN_SPECIAL_WORKSPACE,
        selector.class_pattern,
        selector.title_pattern,
        selector.on_no_match
    ))
}
