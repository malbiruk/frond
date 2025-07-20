use crate::actions::NormalModeAction;
use crate::app::state::{AppState, FocusRequest};
use crate::services::DialogueService;
use uuid::Uuid;

pub fn handle_normal_mode_action(state: &mut AppState, action: NormalModeAction) {
    match action {
        // Navigation actions
        NormalModeAction::ScrollUp => handle_scroll_up(state),
        NormalModeAction::ScrollDown => handle_scroll_down(state),

        // Mode transitions
        NormalModeAction::EnterEditMode(message_id) => handle_enter_edit_mode(state, message_id),
        NormalModeAction::EnterAppendMode => handle_enter_append_mode(state),
        NormalModeAction::EnterCommandPalette => handle_enter_command_palette(state),

        // Message actions that use frond-core
        NormalModeAction::DeleteMessage(message_id) => handle_delete_message(state, message_id),
        NormalModeAction::ForkBranch(message_id) => handle_fork_branch(state, message_id),
        NormalModeAction::HideMessage(message_id) => handle_hide_message(state, message_id),
        NormalModeAction::ShowMessage(message_id) => handle_show_message(state, message_id),

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

// Mode transition handlers
fn handle_enter_edit_mode(state: &mut AppState, message_id: Uuid) {
    if message_id == Uuid::nil() {
        state.error_message = Some("No message selected for editing".to_string());
        return;
    }

    let has_messages_below = state.has_messages_after(message_id);
    state.mode = crate::app::Mode::Edit(crate::app::EditMode::EditInPlace {
        message_id,
        has_messages_below,
    });
}

fn handle_enter_append_mode(state: &mut AppState) {
    // Transition to append mode
    state.mode = crate::app::Mode::Edit(crate::app::EditMode::Append);
}

fn handle_enter_command_palette(state: &mut AppState) {
    // TODO: Implement command palette mode
    state.error_message = Some("Command palette not implemented yet".to_string());
}

// Message action handlers that bridge to frond-core

fn handle_delete_message(state: &mut AppState, message_id: Uuid) {
    if let Some(branch_id) = state.current_branch_id {
        let message_index = state.get_message_index(message_id);

        match DialogueService::delete_message(&mut state.dialogue, message_id, branch_id) {
            Ok(_) => {
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

fn handle_fork_branch(state: &mut AppState, message_id: Uuid) {
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

fn handle_hide_message(state: &mut AppState, message_id: Uuid) {
    if let Err(e) = DialogueService::hide_message(&mut state.dialogue, message_id) {
        state.error_message = Some(e);
    }
}

fn handle_show_message(state: &mut AppState, message_id: Uuid) {
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
                    state.pending_focus_request = Some(FocusRequest::SameIndexOrLast {
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
                    state.pending_focus_request = Some(FocusRequest::SameIndexOrLast {
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
                state.pending_focus_request = Some(FocusRequest::LastMessage);
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
                state.pending_focus_request = Some(FocusRequest::LastMessage);
            }
        }
    }
}
