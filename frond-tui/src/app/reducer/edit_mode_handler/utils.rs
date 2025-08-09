use crate::app::state::AppState;
use tui_textarea::{CursorMove, TextArea};

/// Helper function to ensure textarea exists and execute an operation on it
pub fn with_textarea<F>(state: &mut AppState, operation: F) -> bool
where
    F: FnOnce(&mut TextArea<'_>),
{
    if state.edit_textarea.is_none() {
        state.error_message = Some("No textarea available".to_string());
        return false;
    }
    
    if let Some(ref mut textarea) = state.edit_textarea {
        operation(textarea);
        true
    } else {
        false
    }
}

/// Execute a simple cursor move operation
pub fn move_cursor(state: &mut AppState, cursor_move: CursorMove) {
    with_textarea(state, |textarea| {
        // Cancel selection if active before moving cursor
        if textarea.is_selecting() {
            textarea.cancel_selection();
        }
        textarea.move_cursor(cursor_move);
    });
}

/// Execute a selection operation (start selection if not active, then move cursor)
pub fn select_with_cursor_move(state: &mut AppState, cursor_move: CursorMove) {
    with_textarea(state, |textarea| {
        if !textarea.is_selecting() {
            textarea.start_selection();
        }
        textarea.move_cursor(cursor_move);
    });
}

/// Execute a simple textarea method call
pub fn textarea_operation<F>(state: &mut AppState, operation: F)
where
    F: FnOnce(&mut TextArea<'_>),
{
    with_textarea(state, operation);
}