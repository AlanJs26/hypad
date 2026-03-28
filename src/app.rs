use crate::domain::WindowSelector;
use crate::errors::AppError;
use crate::hypr::HyprctlGateway;
use crate::ipc::AppCommand;
use crate::state::RuntimeState;
use crate::usecases;

#[derive(Debug, Default)]
pub struct App {
    state: RuntimeState,
    gateway: HyprctlGateway,
}

impl App {
    pub fn execute(&mut self, command: AppCommand) -> Result<String, AppError> {
        match command {
            AppCommand::Register { id, class, title } => {
                let selector = selector_from_options(class, title, true)?
                    .ok_or_else(|| {
                        AppError::InvalidSelector(
                            "at least one selector is required (--class or --title)".to_string(),
                        )
                    })?;
                usecases::register_group(&mut self.state, id, selector)
            }
            AppCommand::Hide { id, class, title } => {
                let selector = selector_from_options(class, title, false)?;
                usecases::hide_group(&self.gateway, &self.state, &id, selector)
            }
            AppCommand::Show { id, class, title } => {
                let selector = selector_from_options(class, title, false)?;
                usecases::show_group(&self.gateway, &self.state, &id, selector)
            }
            AppCommand::Toggle { id, class, title } => {
                let selector = selector_from_options(class, title, false)?;
                usecases::toggle_group(&self.gateway, &self.state, &id, selector)
            }
            AppCommand::Status { id, class, title } => {
                let selector = selector_from_options(class, title, false)?;
                usecases::status_group(&self.gateway, &self.state, &id, selector)
            }
            AppCommand::List => Ok(usecases::list_groups(&self.state)),
            AppCommand::Unregister { id } => usecases::unregister_group(&mut self.state, &id),
        }
    }

    pub fn apply_config_groups(&mut self, groups: Vec<(String, WindowSelector)>) {
        self.state.apply_config_snapshot(groups);
    }

    pub fn state_stats(&self) -> (usize, usize, usize) {
        self.state.source_counts()
    }
}

fn selector_from_options(
    class: Option<String>,
    title: Option<String>,
    required: bool,
) -> Result<Option<WindowSelector>, AppError> {
    if class.is_none() && title.is_none() {
        if required {
            return Err(AppError::InvalidSelector(
                "at least one selector is required (--class or --title)".to_string(),
            ));
        }
        return Ok(None);
    }

    Ok(Some(WindowSelector::new(class, title)?))
}
