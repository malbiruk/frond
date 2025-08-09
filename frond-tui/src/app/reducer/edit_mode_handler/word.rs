use crate::app::state::AppState;
use super::utils::textarea_operation;

pub fn handle_delete_word(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.delete_word();
    });
}

pub fn handle_delete_next_word(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.delete_next_word();
    });
}