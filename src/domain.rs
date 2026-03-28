use regex::Regex;

use crate::errors::AppError;

#[derive(Debug, Clone)]
pub struct WindowSelector {
    pub class_pattern: Option<String>,
    pub title_pattern: Option<String>,
}

impl WindowSelector {
    pub fn new(class_pattern: Option<String>, title_pattern: Option<String>) -> Result<Self, AppError> {
        if class_pattern.is_none() && title_pattern.is_none() {
            return Err(AppError::InvalidSelector(
                "at least one selector is required (--class or --title)".to_string(),
            ));
        }

        if let Some(pattern) = class_pattern.as_ref() {
            Regex::new(pattern).map_err(|err| AppError::InvalidSelector(format!("invalid --class regex: {err}")))?;
        }

        if let Some(pattern) = title_pattern.as_ref() {
            Regex::new(pattern).map_err(|err| AppError::InvalidSelector(format!("invalid --title regex: {err}")))?;
        }

        Ok(Self {
            class_pattern,
            title_pattern,
        })
    }

    pub fn matches(&self, window: &WindowInfo) -> bool {
        let class_ok = self
            .class_pattern
            .as_ref()
            .is_none_or(|pattern| Regex::new(pattern).is_ok_and(|regex| regex.is_match(&window.class)));

        let title_ok = self
            .title_pattern
            .as_ref()
            .is_none_or(|pattern| Regex::new(pattern).is_ok_and(|regex| regex.is_match(&window.title)));

        class_ok && title_ok
    }
}

#[derive(Debug, Clone)]
pub struct ScratchpadGroup {
    pub id: String,
    pub selector: WindowSelector,
    pub source: GroupSource,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GroupSource {
    Cli,
    Config,
}

#[derive(Debug, Clone)]
pub struct WindowInfo {
    pub address: String,
    pub class: String,
    pub title: String,
    pub workspace_name: String,
}
