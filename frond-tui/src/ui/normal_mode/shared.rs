use ratatui::text::Line;
use ratatui::text::Span;

pub fn calculate_dash_count(left_len: usize, right_len: usize, width: u16) -> usize {
    let available_width = width as usize;
    available_width.saturating_sub(left_len + right_len + 2)
}

pub fn create_line_with_dashes(
    left_text: String,
    right_text: String,
    dash_count: usize,
) -> Line<'static> {
    Line::from(vec![
        Span::raw(left_text),
        Span::raw(" "),
        Span::raw("─".repeat(dash_count)),
        Span::raw(" "),
        Span::raw(right_text),
    ])
}

pub fn wrap_text(text: &str, width: usize) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut current_line = text;

    while current_line.len() > width {
        if let Some(space_pos) = current_line[..width].rfind(' ') {
            lines.push(&current_line[..space_pos]);
            current_line = &current_line[space_pos + 1..];
        } else {
            lines.push(&current_line[..width]);
            current_line = &current_line[width..];
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
