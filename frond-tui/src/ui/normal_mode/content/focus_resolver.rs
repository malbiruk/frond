//! Focus request resolution logic
//!
//! Handles all scrolling request types and resolves them to specific message focus
//! and scroll offset pairs.

use crate::app::state::ScrollingRequest;
use crate::ui::normal_mode::scrolling;
use uuid::Uuid;

/// Resolves a pending focus request and returns the target message ID and scroll offset
pub fn resolve_pending_focus_request(
    request: ScrollingRequest,
    messages: &[&frond_core::Message],
    current_scroll_offset: isize,
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(Uuid, isize)> {
    match request {
        ScrollingRequest::ScrollToMessage(message_id) => resolve_focus_specific_message(
            messages,
            message_id,
            viewport_height,
            viewport_width,
        ),
        ScrollingRequest::ScrollToLastMessage => {
            resolve_focus_last_message(messages, viewport_height, viewport_width)
        }
        ScrollingRequest::ScrollToMessageWithSameIndexOrLast { previous_index } => {
            resolve_focus_same_index_or_last(
                messages,
                previous_index,
                viewport_height,
                viewport_width,
            )
        }
        ScrollingRequest::ScrollToTop => {
            resolve_focus_top(messages, viewport_height, viewport_width)
        }
        ScrollingRequest::ScrollToBottom => {
            resolve_focus_bottom(messages, viewport_height, viewport_width)
        }
        ScrollingRequest::ScrollPageUp => resolve_focus_scroll_by(
            messages,
            current_scroll_offset,
            -(viewport_height as isize),
            viewport_height,
            viewport_width,
        ),
        ScrollingRequest::ScrollPageDown => resolve_focus_scroll_by(
            messages,
            current_scroll_offset,
            viewport_height as isize,
            viewport_height,
            viewport_width,
        ),
        ScrollingRequest::ScrollHalfPageUp => resolve_focus_scroll_by(
            messages,
            current_scroll_offset,
            -((viewport_height as f32 / 2.0).ceil() as isize),
            viewport_height,
            viewport_width,
        ),
        ScrollingRequest::ScrollHalfPageDown => resolve_focus_scroll_by(
            messages,
            current_scroll_offset,
            (viewport_height as f32 / 2.0).ceil() as isize,
            viewport_height,
            viewport_width,
        ),
    }
}

fn resolve_focus_specific_message(
    messages: &[&frond_core::Message],
    message_id: Uuid,
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(Uuid, isize)> {
    scrolling::calculate_scroll_to_focus_message(
        messages,
        message_id,
        viewport_height as isize,
        viewport_width,
    )
    .map(|scroll_offset| (message_id, scroll_offset))
}

fn resolve_focus_last_message(
    messages: &[&frond_core::Message],
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(Uuid, isize)> {
    if let Some(last_message) = messages.last() {
        resolve_focus_specific_message(messages, last_message.id(), viewport_height, viewport_width)
    } else {
        None
    }
}

fn resolve_focus_same_index_or_last(
    messages: &[&frond_core::Message],
    previous_index: usize,
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(Uuid, isize)> {
    let target_index = previous_index.min(messages.len().saturating_sub(1));
    if let Some(target_message) = messages.get(target_index) {
        resolve_focus_specific_message(
            messages,
            target_message.id(),
            viewport_height,
            viewport_width,
        )
    } else {
        None
    }
}

fn resolve_focus_top(
    messages: &[&frond_core::Message],
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(Uuid, isize)> {
    messages.first().map(|msg| {
        let scroll_offset = scrolling::calculate_scroll_to_focus_message(
            messages,
            msg.id(),
            viewport_height as isize,
            viewport_width,
        )
        .unwrap_or(0);
        (msg.id(), scroll_offset)
    })
}

fn resolve_focus_bottom(
    messages: &[&frond_core::Message],
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(Uuid, isize)> {
    resolve_focus_last_message(messages, viewport_height, viewport_width)
}

fn resolve_focus_scroll_by(
    messages: &[&frond_core::Message],
    current_scroll_offset: isize,
    delta: isize,
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(Uuid, isize)> {
    let total_content_height = scrolling::calculate_total_content_height(messages, viewport_width);
    let new_scroll_offset = scrolling::clamp_scroll_offset(
        current_scroll_offset + delta,
        viewport_height,
        total_content_height,
    );
    let focused_message_id = scrolling::update_focused_message_from_scroll(
        messages,
        new_scroll_offset,
        viewport_height as isize,
        viewport_width,
    );
    focused_message_id.map(|id| (id, new_scroll_offset))
}