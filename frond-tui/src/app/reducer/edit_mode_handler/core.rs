use std::ops::ControlFlow;

use crate::app::Mode;
use crate::app::state::AppState;
use crate::services::DialogueService;
use ratatui::crossterm::event::KeyEvent;

/// Handle raw key input in edit mode (for TextArea character input)
pub fn handle_edit_mode_raw_input(state: &mut AppState, key_event: KeyEvent) {
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.input(key_event);
    }
}

pub fn handle_exit_current_mode(state: &mut AppState) {
    if let ControlFlow::Break(_) = remove_active_selection(state) {
        return;
    }

    exit_to_normal_mode(state);
}

fn remove_active_selection(state: &mut AppState) -> ControlFlow<()> {
    if let Some(ref mut textarea) = state.edit_textarea {
        if textarea.is_selecting() {
            textarea.cancel_selection();
            return ControlFlow::Break(());
        }
    }
    ControlFlow::Continue(())
}

fn exit_to_normal_mode(state: &mut AppState) {
    let is_append_mode = state.is_append_mode();

    if let Some(result) = process_edit_content(state) {
        match result {
            EditResult::Error(error) => {
                state.error_message = Some(error);
                return;
            }
            EditResult::Deleted(message_id) => {
                invalidate_message_caches(state, message_id);
                if let Some(index) = state.get_message_index(message_id) {
                    state.update_focused_message_after_deletion(index);
                }
            }
            EditResult::Updated(message_id) => {
                invalidate_message_caches(state, message_id);
            }
        }
    }

    reset_edit_state(state);
    set_scrolling_after_exit(state, is_append_mode);
}

enum EditResult {
    Deleted(uuid::Uuid),
    Updated(uuid::Uuid),
    Error(String),
}

fn process_edit_content(state: &mut AppState) -> Option<EditResult> {
    let (textarea, message_id, branch_id) = (
        state.edit_textarea.as_ref()?,
        state.focused_message_id?,
        state.current_branch_id?,
    );

    let content = textarea.lines().join("\n").trim().to_string();

    if content.is_empty() {
        delete_empty_message(state, message_id, branch_id)
    } else {
        update_message_content(state, message_id, content)
    }
}

fn delete_empty_message(
    state: &mut AppState,
    message_id: uuid::Uuid,
    branch_id: uuid::Uuid,
) -> Option<EditResult> {
    match DialogueService::delete_message(&mut state.dialogue, message_id, branch_id) {
        Ok(_) => Some(EditResult::Deleted(message_id)),
        Err(e) => Some(EditResult::Error(format!("Failed to delete message: {}", e))),
    }
}

fn update_message_content(
    state: &mut AppState,
    message_id: uuid::Uuid,
    content: String,
) -> Option<EditResult> {
    match DialogueService::edit_message(&mut state.dialogue, message_id, content) {
        Ok(_) => Some(EditResult::Updated(message_id)),
        Err(e) => Some(EditResult::Error(format!("Failed to save message: {}", e))),
    }
}

fn invalidate_message_caches(state: &mut AppState, message_id: uuid::Uuid) {
    state.invalidate_message_highlight(message_id);
    state.invalidate_message_height(message_id);
}

fn reset_edit_state(state: &mut AppState) {
    state.mode = Mode::Normal;
    state.error_message = None;
    state.edit_textarea = None;
}

fn set_scrolling_after_exit(state: &mut AppState, is_append_mode: bool) {
    use crate::app::state::ScrollingRequest;
    
    state.pending_scrolling_request = if is_append_mode {
        Some(ScrollingRequest::ScrollToLastMessage)
    } else {
        state.focused_message_id.map(ScrollingRequest::ScrollToMessage)
    };
}

pub fn handle_submit_message(state: &mut AppState) {
    state.error_message = Some("Submit message action is not implemented yet".to_string());
}
