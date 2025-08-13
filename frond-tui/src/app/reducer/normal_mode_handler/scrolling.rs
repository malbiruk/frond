use crate::app::state::{AppState, ScrollingRequest};
use uuid::Uuid;

pub fn handle_scroll_up(state: &mut AppState) {
    state.scroll_offset = state.scroll_offset.saturating_sub(1);
}

pub fn handle_scroll_down(state: &mut AppState) {
    state.scroll_offset = state.scroll_offset.saturating_add(1);
}

pub fn handle_scroll_page_up(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollPageUp);
}

pub fn handle_scroll_page_down(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollPageDown);
}

pub fn handle_scroll_half_page_up(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollHalfPageUp);
}

pub fn handle_scroll_half_page_down(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollHalfPageDown);
}

pub fn handle_scroll_to_top(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollToTop);
}

pub fn handle_scroll_to_bottom(state: &mut AppState) {
    state.pending_scrolling_request = Some(ScrollingRequest::ScrollToBottom);
}

fn handle_scroll_to_adjacent_message<F>(
    state: &mut AppState,
    message_id: Uuid,
    get_adjacent_index: F,
) where
    F: Fn(usize, usize) -> Option<usize>,
{
    let messages = state.current_messages();
    if let Some(current_index) = messages.iter().position(|m| m.id() == message_id) {
        if let Some(adjacent_index) = get_adjacent_index(current_index, messages.len()) {
            if let Some(adjacent_message) = messages.get(adjacent_index) {
                state.pending_scrolling_request =
                    Some(ScrollingRequest::ScrollToMessage(adjacent_message.id()));
            }
        }
    }
}

pub fn handle_scroll_to_next_message(state: &mut AppState) {
    if let Some(message_id) = state.focused_message_id {
        handle_scroll_to_adjacent_message(state, message_id, |current, len| {
            if current + 1 < len {
                Some(current + 1)
            } else {
                None
            }
        });
    }
}

pub fn handle_scroll_to_previous_message(state: &mut AppState) {
    if let Some(message_id) = state.focused_message_id {
        handle_scroll_to_adjacent_message(state, message_id, |current, _len| {
            if current > 0 { Some(current - 1) } else { None }
        });
    }
}