//! Widget creation and styling utilities
//!
//! Functions for creating and styling message widgets, blocks, and other UI components.

use super::layout_calc;
use crate::app::AppState;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap};
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
    
    let theme = get_message_theme(message, app_state);
    let block = create_message_block(app_state, theme, is_focused);
    let scroll_offset = layout_calc::calculate_message_scroll_offset(y_offset, content_area);

    let text_color = if is_focused {
        app_state.config.theme.focused.text_color
    } else {
        theme.text_color
    };

    Paragraph::new(text)
        .block(block)
        .style(Style::default().fg(text_color))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0))
}

/// Create a message block with title and appropriate styling
pub fn create_message_block(
    app_state: &AppState,
    theme: &crate::config::theme::MessageTheme,
    is_focused: bool,
) -> Block<'static> {
    let title = format!(" {} ", theme.display_name);
    let base_block = Block::bordered()
        .title(title)
        .title_alignment(Alignment::Left)
        .title_style(Style::default().fg(theme.title_color));

    apply_focus_styling(app_state, base_block, theme, is_focused)
}

/// Apply focus-specific styling to a block
pub fn apply_focus_styling(
    app_state: &AppState,
    block: Block<'static>,
    theme: &crate::config::theme::MessageTheme,
    is_focused: bool,
) -> Block<'static> {
    if is_focused {
        let focused = &app_state.config.theme.focused;
        block
            .border_type(focused.border_type)
            .style(Style::default().fg(focused.frame_color))
    } else {
        block
            .border_type(theme.border_type)
            .style(Style::default().fg(theme.frame_color))
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

/// Get the theme for a message based on its role
pub fn get_message_theme<'a>(
    message: &frond_core::Message,
    app_state: &'a AppState,
) -> &'a crate::config::theme::MessageTheme {
    match message.role() {
        frond_core::Role::Assistant => &app_state.config.theme.assistant,
        frond_core::Role::User => &app_state.config.theme.user,
    }
}