use crate::actions::EditModeAction;
use crate::app::state::AppState;

mod clipboard;
mod core;
mod cursor_movement;
mod character;
mod line;
mod selection; 
mod utils;
mod word;

pub use core::handle_edit_mode_raw_input;

pub fn handle_edit_mode_action(state: &mut AppState, action: EditModeAction) {
    match action {
        // Core operations
        EditModeAction::ExitCurrentMode => core::handle_exit_current_mode(state),
        EditModeAction::SubmitMessage => core::handle_submit_message(state),
        
        // Character operations
        EditModeAction::DeleteChar => character::handle_delete_char(state),
        EditModeAction::DeleteNextChar => character::handle_delete_next_char(state),
        EditModeAction::InsertNewline => character::handle_insert_newline(state),
        
        // Line operations
        EditModeAction::DeleteLineByEnd => line::handle_delete_line_by_end(state),
        EditModeAction::DeleteLineByHead => line::handle_delete_line_by_head(state),
        EditModeAction::DeleteLine => line::handle_delete_line(state),
        
        // Word operations
        EditModeAction::DeleteWord => word::handle_delete_word(state),
        EditModeAction::DeleteNextWord => word::handle_delete_next_word(state),
        
        // Clipboard operations
        EditModeAction::Copy => clipboard::handle_copy(state),
        EditModeAction::Cut => clipboard::handle_cut(state),
        EditModeAction::Paste => clipboard::handle_paste(state),
        EditModeAction::SelectAll => clipboard::handle_select_all(state),
        EditModeAction::Undo => clipboard::handle_undo(state),
        EditModeAction::Redo => clipboard::handle_redo(state),
        
        // Cursor movement - character level
        EditModeAction::MoveCursorForward => cursor_movement::handle_move_cursor_forward(state),
        EditModeAction::MoveCursorBack => cursor_movement::handle_move_cursor_back(state),
        EditModeAction::MoveCursorUp => cursor_movement::handle_move_cursor_up(state),
        EditModeAction::MoveCursorDown => cursor_movement::handle_move_cursor_down(state),
        
        // Cursor movement - word level
        EditModeAction::MoveCursorWordForward => cursor_movement::handle_move_cursor_word_forward(state),
        EditModeAction::MoveCursorWordEnd => cursor_movement::handle_move_cursor_word_end(state),
        EditModeAction::MoveCursorWordBack => cursor_movement::handle_move_cursor_word_back(state),
        
        // Cursor movement - line boundaries
        EditModeAction::MoveCursorEnd => cursor_movement::handle_move_cursor_end(state),
        EditModeAction::MoveCursorHead => cursor_movement::handle_move_cursor_head(state),
        
        // Cursor movement - document boundaries
        EditModeAction::MoveCursorTop => cursor_movement::handle_move_cursor_top(state),
        EditModeAction::MoveCursorBottom => cursor_movement::handle_move_cursor_bottom(state),
        
        // Cursor movement - paragraph level
        EditModeAction::MoveCursorParagraphForward => cursor_movement::handle_move_cursor_paragraph_forward(state),
        EditModeAction::MoveCursorParagraphBack => cursor_movement::handle_move_cursor_paragraph_back(state),
        
        // Selection operations
        EditModeAction::SelectForward => selection::handle_select_forward(state),
        EditModeAction::SelectBack => selection::handle_select_back(state),
        EditModeAction::SelectUp => selection::handle_select_up(state),
        EditModeAction::SelectDown => selection::handle_select_down(state),
        EditModeAction::SelectWordForward => selection::handle_select_word_forward(state),
        EditModeAction::SelectWordBack => selection::handle_select_word_back(state),
        EditModeAction::SelectToEnd => selection::handle_select_to_end(state),
        EditModeAction::SelectToHead => selection::handle_select_to_head(state),
        EditModeAction::SelectLine => selection::handle_select_line(state),
        EditModeAction::SelectToTop => selection::handle_select_to_top(state),
        EditModeAction::SelectToBottom => selection::handle_select_to_bottom(state),
    }
}