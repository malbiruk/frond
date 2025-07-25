use frond_core::Message;

pub fn calculate_message_display_height(message: &Message, viewport_width: u16) -> usize {
    let content_width = calculate_content_width(viewport_width);
    let line_count = calculate_wrapped_line_count(message.content(), content_width);
    line_count + calculate_border_padding()
}

pub fn calculate_total_content_height(messages: &[&Message], viewport_width: u16) -> usize {
    let total: usize = messages
        .iter()
        .map(|msg| calculate_message_display_height(msg, viewport_width))
        .sum();
    total.max(1)
}

fn calculate_content_width(viewport_width: u16) -> usize {
    viewport_width.saturating_sub(4).max(1) as usize
}

fn calculate_wrapped_line_count(content: &str, content_width: usize) -> usize {
    let mut line_count = 0;
    for line in content.lines() {
        if line.is_empty() {
            line_count += 1;
        } else {
            line_count += line.len().div_ceil(content_width);
        }
    }
    if line_count == 0 {
        line_count = 1;
    }
    line_count
}

fn calculate_border_padding() -> usize {
    2
}
