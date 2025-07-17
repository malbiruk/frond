use crate::app::AppState;
use ratatui::prelude::Alignment;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap};
use ratatui::{Frame, layout::Rect};

pub fn render(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    // Update focus and scrollbar state first
    let viewport_height = area.height as usize;
    app_state.update_focused_message_from_scroll(viewport_height);
    app_state.update_scrollbar_state(viewport_height);

    let messages = app_state.current_messages();

    if messages.is_empty() {
        render_empty_message(frame, area);
        return;
    }

    render_messages_with_scroll(frame, area, messages, app_state);
    render_scrollbar(frame, area, app_state);
}

fn render_empty_message(frame: &mut Frame, area: Rect) {
    let empty_msg = create_empty_message_widget();
    frame.render_widget(empty_msg, area);
}

fn create_empty_message_widget() -> Paragraph<'static> {
    Paragraph::new("No messages in this branch")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Gray))
}

fn render_messages_with_scroll(
    frame: &mut Frame,
    area: Rect,
    messages: Vec<&frond_core::Message>,
    app_state: &AppState,
) {
    // Reserve space for scrollbar
    let content_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width.saturating_sub(1),
        height: area.height,
    };

    // Calculate which messages are visible and their positions
    let mut y_offset = content_area.y as isize - app_state.scroll_offset as isize;

    for message in &messages {
        let is_focused = app_state.focused_message_id == Some(message.id());
        let message_height = calculate_message_height(message, content_area.width);

        // Check if this message is visible in the viewport
        let message_bottom = y_offset + message_height as isize;
        let viewport_bottom = (content_area.y + content_area.height) as isize;

        if message_bottom > content_area.y as isize && y_offset < viewport_bottom {
            // Message is at least partially visible
            let visible_start = y_offset.max(content_area.y as isize) as u16;
            let visible_end = message_bottom.min(viewport_bottom) as u16;
            let visible_height = visible_end.saturating_sub(visible_start);

            if visible_height > 0 {
                let message_area = Rect {
                    x: content_area.x,
                    y: visible_start,
                    width: content_area.width,
                    height: visible_height,
                };

                let message_widget =
                    create_message_widget(message, app_state, is_focused, y_offset, content_area);
                frame.render_widget(message_widget, message_area);
            }
        }

        y_offset += message_height as isize;

        // Early exit if we're below the viewport
        if y_offset >= viewport_bottom {
            break;
        }
    }
}

fn calculate_message_height(message: &frond_core::Message, width: u16) -> u16 {
    let content_width = width.saturating_sub(4); // Account for borders and padding
    let content = message.content();

    // Calculate wrapped lines
    let mut line_count = 0;
    for line in content.lines() {
        if line.is_empty() {
            line_count += 1;
        } else {
            let chars_per_line = content_width as usize;
            line_count += (line.len() + chars_per_line - 1) / chars_per_line; // Ceiling division
        }
    }

    if line_count == 0 {
        line_count = 1; // At least one line for empty content
    }

    line_count as u16 + 2 // +2 for top and bottom borders
}

fn create_message_widget(
    message: &frond_core::Message,
    app_state: &AppState,
    is_focused: bool,
    y_offset: isize,
    content_area: Rect,
) -> Paragraph<'static> {
    let theme = get_message_theme(message, app_state);
    let block = create_message_block(theme, is_focused);

    // Calculate scroll offset for this specific message if it's partially off-screen
    let message_scroll_y = if y_offset < content_area.y as isize {
        (content_area.y as isize - y_offset) as u16
    } else {
        0
    };

    Paragraph::new(message.content().to_string())
        .block(block)
        .style(Style::default().fg(theme.text_color))
        .wrap(Wrap { trim: false })
        .scroll((message_scroll_y, 0))
}

fn create_message_block(
    theme: &crate::config::theme::MessageTheme,
    is_focused: bool,
) -> Block<'static> {
    let title = format!(" {} ", theme.display_name);

    let mut block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Left)
        .title_style(Style::default().fg(theme.title_color));

    if is_focused {
        block = block
            .border_type(ratatui::widgets::BorderType::Double)
            .style(Style::default().fg(Color::White));
    } else {
        block = block.style(Style::default().fg(theme.frame_color));
    }

    block
}

fn render_scrollbar(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"));

    let scrollbar_area = Rect {
        x: area.x + area.width.saturating_sub(1),
        y: area.y,
        width: 1,
        height: area.height,
    };

    frame.render_stateful_widget(scrollbar, scrollbar_area, &mut app_state.scrollbar_state);
}

fn get_message_theme<'a>(
    message: &frond_core::Message,
    app_state: &'a AppState,
) -> &'a crate::config::theme::MessageTheme {
    match message.role() {
        frond_core::Role::Assistant => &app_state.config.theme.assistant,
        frond_core::Role::User => &app_state.config.theme.user,
    }
}
