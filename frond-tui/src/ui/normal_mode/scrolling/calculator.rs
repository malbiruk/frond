use frond_core::Message;
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

pub fn calculate_message_display_height(message: &Message, viewport_width: u16) -> usize {
    // Create the same paragraph configuration as used in rendering
    let text = Text::raw(message.content());
    let block = Block::default().borders(Borders::ALL);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });

    // Use ratatui's line_count method to get accurate wrapped height
    paragraph.line_count(viewport_width)
}

pub fn calculate_total_content_height(messages: &[&Message], viewport_width: u16) -> usize {
    let total: usize = messages
        .iter()
        .map(|msg| calculate_message_display_height(msg, viewport_width))
        .sum();
    total.max(1)
}
