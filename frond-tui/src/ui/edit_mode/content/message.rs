use crate::app::AppState;
use ratatui::{Frame, layout::Rect, style::{Color, Style}, widgets::{Block, BorderType, Paragraph}, prelude::Alignment, text::Text};

pub fn render_message(
    frame: &mut Frame,
    area: Rect,
    message: &frond_core::Message,
    app_state: &AppState,
) {
    render_message_with_scroll(frame, area, message, app_state, 0);
}

pub fn render_message_with_scroll(
    frame: &mut Frame,
    area: Rect,
    message: &frond_core::Message,
    app_state: &AppState,
    scroll_offset: u16,
) {
    let block = create_message_block(message.role(), app_state);
    let content = get_message_content(message, app_state);
    
    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: false })
        .scroll((scroll_offset, 0));

    frame.render_widget(paragraph, area);
}

fn create_message_block(role: &frond_core::Role, app_state: &AppState) -> Block<'static> {
    let (title, border_color) = match role {
        frond_core::Role::Assistant => ("Assistant", app_state.config.theme.highlight_color),
        frond_core::Role::User => ("User", Color::Reset),
    };

    Block::bordered()
        .title(format!(" {} ", title))
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Plain)
        .style(Style::default().fg(border_color))
}

fn get_message_content<'a>(message: &'a frond_core::Message, app_state: &'a AppState) -> Text<'a> {
    app_state.highlight_cache
        .get(&message.id())
        .cloned()
        .unwrap_or_else(|| Text::raw(message.content()))
}