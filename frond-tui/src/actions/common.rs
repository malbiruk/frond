use crate::app::Mode;
use crate::config::Config;
use strum::EnumIter;

// Schema - parameter-less templates for registry
#[derive(Debug, Clone, EnumIter)]
pub enum CommonActionSchema {
    // Application control
    Quit,

    // Error handling
    ShowError,
    ClearError,

    // Help
    ShowHelp,

    // Configuration
    UpdateConfig,
}

impl CommonActionSchema {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Quit => "quit",
            Self::ShowError => "show_error",
            Self::ClearError => "clear_error",
            Self::ShowHelp => "show_help",
            Self::UpdateConfig => "update_config",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Quit => "quit",
            Self::ShowError => "show error",
            Self::ClearError => "clear error",
            Self::ShowHelp => "help",
            Self::UpdateConfig => "update config",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Quit => "quit the application",
            Self::ShowError => "display an error message",
            Self::ClearError => "clear the current error message",
            Self::ShowHelp => "display help information",
            Self::UpdateConfig => "update application configuration",
        }
    }

    pub fn requires_focus(&self) -> bool {
        // Common actions typically don't require focus
        false
    }
}

// Action - with real parameters for dispatch
#[derive(Debug, Clone)]
pub enum CommonAction {
    // Application control
    Quit,

    // Error handling
    ShowError(String),
    ClearError,

    // Help
    ShowHelp(Mode),

    // Configuration
    UpdateConfig(Config),
}

// ActionInfo struct for compatibility (can be removed later)
#[derive(Debug, Clone)]
pub struct ActionInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub available_in_modes: Vec<Mode>,
    pub requires_focus: bool,
}
