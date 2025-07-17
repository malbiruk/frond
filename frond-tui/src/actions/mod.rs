use crate::app::Mode;
use std::collections::HashMap;
use uuid::Uuid;

pub mod common;
pub mod edit_mode;
pub mod normal_mode;

pub use common::{ActionInfo, CommonAction};
pub use edit_mode::EditModeAction;
pub use normal_mode::NormalModeAction;

#[derive(Debug, Clone)]
pub enum UIAction {
    Common(CommonAction),
    NormalMode(NormalModeAction),
    EditMode(EditModeAction),
}

impl UIAction {
    pub fn info(&self) -> ActionInfo {
        match self {
            UIAction::Common(action) => action.info(),
            UIAction::NormalMode(action) => action.info(),
            UIAction::EditMode(action) => action.info(),
        }
    }
}

pub struct ActionRegistry {
    actions: HashMap<&'static str, UIAction>,
}

impl Default for ActionRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl ActionRegistry {
    pub fn new() -> Self {
        let mut actions = HashMap::new();

        // Register common actions
        let common_actions = vec![
            UIAction::Common(CommonAction::Quit),
            UIAction::Common(CommonAction::ClearError),
        ];

        // Register normal mode actions
        let normal_actions = vec![
            UIAction::NormalMode(NormalModeAction::ScrollUp),
            UIAction::NormalMode(NormalModeAction::ScrollDown),
            UIAction::NormalMode(NormalModeAction::EnterEditMode(Uuid::nil())),
            UIAction::NormalMode(NormalModeAction::EnterAppendMode),
            UIAction::NormalMode(NormalModeAction::EnterCommandPalette),
            UIAction::NormalMode(NormalModeAction::DeleteMessage(Uuid::nil())),
            UIAction::NormalMode(NormalModeAction::ForkBranch(Uuid::nil())),
            UIAction::NormalMode(NormalModeAction::HideMessage(Uuid::nil())),
            UIAction::NormalMode(NormalModeAction::ShowHelp),
            UIAction::NormalMode(NormalModeAction::NextBranch),
            UIAction::NormalMode(NormalModeAction::PrevBranch),
            UIAction::NormalMode(NormalModeAction::NextTree),
            UIAction::NormalMode(NormalModeAction::PrevTree),
        ];

        // Register edit mode actions
        let edit_actions = vec![UIAction::EditMode(EditModeAction::ExitCurrentMode)];

        // Combine all actions
        let all_actions = [common_actions, normal_actions, edit_actions].concat();

        for action in all_actions {
            let info = action.info();
            actions.insert(info.id, action);
        }

        Self { actions }
    }

    pub fn get_action(&self, id: &str) -> Option<&UIAction> {
        self.actions.get(id)
    }

    pub fn get_actions_for_mode(&self, mode: Mode) -> Vec<(&'static str, &UIAction)> {
        self.actions
            .iter()
            .filter(|(_, action)| {
                let info = action.info();
                info.available_in_modes
                    .iter()
                    .any(|m| mode_matches(mode, *m))
            })
            .map(|(id, action)| (*id, action))
            .collect()
    }

    pub fn get_essential_actions_for_mode(&self, mode: Mode) -> Vec<(&'static str, &UIAction)> {
        let essential_order = match mode {
            Mode::Normal => vec![
                "append_message",
                "edit_message",
                "hide_message",
                "fork_branch",
                "delete_message",
                "show_help",
            ],
            Mode::Edit(_) => vec!["exit_mode"],
        };

        essential_order
            .into_iter()
            .filter_map(|action_id| {
                let action = self.actions.get(action_id)?;
                let info = action.info();

                let available = info
                    .available_in_modes
                    .iter()
                    .any(|m| mode_matches(mode, *m));

                if available {
                    Some((action_id, action))
                } else {
                    None
                }
            })
            .collect()
    }
}

fn mode_matches(current: Mode, available: Mode) -> bool {
    matches!(
        (current, available),
        (Mode::Normal, Mode::Normal) | (Mode::Edit(_), Mode::Edit(_))
    )
}

pub trait ActionDispatcher {
    fn dispatch(&mut self, action: UIAction);
}
