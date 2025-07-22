use strum::EnumIter;
use uuid::Uuid;

// Schema - parameter-less templates for registry
#[derive(Debug, Clone, EnumIter)]
pub enum NormalModeActionSchema {
    // Navigation actions
    ScrollUp,
    ScrollDown,
    ScrollHalfPageUp,
    ScrollHalfPageDown,
    ScrollPageUp,
    ScrollPageDown,
    ScrollToTop,
    ScrollToBottom,
    ScrollToPreviousMessage,
    ScrollToNextMessage,

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
            Self::ScrollHalfPageUp => "scroll_half_page_up",
            Self::ScrollHalfPageDown => "scroll_half_page_down",
            Self::ScrollPageUp => "scroll_page_up",
            Self::ScrollPageDown => "scroll_page_down",
            Self::ScrollToTop => "scroll_to_top",
            Self::ScrollToBottom => "scroll_to_bottom",
            Self::ScrollToPreviousMessage => "scroll_to_previous_message",
            Self::ScrollToNextMessage => "scroll_to_next_message",
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
            Self::ScrollUp => "up",
            Self::ScrollDown => "down",
            Self::ScrollHalfPageUp => "half page up",
            Self::ScrollHalfPageDown => "half page down",
            Self::ScrollPageUp => "page up",
            Self::ScrollPageDown => "page down",
            Self::ScrollToTop => "top",
            Self::ScrollToBottom => "bottom",
            Self::ScrollToPreviousMessage => "previous message",
            Self::ScrollToNextMessage => "next message",
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
            Self::ScrollHalfPageUp => "scroll half page up",
            Self::ScrollHalfPageDown => "scroll half page down",
            Self::ScrollPageUp => "scroll page up",
            Self::ScrollPageDown => "scroll page down",
            Self::ScrollToTop => "scroll to top",
            Self::ScrollToBottom => "scroll to bottom",
            Self::ScrollToPreviousMessage => "scroll to previous message",
            Self::ScrollToNextMessage => "scroll to next message",
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
            | Self::ScrollHalfPageUp
            | Self::ScrollHalfPageDown
            | Self::ScrollPageUp
            | Self::ScrollPageDown
            | Self::ScrollToTop
            | Self::ScrollToBottom
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
            | Self::ShowMessage
            | Self::ScrollToPreviousMessage
            | Self::ScrollToNextMessage => true,
        }
    }
}

// Action - with real parameters for dispatch
#[derive(Debug, Clone)]
pub enum NormalModeAction {
    // Navigation actions
    ScrollUp,
    ScrollDown,
    ScrollHalfPageUp,
    ScrollHalfPageDown,
    ScrollPageUp,
    ScrollPageDown,
    ScrollToTop,
    ScrollToBottom,
    ScrollToPreviousMessage(Uuid),
    ScrollToNextMessage(Uuid),

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
