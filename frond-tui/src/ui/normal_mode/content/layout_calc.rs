//! Layout calculation utilities
//!
//! Pure functions for calculating positions, areas, and visibility within the content area.

use crate::ui::normal_mode::scrolling;
use ratatui::layout::Rect;

/// Calculate the content area by subtracting scrollbar width
pub fn calculate_content_area(area: Rect) -> Rect {
    Rect {
        x: area.x,
        y: area.y,
        width: area.width.saturating_sub(1),
        height: area.height,
    }
}

/// Calculate the initial Y offset for rendering based on scroll position
pub fn calculate_initial_y_offset(content_area: Rect, scroll_offset: isize) -> isize {
    content_area.y as isize - scroll_offset
}

/// Check if a message should be rendered based on viewport visibility
pub fn should_render_message(
    y_offset: isize,
    content_area: Rect,
    message: &frond_core::Message,
    viewport_width: u16,
) -> bool {
    let message_height = calculate_message_height_for_rendering(message, viewport_width) as isize;
    let message_bottom = y_offset + message_height;
    let viewport_bottom = (content_area.y + content_area.height) as isize;

    message_bottom > content_area.y as isize && y_offset < viewport_bottom
}

/// Calculate the visible area for a message within the viewport
pub fn calculate_visible_message_area(
    content_area: Rect,
    y_offset: isize,
    message: &frond_core::Message,
    viewport_width: u16,
) -> Rect {
    let message_height =
        scrolling::calculate_message_display_height(message, viewport_width) as isize;
    let message_bottom = y_offset + message_height;
    let viewport_bottom = (content_area.y + content_area.height) as isize;

    let visible_start = y_offset.max(content_area.y as isize) as u16;
    let visible_end = message_bottom.min(viewport_bottom) as u16;
    let visible_height = visible_end.saturating_sub(visible_start);

    Rect {
        x: content_area.x,
        y: visible_start,
        width: content_area.width,
        height: visible_height,
    }
}

/// Check if a Y offset is below the visible viewport
pub fn is_below_viewport(y_offset: isize, content_area: Rect) -> bool {
    let viewport_bottom = (content_area.y + content_area.height) as isize;
    y_offset >= viewport_bottom
}

pub fn calculate_message_height_for_rendering(
    message: &frond_core::Message,
    viewport_width: u16,
) -> usize {
    scrolling::calculate_message_display_height(message, viewport_width)
}

pub fn calculate_message_heights_before(
    messages: &[&frond_core::Message],
    target_message_id: uuid::Uuid,
    viewport_width: u16,
) -> isize {
    let mut total_height = 0isize;
    
    for message in messages {
        if message.id() == target_message_id {
            break;
        }
        total_height += calculate_message_height_for_rendering(message, viewport_width) as isize;
    }
    
    total_height
}

pub fn calculate_message_scroll_offset(y_offset: isize, content_area: Rect) -> u16 {
    if y_offset < content_area.y as isize {
        (content_area.y as isize - y_offset) as u16
    } else {
        0
    }
}