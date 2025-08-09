use crate::actions::NormalModeAction;
use crate::app::state::{AppState, ScrollingRequest};
use crate::services::DialogueService;
use uuid::Uuid;

pub fn handle_normal_mode_action(state: &mut AppState, action: NormalModeAction) {
    match action {
        // Navigation actions
        NormalModeAction::ScrollUp => handle_scroll_up(state),
        NormalModeAction::ScrollDown => handle_scroll_down(state),
        NormalModeAction::ScrollHalfPageUp => handle_scroll_half_page_up(state),
        NormalModeAction::ScrollHalfPageDown => handle_scroll_half_page_down(state),
        NormalModeAction::ScrollPageUp => handle_scroll_page_up(state),
        NormalModeAction::ScrollPageDown => handle_scroll_page_down(state),
        NormalModeAction::ScrollToTop => handle_scroll_to_top(state),
        NormalModeAction::ScrollToBottom => handle_scroll_to_bottom(state),
        NormalModeAction::ScrollToNextMessage => handle_scroll_to_next_message(state),
        NormalModeAction::ScrollToPreviousMessage => handle_scroll_to_previous_message(state),

        // Mode transitions
        NormalModeAction::EnterEditMode => handle_enter_edit_mode(state),
        NormalModeAction::EnterAppendMode => handle_enter_append_mode(state),
        NormalModeAction::EnterCommandPalette => handle_enter_command_palette(state),

        // Message actions that use frond-core
        NormalModeAction::DeleteMessage => handle_delete_message(state),
        NormalModeAction::ForkBranch => handle_fork_branch(state),
        NormalModeAction::HideMessage => handle_hide_message(state),
        NormalModeAction::ShowMessage => handle_show_message(state),

        // Branch/Tree navigation
        NormalModeAction::NextBranch => handle_next_branch(state),
        NormalModeAction::PrevBranch => handle_prev_branch(state),
        NormalModeAction::NextTree => handle_next_tree(state),
        NormalModeAction::PrevTree => handle_prev_tree(state),
    }
}

// Navigation handlers
fn handle_scroll_up(state: &mut AppState) {
    state.scroll_offset = state.scroll_offset.saturating_sub(1);
}

fn handle_scroll_down(state: &mut AppState) {
    state.scroll_offset = state.scroll_offset.saturating_add(1);
}

fn handle_scroll_page_up(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollPageUp);
}

fn handle_scroll_page_down(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollPageDown);
}

fn handle_scroll_half_page_up(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollHalfPageUp);
}

fn handle_scroll_half_page_down(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollHalfPageDown);
}

fn handle_scroll_to_top(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollToTop);
}

fn handle_scroll_to_bottom(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollToBottom);
}

fn handle_scroll_to_adjacent_message<F>(
    state: &mut AppState,
    message_id: Uuid,
    get_adjacent_index: F,
) where
    F: Fn(usize, usize) -> Option<usize>,
{
    let messages = state.current_messages();
    if let Some(current_index) = messages.iter().position(|m| m.id() == message_id) {
        if let Some(adjacent_index) = get_adjacent_index(current_index, messages.len()) {
            if let Some(adjacent_message) = messages.get(adjacent_index) {
                state.pending_scrolling_request =
                    Some(ScrollingRequest::ScrollToMessage(adjacent_message.id()));
            }
        }
    }
}

fn handle_scroll_to_next_message(state: &mut AppState) {
    if let Some(message_id) = state.focused_message_id {
        handle_scroll_to_adjacent_message(state, message_id, |current, len| {
            if current + 1 < len {
                Some(current + 1)
            } else {
                None
            }
        });
    }
}

fn handle_scroll_to_previous_message(state: &mut AppState) {
    if let Some(message_id) = state.focused_message_id {
        handle_scroll_to_adjacent_message(state, message_id, |current, _len| {
            if current > 0 { Some(current - 1) } else { None }
        });
    }
}

