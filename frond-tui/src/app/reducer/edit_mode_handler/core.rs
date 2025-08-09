use crate::app::state::AppState;
use crate::app::Mode;
use crate::services::DialogueService;
use ratatui::crossterm::event::KeyEvent;

/// Handle raw key input in edit mode (for TextArea character input)
pub fn handle_edit_mode_raw_input(state: &mut AppState, key_event: KeyEvent) {
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.input(key_event);
    }
}

pub fn handle_exit_current_mode(state: &mut AppState) {
    // First check if there's an active selection - if so, cancel it instead of exiting
    if let Some(ref mut textarea) = state.edit_textarea {
        if textarea.is_selecting() {
            textarea.cancel_selection();
            return; // Stay in edit mode, just cancel selection
        }
    }
    
    // No selection active, proceed with normal exit behavior
    exit_to_normal_mode(state);
}

fn exit_to_normal_mode(state: &mut AppState) {
    // Check if we're in append mode before processing (editing last User message)
    let is_append_mode = if let Some(message_id) = state.focused_message_id {
        state
            .current_branch()
            .and_then(|branch| branch.messages().iter().last())
            .map(|last| last.id() == message_id && *last.role() == frond_core::Role::User)
            .unwrap_or(false)
    } else {
        false
    };

    if let (Some(textarea), Some(message_id), Some(branch_id)) = (
        &state.edit_textarea,
        state.focused_message_id,
        state.current_branch_id,
    ) {
        // Get content as-is, preserving all newlines the user typed
        let content = textarea.lines().join("\n").trim().to_string();

        if content.is_empty() {
            if let Err(e) =
                DialogueService::delete_message(&mut state.dialogue, message_id, branch_id)
            {
                state.error_message = Some(format!("Failed to delete message: {}", e));
                return;
            }
            // Invalidate cache for deleted message
            state.invalidate_message_highlight(message_id);
            if let Some(index) = state.get_message_index(message_id) {
                state.update_focused_message_after_deletion(index);
            }
        } else if let Err(e) =
            DialogueService::edit_message(&mut state.dialogue, message_id, content)
        {
            state.error_message = Some(format!("Failed to save message: {}", e));
            return;
        } else {
            // Invalidate cache for edited message
            state.invalidate_message_highlight(message_id);
        }
    }

    state.mode = Mode::Normal;
    state.error_message = None;
    state.edit_textarea = None;

    if is_append_mode {
        // Scroll to the last message after exiting append mode
        // This centers the lower part of the message if it's big
        state.pending_scrolling_request =
            Some(crate::app::state::ScrollingRequest::ScrollToLastMessage);
    } else {
        // For regular edit mode, focus the just edited message
        if let Some(message_id) = state.focused_message_id {
            state.pending_scrolling_request = Some(
                crate::app::state::ScrollingRequest::ScrollToMessage(message_id),
            );
        }
    }
}

pub fn handle_submit_message(state: &mut AppState) {
    state.error_message = Some("Submit message action is not implemented yet".to_string());
}