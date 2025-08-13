use crate::app::state::AppState;
use crate::services::DialogueService;

pub fn handle_enter_edit_mode(state: &mut AppState) {
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

pub fn handle_enter_append_mode(state: &mut AppState) {
    let Some(branch_id) = state.current_branch_id else {
        state.error_message = Some("No branch selected for appending".to_string());
        return;
    };

    if let Some((message_id, content)) = get_last_user_message(state) {
        continue_editing_message(state, message_id, content);
    } else {
        create_new_user_message(state, branch_id);
    }
}

fn get_last_user_message(state: &AppState) -> Option<(uuid::Uuid, String)> {
    state
        .current_branch()
        .and_then(|branch| branch.messages().iter().last())
        .and_then(|last_message| {
            if *last_message.role() == frond_core::Role::User {
                Some((last_message.id(), last_message.content().to_string()))
            } else {
                None
            }
        })
}

fn continue_editing_message(state: &mut AppState, message_id: uuid::Uuid, content: String) {
    state.focused_message_id = Some(message_id);
    
    let mut textarea = tui_textarea::TextArea::default();
    textarea.insert_str(&content);
    textarea.move_cursor(tui_textarea::CursorMove::Bottom);
    textarea.move_cursor(tui_textarea::CursorMove::End);
    state.edit_textarea = Some(textarea);
    
    state.mode = crate::app::Mode::Edit;
    state.pending_scrolling_request = Some(crate::app::state::ScrollingRequest::ScrollToBottom);
}

fn create_new_user_message(state: &mut AppState, branch_id: uuid::Uuid) {
    match DialogueService::append_message(&mut state.dialogue, branch_id, "".to_string()) {
        Ok(()) => setup_editing_for_new_message(state),
        Err(e) => {
            state.error_message = Some(format!("Failed to create new message: {}", e));
        }
    }
}

fn setup_editing_for_new_message(state: &mut AppState) {
    let Some(branch) = state.current_branch() else {
        state.error_message = Some("Branch no longer exists after creating message".to_string());
        return;
    };

    let Some(last_message) = branch.messages().iter().last() else {
        state.error_message = Some("Failed to find the newly created message".to_string());
        return;
    };

    state.focused_message_id = Some(last_message.id());
    state.edit_textarea = Some(tui_textarea::TextArea::from(vec![""]));
    state.mode = crate::app::Mode::Edit;
    state.pending_scrolling_request = Some(crate::app::state::ScrollingRequest::ScrollToBottom);
}

pub fn handle_enter_command_palette(state: &mut AppState) {
    // TODO: Implement command palette mode
    state.error_message = Some("Command palette not implemented yet".to_string());
}