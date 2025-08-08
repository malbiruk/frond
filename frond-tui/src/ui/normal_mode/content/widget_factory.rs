//! Widget creation and styling utilities
//!
//! Functions for creating and styling message widgets, blocks, and other UI components.

use super::layout_calc;
use crate::app::AppState;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph, Scrollbar, ScrollbarOrientation, Wrap};
use ratatui::Frame;

/// Create a complete message widget with highlighting, styling, and scrolling
pub fn create_message_widget(
    message: &frond_core::Message,
    app_state: &AppState,
    y_offset: isize,
    content_area: Rect,
) -> Paragraph<'static> {
    let is_focused = app_state.focused_message_id == Some(message.id());
    
    // Use cached highlighting if available, otherwise plain text
    let text = app_state.highlight_cache.get(&message.id())
        .cloned()
        .unwrap_or_else(|| {
            // Plain text fallback for non-visible messages or cache misses
            ratatui::text::Text::raw(message.content().to_string())
        });
    
    let block = create_message_block(message, app_state, is_focused);
    let scroll_offset = layout_calc::calculate_message_scroll_offset(y_offset, content_area);

    let text_color = Color::Reset;

    Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(text_color))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0))
}

/// Create a message block with title and appropriate styling
pub fn create_message_block(
    message: &frond_core::Message,
    app_state: &AppState,
    is_focused: bool,
) -> Block<'static> {
    let (display_name, title_color) = match message.role() {
        frond_core::Role::Assistant => ("Assistant", app_state.config.theme.highlight_color),
        frond_core::Role::User => ("User", Color::Reset),
    };
    let title = format!(" {} ", display_name);
    let base_block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Left)
        .title_style(Style::default().fg(title_color));

    apply_focus_styling(message, app_state, base_block, is_focused)
}

/// Apply focus-specific styling to a block
pub fn apply_focus_styling(
    message: &frond_core::Message,
    app_state: &AppState,
    block: Block<'static>,
    is_focused: bool,
) -> Block<'static> {
    let border_color = match message.role() {
        frond_core::Role::Assistant => app_state.config.theme.highlight_color,
        frond_core::Role::User => Color::Reset,
    };
    
    if is_focused {
        block
            .border_type(BorderType::Double)
            .style(Style::default().fg(border_color))
    } else {
        block
            .border_type(BorderType::Plain)
            .style(Style::default().fg(border_color))
    }
}

/// Render a scrollbar in the specified area
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

/// Render an empty message when no content is available
pub fn render_empty_message(frame: &mut Frame, area: Rect) {
    let empty_msg = Paragraph::new("No messages in this branch")
        .alignment(Alignment::Center)
        .style(Style::default());
    frame.render_widget(empty_msg, area);
}

