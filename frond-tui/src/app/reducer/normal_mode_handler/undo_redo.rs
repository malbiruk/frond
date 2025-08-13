use crate::app::state::AppState;
use crate::services::DialogueService;
use frond_core::{Action, BranchAction, MessageAction};

pub fn handle_undo(state: &mut AppState) {
    match DialogueService::undo(&mut state.dialogue) {
        Ok(Some(action)) => {
            handle_action_cache_invalidation(state, &action);
            update_focus_after_action(state, &action, true); // true for undo
        }
        Ok(None) => {
            // No action to undo, silently ignore
        }
        Err(e) => {
            state.error_message = Some(e);
        }
    }
}

pub fn handle_redo(state: &mut AppState) {
    match DialogueService::redo(&mut state.dialogue) {
        Ok(Some(action)) => {
            handle_action_cache_invalidation(state, &action);
            update_focus_after_action(state, &action, false); // false for redo
        }
        Ok(None) => {
            // No action to redo, silently ignore
        }
        Err(e) => {
            state.error_message = Some(e);
        }
    }
}

fn handle_action_cache_invalidation(state: &mut AppState, action: &Action) {
    match action {
        Action::Message(message_action) => match message_action {
            MessageAction::EditMessage { message_id, .. } => {
                // Content changed, invalidate height and highlight
                state.invalidate_message_height(*message_id);
                state.invalidate_message_highlight(*message_id);
            }
            MessageAction::DeleteMessage { message_id, .. } => {
                // Message deleted/restored, invalidate its cache
                state.invalidate_message_height(*message_id);
                state.invalidate_message_highlight(*message_id);
            }
            MessageAction::HideMessage { message_id }
            | MessageAction::ShowMessage { message_id } => {
                // Visibility changed, might affect highlighting
                state.invalidate_message_highlight(*message_id);
            }
            MessageAction::ToggleMessageRole { message_id } => {
                // Role changed, might affect highlighting
                state.invalidate_message_highlight(*message_id);
            }
        },
        Action::Branch(branch_action) => {
            if let BranchAction::AppendMessage { .. } = branch_action {
                // New message added/removed, no specific cache to invalidate
                // The message itself will handle its own cache when it gets created
            }
        }
        // Tree and Dialogue actions typically don't affect individual message cache
        Action::Tree(_) | Action::Dialogue(_) => {}
    }
}

fn update_focus_after_action(state: &mut AppState, action: &Action, is_undo: bool) {
    match action {
        Action::Message(message_action) => match message_action {
            MessageAction::EditMessage { message_id, .. } => {
                // Focus the edited message
                state.focus_message(*message_id);
            }
            MessageAction::DeleteMessage { message_id, .. } => {
                if is_undo {
                    // Message was restored, focus it
                    state.focus_message(*message_id);
                } else {
                    // Message was deleted, update focus using the same logic as normal deletion
                    if let Some(index) = state.get_message_index(*message_id) {
                        state.update_focused_message_after_deletion(index);
                    }
                }
            }
            MessageAction::HideMessage { message_id }
            | MessageAction::ShowMessage { message_id } => {
                // Focus the message that had its visibility changed
                state.focus_message(*message_id);
            }
            MessageAction::ToggleMessageRole { message_id } => {
                // Focus the message that had its role changed
                state.focus_message(*message_id);
            }
        },
        Action::Branch(branch_action) => {
            if let BranchAction::AppendMessage { message_id, .. } = branch_action {
                // Focus the appended message directly by its known ID
                state.focus_message(*message_id);
            }
        }
        Action::Tree(_) | Action::Dialogue(_) => {
            // These actions typically don't affect message focus
        }
    }
}
