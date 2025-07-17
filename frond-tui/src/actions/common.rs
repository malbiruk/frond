use crate::app::Mode;

#[derive(Debug, Clone)]
pub enum CommonAction {
    // Application control
    Quit,

    // Error handling
    ShowError(String),
    ClearError,

    ShowHelp(Mode),

    // Configuration
    UpdateConfig(crate::config::Config),
}

#[derive(Debug, Clone)]
pub struct ActionInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub available_in_modes: Vec<Mode>,
    pub requires_focus: bool,
}

impl CommonAction {
    pub fn info(&self) -> ActionInfo {
        match self {
            CommonAction::Quit => ActionInfo {
                id: "quit",
                name: "quit",
                description: "quit the application",
                available_in_modes: vec![Mode::Normal, Mode::Edit(crate::app::EditMode::Append)],
                requires_focus: false,
            },
            CommonAction::ShowError(_) => ActionInfo {
                id: "show_error",
                name: "show error",
                description: "display an error message",
                available_in_modes: vec![Mode::Normal, Mode::Edit(crate::app::EditMode::Append)],
                requires_focus: false,
            },
            CommonAction::ClearError => ActionInfo {
                id: "clear_error",
                name: "clear error",
                description: "clear the current error message",
                available_in_modes: vec![Mode::Normal, Mode::Edit(crate::app::EditMode::Append)],
                requires_focus: false,
            },
            CommonAction::UpdateConfig(_) => ActionInfo {
                id: "update_config",
                name: "update config",
                description: "update application configuration",
                available_in_modes: vec![Mode::Normal, Mode::Edit(crate::app::EditMode::Append)],
                requires_focus: false,
            },
            CommonAction::ShowHelp(_) => ActionInfo {
                id: "show_help",
                name: "help",
                description: "display help information",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
        }
    }
}
