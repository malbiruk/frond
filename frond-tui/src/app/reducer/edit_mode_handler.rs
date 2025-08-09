use crate::actions::EditModeAction;
use crate::app::Mode;
use crate::app::state::AppState;
use crate::services::DialogueService;
use tui_textarea::CursorMove;
use ratatui::crossterm::event::KeyEvent;

/// Handle raw key input in edit mode (for TextArea character input)
pub fn handle_edit_mode_raw_input(state: &mut AppState, key_event: KeyEvent) {
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.input(key_event);
    }
}

pub fn handle_edit_mode_action(state: &mut AppState, action: EditModeAction) {
    match action {
        EditModeAction::ExitCurrentMode => handle_exit_current_mode(state),
        EditModeAction::SubmitMessage => handle_submit_message(state),

        // Character operations
        EditModeAction::DeleteChar => handle_delete_char(state),
        EditModeAction::DeleteNextChar => handle_delete_next_char(state),
        EditModeAction::InsertNewline => handle_insert_newline(state),

        // Line operations
        EditModeAction::DeleteLineByEnd => handle_delete_line_by_end(state),
        EditModeAction::DeleteLineByHead => handle_delete_line_by_head(state),

        // Word operations
        EditModeAction::DeleteWord => handle_delete_word(state),
        EditModeAction::DeleteNextWord => handle_delete_next_word(state),

        // Undo/Redo
        EditModeAction::Undo => handle_undo(state),
        EditModeAction::Redo => handle_redo(state),

        // Clipboard operations
        EditModeAction::Copy => handle_copy(state),
        EditModeAction::Cut => handle_cut(state),
        EditModeAction::Paste => handle_paste(state),

        // Selection operations
        EditModeAction::SelectAll => handle_select_all(state),

        // Cursor movement - character level
        EditModeAction::MoveCursorForward => handle_move_cursor_forward(state),
        EditModeAction::MoveCursorBack => handle_move_cursor_back(state),
        EditModeAction::MoveCursorUp => handle_move_cursor_up(state),
        EditModeAction::MoveCursorDown => handle_move_cursor_down(state),

        // Cursor movement - word level
        EditModeAction::MoveCursorWordForward => handle_move_cursor_word_forward(state),
        EditModeAction::MoveCursorWordEnd => handle_move_cursor_word_end(state),
        EditModeAction::MoveCursorWordBack => handle_move_cursor_word_back(state),

        // Cursor movement - line boundaries
        EditModeAction::MoveCursorEnd => handle_move_cursor_end(state),
        EditModeAction::MoveCursorHead => handle_move_cursor_head(state),

        // Cursor movement - document boundaries
        EditModeAction::MoveCursorTop => handle_move_cursor_top(state),
        EditModeAction::MoveCursorBottom => handle_move_cursor_bottom(state),

        _ => {
            state.error_message = Some(format!("Action {:?} not implemented yet", action));
        }
    }
}

fn handle_exit_current_mode(state: &mut AppState) {
    // Check if we're in append mode before processing (editing last User message)
    let is_append_mode = if let Some(message_id) = state.focused_message_id {
        state.current_branch()
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
        state.pending_scrolling_request = Some(crate::app::state::ScrollingRequest::ScrollToLastMessage);
    } else {
        // For regular edit mode, focus the just edited message
        if let Some(message_id) = state.focused_message_id {
            state.pending_scrolling_request = Some(crate::app::state::ScrollingRequest::ScrollToMessage(message_id));
        }
    }
}

fn handle_submit_message(state: &mut AppState) {
    state.error_message = Some("Submitting messages is not implemented yet".to_string());
}

// Helper function to ensure textarea exists
fn ensure_textarea(state: &mut AppState) -> bool {
    if state.edit_textarea.is_none() {
        state.error_message = Some("No textarea available".to_string());
        false
    } else {
        true
    }
}

// Character operations
fn handle_delete_char(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.delete_char();
        }
    }
}

fn handle_delete_next_char(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.delete_next_char();
        }
    }
}

fn handle_insert_newline(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.insert_newline();
        }
    }
}

// Line operations
fn handle_delete_line_by_end(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.delete_line_by_end();
        }
    }
}

fn handle_delete_line_by_head(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.delete_line_by_head();
        }
    }
}

// Word operations
fn handle_delete_word(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.delete_word();
        }
    }
}

fn handle_delete_next_word(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.delete_next_word();
        }
    }
}

// Undo/Redo
fn handle_undo(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.undo();
        }
    }
}

fn handle_redo(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.redo();
        }
    }
}

// Clipboard operations
fn handle_copy(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.copy();
        }
    }
}

fn handle_cut(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.cut();
        }
    }
}

fn handle_paste(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.paste();
        }
    }
}

// Selection operations
fn handle_select_all(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.select_all();
        }
    }
}

// Cursor movement - character level
fn handle_move_cursor_forward(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::Forward);
        }
    }
}

fn handle_move_cursor_back(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::Back);
        }
    }
}

fn handle_move_cursor_up(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::Up);
        }
    }
}

fn handle_move_cursor_down(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::Down);
        }
    }
}

// Cursor movement - word level
fn handle_move_cursor_word_forward(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::WordForward);
        }
    }
}

fn handle_move_cursor_word_end(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::WordEnd);
        }
    }
}

fn handle_move_cursor_word_back(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::WordBack);
        }
    }
}

// Cursor movement - line boundaries
fn handle_move_cursor_end(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::End);
        }
    }
}

fn handle_move_cursor_head(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::Head);
        }
    }
}

// Cursor movement - document boundaries
fn handle_move_cursor_top(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::Top);
        }
    }
}

fn handle_move_cursor_bottom(state: &mut AppState) {
    if ensure_textarea(state) {
        if let Some(ref mut textarea) = state.edit_textarea {
            textarea.move_cursor(CursorMove::Bottom);
        }
    }
}
