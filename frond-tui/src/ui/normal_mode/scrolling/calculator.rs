use frond_core::Message;
use crate::app::AppState;

/// Fast message height calculation using cached heights from rendered widgets, with fallback
pub fn calculate_message_display_height_with_cache(
    message: &Message, 
    viewport_width: u16, 
    app_state: &AppState
) -> usize {
    // Try cache first
    if let Some(cached_height) = app_state.get_cached_height(message.id(), viewport_width) {
        return cached_height;
    }
    
    // Fallback to simple math if not cached
    calculate_message_display_height_fallback(message, viewport_width)
}

/// Fast message height calculation using simple math instead of expensive ratatui widgets
pub fn calculate_message_display_height(message: &Message, viewport_width: u16) -> usize {
    calculate_message_display_height_fallback(message, viewport_width)
}

/// Fallback calculation when cache miss occurs
fn calculate_message_display_height_fallback(message: &Message, viewport_width: u16) -> usize {
    let content_width = calculate_content_width(viewport_width);
    let line_count = calculate_wrapped_line_count(message.content(), content_width);
    line_count + calculate_border_padding()
}

pub fn calculate_total_content_height_with_cache(
    messages: &[&Message], 
    viewport_width: u16, 
    app_state: &AppState
) -> usize {
    let total: usize = messages
        .iter()
        .map(|msg| calculate_message_display_height_with_cache(msg, viewport_width, app_state))
        .sum();
    total.max(1)
}

pub fn calculate_total_content_height(messages: &[&Message], viewport_width: u16) -> usize {
    let total: usize = messages
        .iter()
        .map(|msg| calculate_message_display_height(msg, viewport_width))
        .sum();
    total.max(1)
}

/// Calculate content width accounting for borders (subtract 4 for left/right borders + padding)
fn calculate_content_width(viewport_width: u16) -> usize {
    viewport_width.saturating_sub(4).max(1) as usize
}

/// Fast wrapped line counting using simple math
fn calculate_wrapped_line_count(content: &str, content_width: usize) -> usize {
    let mut line_count = 0;
    for line in content.lines() {
        if line.is_empty() {
            line_count += 1;
        } else {
            // Simple ceiling division: (len + width - 1) / width  
            line_count += line.len().div_ceil(content_width);
        }
    }
    if line_count == 0 {
        line_count = 1; // Always at least one line
    }
    line_count
}

/// Border padding (top + bottom borders)
fn calculate_border_padding() -> usize {
    2
}