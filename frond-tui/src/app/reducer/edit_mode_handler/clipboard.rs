use crate::app::state::AppState;
use super::utils::textarea_operation;

pub fn handle_undo(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.undo();
    });
}

pub fn handle_redo(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.redo();
    });
}

pub fn handle_copy(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.copy();
    });
}

pub fn handle_cut(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.cut();
    });
}

pub fn handle_paste(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.paste();
    });
}

pub fn handle_select_all(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.select_all();
    });
}