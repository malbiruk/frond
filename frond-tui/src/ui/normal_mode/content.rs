use crate::app::AppState;
use ratatui::prelude::Alignment;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap};
use ratatui::{Frame, layout::Rect};

pub fn render(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let viewport_height = area.height as usize;
    let viewport_width = area.width;

    update_focus_from_visible_content(app_state, viewport_height, viewport_width);
    app_state.update_scrollbar_state(viewport_height, viewport_width);

    let messages = app_state.current_messages();

    if messages.is_empty() {
        render_empty_message(frame, area);
        return;
    }

    render_messages_with_scroll(frame, area, messages, app_state, viewport_width);
    render_scrollbar(frame, area, app_state);
}

fn update_focus_from_visible_content(
    app_state: &mut AppState,
    viewport_height: usize,
    viewport_width: u16,
) {
    let messages_vec = app_state.current_messages();
    let messages = &messages_vec;
    if messages.is_empty() {
        app_state.focused_message_id = None;
        return;
    }

    let total_content_height = get_total_content_height_for_messages(messages, viewport_width);

    let min_scroll = -(viewport_height as isize - 3);
    let max_scroll = total_content_height as isize - 3;

    let clamped_scroll_offset = app_state.scroll_offset.clamp(min_scroll, max_scroll);
    let center_line = clamped_scroll_offset + viewport_height as isize / 2;

    let focused_message_id = if center_line < 0 {
        messages.first().map(|m| m.id())
    } else if center_line >= total_content_height as isize {
        messages.last().map(|m| m.id())
    } else {
        let mut current_line = 0_isize;
        let mut found_id = None;
        for message in messages {
            let message_height = calculate_message_height_lines(message, viewport_width) as isize;
            if current_line + message_height > center_line {
                found_id = Some(message.id());
                break;
            }
            current_line += message_height;
        }
        found_id.or_else(|| messages.last().map(|m| m.id()))
    };

    app_state.scroll_offset = clamped_scroll_offset;
    app_state.focused_message_id = focused_message_id;
}

fn get_total_content_height_for_messages(
    messages: &[&frond_core::Message],
    viewport_width: u16,
) -> usize {
    messages
        .iter()
        .map(|msg| calculate_message_height_lines(msg, viewport_width))
        .sum::<usize>()
        .max(1)
}

fn calculate_message_height_lines(message: &frond_core::Message, viewport_width: u16) -> usize {
    let border_size = 2;
    let padding = 2;
    let content_width = viewport_width.saturating_sub(border_size + padding) as usize;
    calculate_wrapped_line_count(message.content(), content_width) + border_size as usize
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
    if line_count == 0 { 1 } else { line_count }
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
    viewport_width: u16,
) {
    let content_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width.saturating_sub(1),
        height: area.height,
    };

    // Handle negative scroll offset
    let mut y_offset = content_area.y as isize - app_state.scroll_offset;

    for message in &messages {
        let is_focused = app_state.focused_message_id == Some(message.id());
        let message_height = calculate_message_height_lines(message, viewport_width) as isize;

        // Check if this message is visible in the viewport
        let message_bottom = y_offset + message_height;
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

        y_offset += message_height;

        // Early exit if we're below the viewport
        if y_offset >= viewport_bottom {
            break;
        }
    }
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
