use super::common::ActionInfo;
use crate::app::{EditMode, Mode};

#[derive(Debug, Clone)]
pub enum EditModeAction {
    // Mode transitions
    ExitCurrentMode,
    // Text editing actions (these could be expanded later)
    // For now, we just have the exit action since text editing
    // is likely handled by the text input component
}

impl EditModeAction {
    pub fn info(&self) -> ActionInfo {
        match self {
            EditModeAction::ExitCurrentMode => ActionInfo {
                id: "exit_mode",
                name: "exit mode",
                description: "exit the current mode",
                available_in_modes: vec![
                    Mode::Edit(EditMode::Append),
                    Mode::Edit(EditMode::EditInPlace {
                        message_id: uuid::Uuid::nil(),
                        has_messages_below: false,
                    }),
                ],
                requires_focus: false,
            },
        }
    }
}
