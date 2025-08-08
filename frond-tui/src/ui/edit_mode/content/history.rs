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
    let messages = get_filtered_messages(app_state, exclude_message_id);
    if messages.is_empty() {
        return;
    }

    let scroll_info = calculate_scroll(area, &messages);
    render_visible_messages(frame, area, &messages, scroll_info, app_state);
}

struct ScrollInfo {
    scroll_offset: usize,
}

fn get_filtered_messages(app_state: &AppState, exclude_id: Uuid) -> Vec<&frond_core::Message> {
    app_state
        .current_messages()
        .into_iter()
        .filter(|m| m.id() != exclude_id)
        .collect()
}

fn calculate_scroll(area: Rect, messages: &[&frond_core::Message]) -> ScrollInfo {
    let total_height: usize = messages
        .iter()
        .map(|m| calculate_message_height(m, area.width))
        .sum();

    let available_height = area.height as usize;
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
            message::render_message(frame, visible_portion.area, message, app_state);
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
    let visible_height = (message_height - skip_lines)
        .min((area.y + area.height - render_y) as usize);

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