// Mode transition handlers
fn handle_enter_edit_mode(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for editing".to_string());
        return;
    };

    // Center the message being edited before switching to edit mode for visual consistency
    state.pending_scrolling_request = Some(crate::app::state::ScrollingRequest::ScrollToMessage(message_id));
    // Switch to edit mode - scrollbar state will be properly set from the scrolling request
    state.mode = crate::app::Mode::Edit;
    // TextArea will be initialized in edit_mode::content::render with proper viewport width
}

fn handle_enter_append_mode(state: &mut AppState) {
    let Some(branch_id) = state.current_branch_id else {
        state.error_message = Some("No branch selected for appending".to_string());
        return;
    };

    // Check if the last message is a User message - if so, continue editing it
    let should_continue_editing = state
        .current_branch()
        .and_then(|branch| branch.messages().iter().last())
        .and_then(|last_message| {
            if *last_message.role() == frond_core::Role::User {
                Some((last_message.id(), last_message.content().to_string()))
            } else {
                None
            }
        });

    if let Some((message_id, content)) = should_continue_editing {
        // Continue editing the existing last user message
        state.focused_message_id = Some(message_id);

        // Initialize TextArea immediately for append mode
        let mut textarea = tui_textarea::TextArea::default();
        textarea.insert_str(&content);
        textarea.move_cursor(tui_textarea::CursorMove::Bottom);
        textarea.move_cursor(tui_textarea::CursorMove::End);
        state.edit_textarea = Some(textarea);

        state.mode = crate::app::Mode::Edit;
        // Scroll to bottom to show the message being edited
        state.pending_scrolling_request = Some(crate::app::state::ScrollingRequest::ScrollToBottom);
        return;
    }

    // Otherwise create a new empty User message
    match DialogueService::append_message(&mut state.dialogue, branch_id, "".to_string()) {
        Ok(()) => {
            // Get the last message (the one we just created) and focus it
            if let Some(branch) = state.current_branch() {
                if let Some(last_message) = branch.messages().iter().last() {
                    let new_message_id = last_message.id();
                    state.focused_message_id = Some(new_message_id);

                    // Initialize empty TextArea immediately for append mode
                    let textarea = tui_textarea::TextArea::from(vec![""]); // Start with one empty line
                    state.edit_textarea = Some(textarea);

                    // Switch to edit mode and scroll to bottom
                    state.mode = crate::app::Mode::Edit;
                    state.pending_scrolling_request =
                        Some(crate::app::state::ScrollingRequest::ScrollToBottom);
                } else {
                    state.error_message =
                        Some("Failed to find the newly created message".to_string());
                }
            } else {
                state.error_message =
                    Some("Branch no longer exists after creating message".to_string());
            }
        }
        Err(e) => {
            state.error_message = Some(format!("Failed to create new message: {}", e));
        }
    }
}

fn handle_enter_command_palette(state: &mut AppState) {
    // TODO: Implement command palette mode
    state.error_message = Some("Command palette not implemented yet".to_string());
}

// Message action handlers that bridge to frond-core
fn handle_delete_message(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for deletion".to_string());
        return;
    };

    if let Some(branch_id) = state.current_branch_id {
        let message_index = state.get_message_index(message_id);

        match DialogueService::delete_message(&mut state.dialogue, message_id, branch_id) {
            Ok(_) => {
                // Invalidate cache for deleted message
                state.invalidate_message_highlight(message_id);
                if let Some(index) = message_index {
                    state.update_focused_message_after_deletion(index);
                }
            }
            Err(e) => {
                state.error_message = Some(e);
            }
        }
    }
}

fn handle_fork_branch(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for forking".to_string());
        return;
    };

    if let Some(tree_id) = state.current_tree_id {
        if let Some(branch_id) = state.current_branch_id {
            let new_branch_name = format!("fork-{}", chrono::Utc::now().timestamp());

            match DialogueService::fork_branch(
                &mut state.dialogue,
                tree_id,
                branch_id,
                message_id,
                new_branch_name,
            ) {
                Ok(_) => {
                    // Branch created successfully
                }
                Err(e) => {
                    state.error_message = Some(e);
                }
            }
        }
    }
}

