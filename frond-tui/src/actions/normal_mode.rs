use strum::EnumIter;
use uuid::Uuid;

// Schema - parameter-less templates for registry
#[derive(Debug, Clone, EnumIter)]
pub enum NormalModeActionSchema {
    // Navigation actions
    ScrollUp,
    ScrollDown,

    // Mode transitions
    EnterEditMode,
    EnterAppendMode,
    EnterCommandPalette,

    // Message actions
    DeleteMessage,
    ForkBranch,
    HideMessage,
    ShowMessage,

    // Branch navigation
    NextBranch,
    PrevBranch,
    NextTree,
    PrevTree,
}

impl NormalModeActionSchema {
    pub fn id(&self) -> &'static str {
        match self {
            Self::ScrollUp => "scroll_up",
            Self::ScrollDown => "scroll_down",
            Self::EnterEditMode => "edit_message",
            Self::EnterAppendMode => "append_message",
            Self::EnterCommandPalette => "command_palette",
            Self::DeleteMessage => "delete_message",
            Self::ForkBranch => "fork_branch",
            Self::HideMessage => "hide_message",
            Self::ShowMessage => "show_message",
            Self::NextBranch => "next_branch",
            Self::PrevBranch => "prev_branch",
            Self::NextTree => "next_tree",
            Self::PrevTree => "prev_tree",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Self::ScrollUp => "scroll up",
            Self::ScrollDown => "scroll down",
            Self::EnterEditMode => "edit",
            Self::EnterAppendMode => "append",
            Self::EnterCommandPalette => "command palette",
            Self::DeleteMessage => "delete",
            Self::ForkBranch => "fork",
            Self::HideMessage => "hide",
            Self::ShowMessage => "show",
            Self::NextBranch => "next branch",
            Self::PrevBranch => "previous branch",
            Self::NextTree => "next tree",
            Self::PrevTree => "previous tree",
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ScrollUp => "scroll up one line",
            Self::ScrollDown => "scroll down one line",
            Self::EnterEditMode => "edit the currently focused message",
            Self::EnterAppendMode => "add a new message to the conversation",
            Self::EnterCommandPalette => "open the command palette",
            Self::DeleteMessage => "delete the currently focused message",
            Self::ForkBranch => "create a new branch from the current message",
            Self::HideMessage => "exclude the currently focused message from llm context",
            Self::ShowMessage => "include the message in llm context",
            Self::NextBranch => "switch to the next branch",
            Self::PrevBranch => "switch to the previous branch",
            Self::NextTree => "switch to the next tree",
            Self::PrevTree => "switch to the previous tree",
        }
    }

    pub fn requires_focus(&self) -> bool {
        match self {
            Self::ScrollUp
            | Self::ScrollDown
            | Self::EnterAppendMode
            | Self::EnterCommandPalette
            | Self::NextBranch
            | Self::PrevBranch
            | Self::NextTree
            | Self::PrevTree => false,
            Self::EnterEditMode
            | Self::DeleteMessage
            | Self::ForkBranch
            | Self::HideMessage
            | Self::ShowMessage => true,
        }
    }
}

// Action - with real parameters for dispatch
#[derive(Debug, Clone)]
pub enum NormalModeAction {
    // Navigation actions
    ScrollUp,
    ScrollDown,

    // Mode transitions
    EnterEditMode(Uuid),
    EnterAppendMode,
    EnterCommandPalette,

    // Message actions
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
