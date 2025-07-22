use crate::actions::macros::define_actions;

define_actions! {
    NormalModeActionSchema, NormalModeAction {
        // Navigation actions
        ScrollUp {
            id: "scroll_up",
            name: "up",
            description: "scroll up one line",
        },
        ScrollDown {
            id: "scroll_down", 
            name: "down",
            description: "scroll down one line",
        },
        ScrollHalfPageUp {
            id: "scroll_half_page_up",
            name: "half page up", 
            description: "scroll half page up",
        },
        ScrollHalfPageDown {
            id: "scroll_half_page_down",
            name: "half page down",
            description: "scroll half page down", 
        },
        ScrollPageUp {
            id: "scroll_page_up",
            name: "page up",
            description: "scroll page up",
        },
        ScrollPageDown {
            id: "scroll_page_down",
            name: "page down", 
            description: "scroll page down",
        },
        ScrollToTop {
            id: "scroll_to_top",
            name: "top",
            description: "scroll to top",
        },
        ScrollToBottom {
            id: "scroll_to_bottom", 
            name: "bottom",
            description: "scroll to bottom",
        },
        ScrollToPreviousMessage {
            id: "scroll_to_previous_message",
            name: "previous message",
            description: "scroll to previous message",
            requires_focus: true,
        },
        ScrollToNextMessage {
            id: "scroll_to_next_message",
            name: "next message", 
            description: "scroll to next message",
            requires_focus: true,
        },

        // Mode transitions
        EnterEditMode {
            id: "edit_message",
            name: "edit",
            description: "edit the currently focused message",
            requires_focus: true,
        },
        EnterAppendMode {
            id: "append_message",
            name: "append",
            description: "add a new message to the conversation",
        },
        EnterCommandPalette {
            id: "command_palette", 
            name: "command palette",
            description: "open the command palette",
        },

        // Message actions
        DeleteMessage {
            id: "delete_message",
            name: "delete",
            description: "delete the currently focused message", 
            requires_focus: true,
        },
        ForkBranch {
            id: "fork_branch",
            name: "fork",
            description: "create a new branch from the current message",
            requires_focus: true,
        },
        HideMessage {
            id: "hide_message",
            name: "hide", 
            description: "exclude the currently focused message from llm context",
            requires_focus: true,
        },
        ShowMessage {
            id: "show_message",
            name: "show",
            description: "include the message in llm context",
            requires_focus: true,
        },

        // Branch navigation
        NextBranch {
            id: "next_branch",
            name: "next branch",
            description: "switch to the next branch",
        },
        PrevBranch {
            id: "prev_branch",
            name: "previous branch", 
            description: "switch to the previous branch",
        },
        NextTree {
            id: "next_tree",
            name: "next tree",
            description: "switch to the next tree",
        },
        PrevTree {
            id: "prev_tree",
            name: "previous tree",
            description: "switch to the previous tree", 
        },
    }
}