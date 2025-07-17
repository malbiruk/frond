use crate::app::Mode;
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum UIAction {
    // Application control
    Quit,

    // Navigation actions
    ScrollUp,
    ScrollDown,
    ScrollToMessage(Uuid),
    FocusMessage(Uuid),

    // Mode transitions
    EnterEditMode(Uuid),
    EnterAppendMode,
    EnterCommandPalette,
    ExitCurrentMode,

    // Content actions (these will dispatch to frond-core)
    EditMessage { message_id: Uuid, content: String },
    AppendMessage(String),
    DeleteMessage(Uuid),
    ForkBranch(Uuid),
    HideMessage(Uuid),
    ShowMessage(Uuid),

    // Branch navigation
    NextBranch,
    PrevBranch,
    NextTree,
    PrevTree,

    // UI-specific actions
    UpdateConfig(crate::config::Config),
    ShowHelp,

    // Error handling
    ShowError(String),
    ClearError,
}

#[derive(Debug, Clone)]
pub struct ActionInfo {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub available_in_modes: Vec<Mode>,
    pub requires_focus: bool,
}

impl UIAction {
    pub fn info(&self) -> ActionInfo {
        match self {
            UIAction::ScrollUp => ActionInfo {
                id: "scroll_up",
                name: "Scroll Up",
                description: "Scroll up one line",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::ScrollDown => ActionInfo {
                id: "scroll_down",
                name: "Scroll Down",
                description: "Scroll down one line",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::EnterEditMode(_) => ActionInfo {
                id: "edit_message",
                name: "Edit Message",
                description: "Edit the currently focused message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            UIAction::EnterAppendMode => ActionInfo {
                id: "append_message",
                name: "Append Message",
                description: "Add a new message to the conversation",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::DeleteMessage(_) => ActionInfo {
                id: "delete_message",
                name: "Delete Message",
                description: "Delete the currently focused message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            UIAction::ForkBranch(_) => ActionInfo {
                id: "fork_branch",
                name: "Fork Branch",
                description: "Create a new branch from the current message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            UIAction::HideMessage(_) => ActionInfo {
                id: "hide_message",
                name: "Hide Message",
                description: "Exclude the currently focused message from LLM context",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            UIAction::ShowHelp => ActionInfo {
                id: "show_help",
                name: "Show Help",
                description: "Display help information",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::NextBranch => ActionInfo {
                id: "next_branch",
                name: "Next Branch",
                description: "Switch to the next branch",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::PrevBranch => ActionInfo {
                id: "prev_branch",
                name: "Previous Branch",
                description: "Switch to the previous branch",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::NextTree => ActionInfo {
                id: "next_tree",
                name: "Next Tree",
                description: "Switch to the next tree",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::PrevTree => ActionInfo {
                id: "prev_tree",
                name: "Previous Tree",
                description: "Switch to the previous tree",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::EnterCommandPalette => ActionInfo {
                id: "command_palette",
                name: "Command Palette",
                description: "Open the command palette",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::ExitCurrentMode => ActionInfo {
                id: "exit_mode",
                name: "Exit Mode",
                description: "Exit the current mode",
                available_in_modes: vec![Mode::Edit(crate::app::EditMode::Append)],
                requires_focus: false,
            },
            UIAction::ScrollToMessage(_) => ActionInfo {
                id: "scroll_to_message",
                name: "Scroll to Message",
                description: "Scroll to a specific message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            UIAction::FocusMessage(_) => ActionInfo {
                id: "focus_message",
                name: "Focus Message",
                description: "Focus on a specific message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            _ => ActionInfo {
                id: "unknown",
                name: "Unknown",
                description: "Unknown action",
                available_in_modes: vec![],
                requires_focus: false,
            },
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

        // Register all available actions
        let action_list = vec![
            UIAction::ScrollUp,
            UIAction::ScrollDown,
            UIAction::EnterEditMode(Uuid::nil()),
            UIAction::EnterAppendMode,
            UIAction::DeleteMessage(Uuid::nil()),
            UIAction::ForkBranch(Uuid::nil()),
            UIAction::HideMessage(Uuid::nil()),
            UIAction::ShowHelp,
            UIAction::NextBranch,
            UIAction::PrevBranch,
            UIAction::NextTree,
            UIAction::PrevTree,
            UIAction::EnterCommandPalette,
            UIAction::ExitCurrentMode,
        ];

        for action in action_list {
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
