use super::common::ActionInfo;
use crate::app::Mode;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub enum NormalModeAction {
    // Navigation actions
    ScrollUp,
    ScrollDown,
    ScrollToMessage(Uuid),
    FocusMessage(Uuid),

    // Mode transitions
    EnterEditMode(Uuid),
    EnterAppendMode,
    EnterCommandPalette,

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
}

impl NormalModeAction {
    pub fn info(&self) -> ActionInfo {
        match self {
            NormalModeAction::ScrollUp => ActionInfo {
                id: "scroll_up",
                name: "scroll up",
                description: "scroll up one line",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::ScrollDown => ActionInfo {
                id: "scroll_down",
                name: "scroll down",
                description: "scroll down one line",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::ScrollToMessage(_) => ActionInfo {
                id: "scroll_to_message",
                name: "scroll to message",
                description: "scroll to a specific message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::FocusMessage(_) => ActionInfo {
                id: "focus_message",
                name: "focus message",
                description: "focus on a specific message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::EnterEditMode(_) => ActionInfo {
                id: "edit_message",
                name: "edit",
                description: "edit the currently focused message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            NormalModeAction::EnterAppendMode => ActionInfo {
                id: "append_message",
                name: "append",
                description: "add a new message to the conversation",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::EnterCommandPalette => ActionInfo {
                id: "command_palette",
                name: "command palette",
                description: "open the command palette",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::EditMessage { .. } => ActionInfo {
                id: "edit_message_content",
                name: "edit message content",
                description: "update the content of a message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::AppendMessage(_) => ActionInfo {
                id: "append_message_content",
                name: "append message content",
                description: "add a new message with specific content",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::DeleteMessage(_) => ActionInfo {
                id: "delete_message",
                name: "delete",
                description: "delete the currently focused message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            NormalModeAction::ForkBranch(_) => ActionInfo {
                id: "fork_branch",
                name: "fork",
                description: "create a new branch from the current message",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            NormalModeAction::HideMessage(_) => ActionInfo {
                id: "hide_message",
                name: "hide",
                description: "exclude the currently focused message from llm context",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            NormalModeAction::ShowMessage(_) => ActionInfo {
                id: "show_message",
                name: "show",
                description: "include the message in llm context",
                available_in_modes: vec![Mode::Normal],
                requires_focus: true,
            },
            NormalModeAction::NextBranch => ActionInfo {
                id: "next_branch",
                name: "next branch",
                description: "switch to the next branch",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::PrevBranch => ActionInfo {
                id: "prev_branch",
                name: "previous branch",
                description: "switch to the previous branch",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::NextTree => ActionInfo {
                id: "next_tree",
                name: "next tree",
                description: "switch to the next tree",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
            NormalModeAction::PrevTree => ActionInfo {
                id: "prev_tree",
                name: "previous tree",
                description: "switch to the previous tree",
                available_in_modes: vec![Mode::Normal],
                requires_focus: false,
            },
        }
    }
}
