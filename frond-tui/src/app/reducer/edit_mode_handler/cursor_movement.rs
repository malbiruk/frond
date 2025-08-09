use crate::app::state::AppState;
use tui_textarea::CursorMove;
use super::utils::move_cursor;

// Character level movement
pub fn handle_move_cursor_forward(state: &mut AppState) {
    move_cursor(state, CursorMove::Forward);
}

pub fn handle_move_cursor_back(state: &mut AppState) {
    move_cursor(state, CursorMove::Back);
}

pub fn handle_move_cursor_up(state: &mut AppState) {
    move_cursor(state, CursorMove::Up);
}

pub fn handle_move_cursor_down(state: &mut AppState) {
    move_cursor(state, CursorMove::Down);
}

// Word level movement
pub fn handle_move_cursor_word_forward(state: &mut AppState) {
    move_cursor(state, CursorMove::WordForward);
}

pub fn handle_move_cursor_word_end(state: &mut AppState) {
    move_cursor(state, CursorMove::WordEnd);
}

pub fn handle_move_cursor_word_back(state: &mut AppState) {
    move_cursor(state, CursorMove::WordBack);
}

// Line boundaries
pub fn handle_move_cursor_end(state: &mut AppState) {
    move_cursor(state, CursorMove::End);
}

pub fn handle_move_cursor_head(state: &mut AppState) {
    move_cursor(state, CursorMove::Head);
}

// Document boundaries
pub fn handle_move_cursor_top(state: &mut AppState) {
    move_cursor(state, CursorMove::Top);
}

pub fn handle_move_cursor_bottom(state: &mut AppState) {
    move_cursor(state, CursorMove::Bottom);
}

// Paragraph level movement
pub fn handle_move_cursor_paragraph_forward(state: &mut AppState) {
    move_cursor(state, CursorMove::ParagraphForward);
}

pub fn handle_move_cursor_paragraph_back(state: &mut AppState) {
    move_cursor(state, CursorMove::ParagraphBack);
}