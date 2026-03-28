use std::collections::HashMap;
use std::path::Path;

use serde::Deserialize;

use crate::domain::WindowSelector;
use crate::errors::AppError;

#[derive(Debug, Deserialize)]
struct ConfigFile {
    version: Option<u32>,
    #[serde(default)]
    groups: HashMap<String, ConfigGroup>,
}

#[derive(Debug, Deserialize)]
struct ConfigGroup {
    class: Option<String>,
    title: Option<String>,
    on_no_match: Option<String>,
}

pub fn load_groups(path: &Path) -> Result<Vec<(String, WindowSelector)>, AppError> {
    let content = std::fs::read_to_string(path)?;
    let parsed: ConfigFile = toml::from_str(&content)?;

    if let Some(version) = parsed.version
        && version != 1
    {
        return Err(AppError::Config(format!(
            "unsupported config version '{version}', expected 1"
        )));
    }

    let mut groups = Vec::with_capacity(parsed.groups.len());
    for (id, group) in parsed.groups {
        let selector = WindowSelector::new(group.class, group.title, group.on_no_match)
            .map_err(|err| AppError::Config(format!("group '{id}': {err}")))?;
        groups.push((id, selector));
    }

    Ok(groups)
}
