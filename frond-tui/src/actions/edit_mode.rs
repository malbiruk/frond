use strum::EnumIter;

// Schema - parameter-less templates for registry
#[derive(Debug, Clone, EnumIter)]
pub enum EditModeActionSchema {
    ExitCurrentMode,
}

impl EditModeActionSchema {
    pub fn id(&self) -> &'static str {
        match self {
            Self::ExitCurrentMode => "exit_mode",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ExitCurrentMode => "exit mode",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ExitCurrentMode => "exit the current mode",
        }
    }

    pub fn requires_focus(&self) -> bool {
        // Most edit actions don't require message focus, just editor focus
        false
    }
}

#[derive(Debug, Clone)]
pub enum EditModeAction {
    ExitCurrentMode,
}
