use frond_core::Message;
use crate::app::AppState;

pub fn calculate_message_display_height_with_cache(
    message: &Message, 
    viewport_width: u16, 
    app_state: &AppState
) -> usize {
    if let Some(cached_height) = app_state.get_cached_height(message.id(), viewport_width) {
        return cached_height;
    }
    
    calculate_message_display_height(message, viewport_width)
}

pub fn calculate_message_display_height(message: &Message, viewport_width: u16) -> usize {
    let text = ratatui::text::Text::raw(message.content());
    let block = ratatui::widgets::Block::default().borders(ratatui::widgets::Borders::ALL);
    let paragraph = ratatui::widgets::Paragraph::new(text)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: false });
    paragraph.line_count(viewport_width)
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