fn handle_hide_message(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for hiding".to_string());
        return;
    };

    if let Err(e) = DialogueService::hide_message(&mut state.dialogue, message_id) {
        state.error_message = Some(e);
    }
}

fn handle_show_message(state: &mut AppState) {
    let Some(message_id) = state.focused_message_id else {
        state.error_message = Some("No message selected for showing".to_string());
        return;
    };

    if let Err(e) = DialogueService::show_message(&mut state.dialogue, message_id) {
        state.error_message = Some(e);
    }
}

// Branch/Tree navigation handlers
fn handle_next_branch(state: &mut AppState) {
    if let Some(tree) = state.current_tree() {
        if let Some(current_branch_id) = state.current_branch_id {
            let branches = tree.branches();
            if let Some(current_index) = branches.get_index_by_id(current_branch_id) {
                // Get current focused message index before switching
                let current_focused_index = state
                    .focused_message_id
                    .and_then(|id| state.get_message_index(id))
                    .unwrap_or(0);

                let next_index = (current_index + 1) % branches.len();
                if let Some(next_branch) = branches.get(next_index) {
                    state.current_branch_id = Some(next_branch.id());
                    state.reset_focus_for_new_branch();

                    // Request focus on same index, or last message if new branch is shorter
                    state.pending_scrolling_request =
                        Some(ScrollingRequest::ScrollToMessageWithSameIndexOrLast {
                            previous_index: current_focused_index,
                        });
                }
            }
        }
    }
}

fn handle_prev_branch(state: &mut AppState) {
    if let Some(tree) = state.current_tree() {
        if let Some(current_branch_id) = state.current_branch_id {
            let branches = tree.branches();
            if let Some(current_index) = branches.get_index_by_id(current_branch_id) {
                // Get current focused message index before switching
                let current_focused_index = state
                    .focused_message_id
                    .and_then(|id| state.get_message_index(id))
                    .unwrap_or(0);

                let prev_index = if current_index == 0 {
                    branches.len() - 1
                } else {
                    current_index - 1
                };
                if let Some(prev_branch) = branches.get(prev_index) {
                    state.current_branch_id = Some(prev_branch.id());
                    state.reset_focus_for_new_branch();

                    // Request focus on same index, or last message if new branch is shorter
                    state.pending_scrolling_request =
                        Some(ScrollingRequest::ScrollToMessageWithSameIndexOrLast {
                            previous_index: current_focused_index,
                        });
                }
            }
        }
    }
}

fn handle_next_tree(state: &mut AppState) {
    let trees = state.dialogue.trees();
    if let Some(current_tree_id) = state.current_tree_id {
        if let Some(current_index) = trees.get_index_by_id(current_tree_id) {
            let next_index = (current_index + 1) % trees.len();
            if let Some(next_tree) = trees.get(next_index) {
                state.current_tree_id = Some(next_tree.id());
                state.reset_focus_for_new_tree();

                // Request focus on last message of new tree
                state.pending_scrolling_request = Some(ScrollingRequest::ScrollToLastMessage);
            }
        }
    }
}

fn handle_prev_tree(state: &mut AppState) {
    let trees = state.dialogue.trees();
    if let Some(current_tree_id) = state.current_tree_id {
        if let Some(current_index) = trees.get_index_by_id(current_tree_id) {
            let prev_index = if current_index == 0 {
                trees.len() - 1
            } else {
                current_index - 1
            };
            if let Some(prev_tree) = trees.get(prev_index) {
                state.current_tree_id = Some(prev_tree.id());
                state.reset_focus_for_new_tree();

                // Request focus on last message of new tree
                state.pending_scrolling_request = Some(ScrollingRequest::ScrollToLastMessage);
            }
        }
    }
}
