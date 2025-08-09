use super::layout_calc;
use crate::app::AppState;
use ratatui::{Frame, layout::Rect, prelude::Alignment, style::{Color, Style}, widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, Wrap}};

pub fn create_message_widget_from_data(
    message_id: uuid::Uuid,
    content: &str,
    role: frond_core::Role,
    app_state: &mut AppState,
    y_offset: isize,
    content_area: Rect,
) -> Paragraph<'static> {
    let is_focused = app_state.focused_message_id == Some(message_id);
    
    let text = app_state.get_cached_highlighted_text(message_id)
        .unwrap_or_else(|| ratatui::text::Text::raw(content.to_string()));
    
    let block = create_message_block(role, app_state, is_focused);
    let scroll_offset = layout_calc::calculate_message_scroll_offset(y_offset, content_area);

    let paragraph = Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(Color::Reset))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0));

    let height = paragraph.line_count(content_area.width);
    app_state.cache_height(message_id, content_area.width, height);

    paragraph
}


fn create_message_block(
    role: frond_core::Role,
    app_state: &AppState,
    is_focused: bool,
) -> Block<'static> {
    let (display_name, title_color) = match role {
        frond_core::Role::Assistant => ("Assistant", app_state.config.theme.highlight_color),
        frond_core::Role::User => ("User", Color::Reset),
    };
    
    let base_block = Block::bordered()
        .title(format!(" {} ", display_name))
        .title_alignment(Alignment::Left)
        .title_style(Style::default().fg(title_color));

    apply_focus_styling(role, app_state, base_block, is_focused)
}


fn apply_focus_styling(
    role: frond_core::Role,
    app_state: &AppState,
    block: Block<'static>,
    is_focused: bool,
) -> Block<'static> {
    let border_color = match role {
        frond_core::Role::Assistant => app_state.config.theme.highlight_color,
        frond_core::Role::User => Color::Reset,
    };
    
    let border_type = if is_focused { BorderType::Double } else { BorderType::Plain };
    
    block
        .border_type(border_type)
        .style(Style::default().fg(border_color))
}


pub fn render_scrollbar(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
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

pub fn render_empty_message(frame: &mut Frame, area: Rect) {
    let empty_msg = Paragraph::new("No messages in this branch")
        .alignment(Alignment::Center)
        .style(Style::default());
    frame.render_widget(empty_msg, area);
}

