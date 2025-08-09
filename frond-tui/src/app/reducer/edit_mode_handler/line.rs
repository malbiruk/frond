use super::utils::textarea_operation;
use crate::app::state::AppState;

pub fn handle_delete_line_by_end(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.delete_line_by_end();
    });
}

pub fn handle_delete_line_by_head(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.delete_line_by_head();
    });
}

pub fn handle_delete_line(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.move_cursor(tui_textarea::CursorMove::Head);
        textarea.delete_line_by_end();
        textarea.delete_next_char();
    });
}
