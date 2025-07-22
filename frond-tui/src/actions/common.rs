use crate::app::Mode;
use strum::EnumIter;

// Schema - parameter-less templates for registry
#[derive(Debug, Clone, EnumIter)]
pub enum CommonActionSchema {
    Quit,
    ShowHelp,
}

impl CommonActionSchema {
    pub fn id(&self) -> &'static str {
        match self {
            Self::Quit => "quit",
            Self::ShowHelp => "show_help",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::Quit => "quit",
            Self::ShowHelp => "help",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::Quit => "quit the application",
            Self::ShowHelp => "display help information",
        }
    }

    pub fn requires_focus(&self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub enum CommonAction {
    Quit,
    ShowHelp(Mode),
}
