use crate::app::AppState;
use super::message;
use ratatui::{Frame, layout::Rect};
use uuid::Uuid;

pub fn render_excluding_message(
    frame: &mut Frame,
    area: Rect,
    app_state: &mut AppState,
    exclude_message_id: Uuid,
) {
    let messages = get_filtered_messages_before(app_state, exclude_message_id);
    if messages.is_empty() {
        return;
    }

    // For history above edited message, we want it positioned at the bottom of the area
    // (right above the textarea) with no gap
    let scroll_info = calculate_scroll_to_bottom(area, &messages);
    render_visible_messages(frame, area, &messages, scroll_info, app_state);
}

pub fn render_excluding_message_below(
    frame: &mut Frame,
    area: Rect,
    app_state: &mut AppState,
    exclude_message_id: Uuid,
) {
    let messages = get_filtered_messages_after(app_state, exclude_message_id);
    if messages.is_empty() {
        return;
    }

    // For messages below, we want to show from the beginning (no scroll offset)
    let scroll_info = ScrollInfo { scroll_offset: 0 };
    render_visible_messages(frame, area, &messages, scroll_info, app_state);
}

struct ScrollInfo {
    scroll_offset: usize,
}

fn get_filtered_messages_before(app_state: &AppState, exclude_id: Uuid) -> Vec<&frond_core::Message> {
    let all_messages = app_state.current_messages();
    let mut messages_before = Vec::new();
    
    for message in all_messages {
        if message.id() == exclude_id {
            break; // Stop when we reach the excluded message
        }
        messages_before.push(message);
    }
    
    messages_before
}

fn get_filtered_messages_after(app_state: &AppState, exclude_id: Uuid) -> Vec<&frond_core::Message> {
    let all_messages = app_state.current_messages();
    let mut found_excluded = false;
    let mut messages_after = Vec::new();
    
    for message in all_messages {
        if found_excluded {
            messages_after.push(message);
        } else if message.id() == exclude_id {
            found_excluded = true;
        }
    }
    
    messages_after
}


fn calculate_scroll_to_bottom(area: Rect, messages: &[&frond_core::Message]) -> ScrollInfo {
    let total_height: usize = messages
        .iter()
        .map(|m| calculate_message_height(m, area.width))
        .sum();

    let available_height = area.height as usize;
    
    // Position messages so they end at the bottom of the area (no gap)
    let scroll_offset = total_height.saturating_sub(available_height);

    ScrollInfo { scroll_offset }
}

fn render_visible_messages(
    frame: &mut Frame,
    area: Rect,
    messages: &[&frond_core::Message],
    scroll_info: ScrollInfo,
    app_state: &AppState,
) {
    let mut current_height = 0usize;
    let mut render_y = area.y;

    for message in messages {
        let message_height = calculate_message_height(message, area.width);

        if should_skip_message(current_height, message_height, scroll_info.scroll_offset) {
            current_height += message_height;
            continue;
        }

        if render_y >= area.y + area.height {
            break;
        }

        let visible_portion = calculate_visible_portion(
            area, render_y, current_height, message_height, scroll_info.scroll_offset
        );

        if visible_portion.height > 0 {
            let skip_lines = scroll_info.scroll_offset.saturating_sub(current_height);
            message::render_message_with_scroll(frame, visible_portion.area, message, app_state, skip_lines as u16);
            render_y += visible_portion.height;
        }

        current_height += message_height;
    }
}

struct VisiblePortion {
    area: Rect,
    height: u16,
}

fn should_skip_message(current_height: usize, message_height: usize, scroll_offset: usize) -> bool {
    current_height + message_height <= scroll_offset
}

fn calculate_visible_portion(
    area: Rect,
    render_y: u16,
    current_height: usize,
    message_height: usize,
    scroll_offset: usize,
) -> VisiblePortion {
    let skip_lines = scroll_offset.saturating_sub(current_height);
    let content_height = message_height - skip_lines;
    let available_height = (area.y + area.height - render_y) as usize;
    let visible_height = content_height.min(available_height);

    VisiblePortion {
        area: Rect {
            x: area.x,
            y: render_y,
            width: area.width,
            height: visible_height as u16,
        },
        height: visible_height as u16,
    }
}

fn calculate_message_height(message: &frond_core::Message, width: u16) -> usize {
    crate::ui::normal_mode::scrolling::calculate_message_display_height(message, width)
}