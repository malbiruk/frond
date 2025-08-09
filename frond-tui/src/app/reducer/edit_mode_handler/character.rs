use crate::app::state::AppState;
use super::utils::textarea_operation;

pub fn handle_delete_char(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.delete_char();
    });
}

pub fn handle_delete_next_char(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.delete_next_char();
    });
}

pub fn handle_insert_newline(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.insert_newline();
    });
}