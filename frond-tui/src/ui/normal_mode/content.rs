use crate::app::AppState;
use ratatui::prelude::Alignment;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::Block;
use ratatui::widgets::Paragraph;
use ratatui::widgets::Wrap;
use ratatui::{Frame, layout::Rect};

use super::shared::wrap_text;

pub fn render(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let messages = app_state.current_messages();

    if messages.is_empty() {
        render_empty_message(frame, area);
        return;
    }

    render_messages(frame, area, messages, app_state);
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

fn render_messages(
    frame: &mut Frame,
    area: Rect,
    messages: Vec<&frond_core::Message>,
    app_state: &AppState,
) {
    let mut y_offset = area.top();

    for message in messages {
        if has_reached_bottom(y_offset, area) {
            break;
        }

        let message_height = calculate_message_height(message, area, y_offset);

        if is_message_too_small(message_height) {
            break;
        }

        let message_area = create_message_area(area, y_offset, message_height);
        let message_widget = create_message_widget(message, app_state);

        frame.render_widget(message_widget, message_area);
        y_offset += message_height;
    }
}

fn has_reached_bottom(y_offset: u16, area: Rect) -> bool {
    y_offset >= area.bottom()
}

fn calculate_message_height(message: &frond_core::Message, area: Rect, y_offset: u16) -> u16 {
    let content_width = area.width.saturating_sub(4);
    let wrapped_lines = wrap_text(message.content(), content_width as usize);
    let required_height = wrapped_lines.len() + 2;
    let available_height = area.bottom().saturating_sub(y_offset);
    required_height.min(available_height as usize) as u16
}

fn is_message_too_small(message_height: u16) -> bool {
    message_height < 3
}

fn create_message_area(area: Rect, y_offset: u16, message_height: u16) -> Rect {
    Rect {
        x: area.x,
        y: y_offset,
        width: area.width,
        height: message_height,
    }
}

fn create_message_widget(
    message: &frond_core::Message,
    app_state: &AppState,
) -> Paragraph<'static> {
    let theme = get_message_theme(message, app_state);
    let block = create_message_block(theme);

    Paragraph::new(message.content().to_string())
        .block(block)
        .style(Style::default().fg(theme.text_color))
        .wrap(Wrap { trim: false })
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

fn create_message_block(theme: &crate::config::theme::MessageTheme) -> Block<'static> {
    let title = format!(" {} ", theme.display_name);

    Block::bordered()
        .title(title)
        .title_alignment(Alignment::Left)
        .title_style(Style::default().fg(theme.title_color))
        .style(Style::default().fg(theme.frame_color))
}
