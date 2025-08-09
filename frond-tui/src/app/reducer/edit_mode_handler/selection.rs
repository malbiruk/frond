use super::utils::select_with_cursor_move;
use crate::app::state::AppState;
use tui_textarea::CursorMove;

// Character level selection
pub fn handle_select_forward(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::Forward);
}

pub fn handle_select_back(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::Back);
}

pub fn handle_select_up(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::Up);
}

pub fn handle_select_down(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::Down);
}

// Word level selection
pub fn handle_select_word_forward(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::WordForward);
}

pub fn handle_select_word_back(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::WordBack);
}

// Line boundaries selection
pub fn handle_select_to_end(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::End);
}

pub fn handle_select_to_head(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::Head);
}

// Document boundaries selection
pub fn handle_select_to_top(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::Top);
}

pub fn handle_select_to_bottom(state: &mut AppState) {
    select_with_cursor_move(state, CursorMove::Bottom);
}

// Entire line selection
pub fn handle_select_line(state: &mut AppState) {
    super::utils::textarea_operation(state, |textarea| {
        textarea.move_cursor(CursorMove::Head);
        textarea.start_selection();
        textarea.move_cursor(CursorMove::End);
        textarea.move_cursor(CursorMove::Forward);
    });
}
