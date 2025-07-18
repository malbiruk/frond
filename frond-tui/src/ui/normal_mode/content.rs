use super::scrolling;
use crate::app::AppState;
use ratatui::prelude::Alignment;
use ratatui::style::Color;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap};
use ratatui::{Frame, layout::Rect};
use uuid;

pub fn render(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let viewport_height = area.height as usize;
    let viewport_width = area.width;

    resolve_pending_focus_request(app_state, viewport_height, viewport_width);
    update_focus_and_scroll_state(app_state, viewport_height, viewport_width);

    let messages = app_state.current_messages();

    if messages.is_empty() {
        render_empty_message(frame, area);
        return;
    }

    render_messages_with_scroll(frame, area, messages, app_state, viewport_width);
    render_scrollbar(frame, area, app_state);
}

fn resolve_pending_focus_request(
    app_state: &mut AppState,
    viewport_height: usize,
    viewport_width: u16,
) {
    if let Some(request) = app_state.pending_focus_request.take() {
        let messages = app_state.current_messages();

        let focus_result = match request {
            crate::app::state::FocusRequest::Message(message_id) => resolve_focus_specific_message(
                &messages,
                message_id,
                viewport_height,
                viewport_width,
            ),
            crate::app::state::FocusRequest::LastMessage => {
                resolve_focus_last_message(&messages, viewport_height, viewport_width)
            }
            crate::app::state::FocusRequest::SameIndexOrLast { previous_index } => {
                resolve_focus_same_index_or_last(
                    &messages,
                    previous_index,
                    viewport_height,
                    viewport_width,
                )
            }
        };

        if let Some((message_id, scroll_offset)) = focus_result {
            app_state.focused_message_id = Some(message_id);
            app_state.scroll_offset = scroll_offset;
        }
    }
}

fn resolve_focus_specific_message(
    messages: &[&frond_core::Message],
    message_id: uuid::Uuid,
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(uuid::Uuid, isize)> {
    scrolling::calculate_scroll_to_focus_message(
        messages,
        message_id,
        viewport_height as isize,
        viewport_width,
    )
    .map(|scroll_offset| (message_id, scroll_offset))
}

fn resolve_focus_last_message(
    messages: &[&frond_core::Message],
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(uuid::Uuid, isize)> {
    if let Some(last_message) = messages.last() {
        resolve_focus_specific_message(messages, last_message.id(), viewport_height, viewport_width)
    } else {
        None
    }
}

fn resolve_focus_same_index_or_last(
    messages: &[&frond_core::Message],
    previous_index: usize,
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(uuid::Uuid, isize)> {
    let target_index = previous_index.min(messages.len().saturating_sub(1));
    if let Some(target_message) = messages.get(target_index) {
        resolve_focus_specific_message(
            messages,
            target_message.id(),
            viewport_height,
            viewport_width,
        )
    } else {
        None
    }
}

fn update_focus_and_scroll_state(
    app_state: &mut AppState,
    viewport_height: usize,
    viewport_width: u16,
) {
    let messages = app_state.current_messages();
    let total_content_height = scrolling::calculate_total_content_height(&messages, viewport_width);

    let clamped_scroll_offset = scrolling::clamp_scroll_offset(
        app_state.scroll_offset,
        viewport_height,
        total_content_height,
    );

    let focused_message_id = scrolling::update_focused_message_from_scroll(
        &messages,
        clamped_scroll_offset,
        viewport_height as isize,
        viewport_width,
    );

    app_state.scroll_offset = clamped_scroll_offset;
    app_state.focused_message_id = focused_message_id;

    // Update scrollbar state
    let messages = app_state.current_messages();
    let total_height = scrolling::calculate_total_content_height(&messages, viewport_width);
    app_state.scrollbar_state = scrolling::update_scrollbar_state(
        app_state.scrollbar_state,
        app_state.scroll_offset,
        viewport_height,
        total_height,
    );
}

fn calculate_message_height_for_rendering(
    message: &frond_core::Message,
    viewport_width: u16,
) -> usize {
    scrolling::calculate_message_display_height(message, viewport_width)
}

fn render_empty_message(frame: &mut Frame, area: Rect) {
    let empty_msg = create_empty_message_widget();
    frame.render_widget(empty_msg, area);
}

fn create_empty_message_widget() -> Paragraph<'static> {
    Paragraph::new("No messages in this branch")
        .alignment(Alignment::Center)
        .style(Style::default())
}

fn render_messages_with_scroll(
    frame: &mut Frame,
    area: Rect,
    messages: Vec<&frond_core::Message>,
    app_state: &AppState,
    viewport_width: u16,
) {
    let content_area = calculate_content_area(area);
    let mut y_offset = calculate_initial_y_offset(content_area, app_state.scroll_offset);

    for message in &messages {
        if should_render_message(y_offset, content_area, message, viewport_width) {
            render_single_message(frame, content_area, message, app_state, y_offset);
        }

        y_offset += calculate_message_height_for_rendering(message, viewport_width) as isize;

        if is_below_viewport(y_offset, content_area) {
            break;
        }
    }
}

fn calculate_content_area(area: Rect) -> Rect {
    Rect {
        x: area.x,
        y: area.y,
        width: area.width.saturating_sub(1),
        height: area.height,
    }
}

fn calculate_initial_y_offset(content_area: Rect, scroll_offset: isize) -> isize {
    content_area.y as isize - scroll_offset
}

fn should_render_message(
    y_offset: isize,
    content_area: Rect,
    message: &frond_core::Message,
    viewport_width: u16,
) -> bool {
    let message_height = calculate_message_height_for_rendering(message, viewport_width) as isize;
    let message_bottom = y_offset + message_height;
    let viewport_bottom = (content_area.y + content_area.height) as isize;

    message_bottom > content_area.y as isize && y_offset < viewport_bottom
}

fn render_single_message(
    frame: &mut Frame,
    content_area: Rect,
    message: &frond_core::Message,
    app_state: &AppState,
    y_offset: isize,
) {
    let is_focused = app_state.focused_message_id == Some(message.id());
    let message_area =
        calculate_visible_message_area(content_area, y_offset, message, content_area.width);

    if message_area.height > 0 {
        let message_widget =
            create_message_widget(message, app_state, is_focused, y_offset, content_area);
        frame.render_widget(message_widget, message_area);
    }
}

fn calculate_visible_message_area(
    content_area: Rect,
    y_offset: isize,
    message: &frond_core::Message,
    viewport_width: u16,
) -> Rect {
    let message_height =
        scrolling::calculate_message_display_height(message, viewport_width) as isize;
    let message_bottom = y_offset + message_height;
    let viewport_bottom = (content_area.y + content_area.height) as isize;

    let visible_start = y_offset.max(content_area.y as isize) as u16;
    let visible_end = message_bottom.min(viewport_bottom) as u16;
    let visible_height = visible_end.saturating_sub(visible_start);

    Rect {
        x: content_area.x,
        y: visible_start,
        width: content_area.width,
        height: visible_height,
    }
}

fn is_below_viewport(y_offset: isize, content_area: Rect) -> bool {
    let viewport_bottom = (content_area.y + content_area.height) as isize;
    y_offset >= viewport_bottom
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
    let scroll_offset = calculate_message_scroll_offset(y_offset, content_area);

    create_paragraph_with_theme(message.content(), theme, block, scroll_offset)
}

fn create_paragraph_with_theme(
    content: &str,
    theme: &crate::config::theme::MessageTheme,
    block: Block<'static>,
    scroll_offset: u16,
) -> Paragraph<'static> {
    Paragraph::new(content.to_string())
        .block(block)
        .style(Style::default().fg(theme.text_color))
        .wrap(Wrap { trim: false })
        .scroll((scroll_offset, 0))
}

fn calculate_message_scroll_offset(y_offset: isize, content_area: Rect) -> u16 {
    if y_offset < content_area.y as isize {
        (content_area.y as isize - y_offset) as u16
    } else {
        0
    }
}

fn create_message_block(
    theme: &crate::config::theme::MessageTheme,
    is_focused: bool,
) -> Block<'static> {
    let title = create_message_title(theme);
    let base_block = create_base_block(title, theme);

    apply_focus_styling(base_block, is_focused, theme)
}

fn create_message_title(theme: &crate::config::theme::MessageTheme) -> String {
    format!(" {} ", theme.display_name)
}

fn create_base_block(title: String, theme: &crate::config::theme::MessageTheme) -> Block<'static> {
    Block::bordered()
        .title(title)
        .title_alignment(Alignment::Left)
        .title_style(Style::default().fg(theme.title_color))
}

fn apply_focus_styling(
    block: Block<'static>,
    is_focused: bool,
    theme: &crate::config::theme::MessageTheme,
) -> Block<'static> {
    if is_focused {
        block
            .border_type(ratatui::widgets::BorderType::Double)
            .style(Style::default().fg(Color::White))
    } else {
        block.style(Style::default().fg(theme.frame_color))
    }
}

fn render_scrollbar(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let scrollbar = create_scrollbar_widget();
    let scrollbar_area = calculate_scrollbar_area(area);
    frame.render_stateful_widget(scrollbar, scrollbar_area, &mut app_state.scrollbar_state);
}

fn create_scrollbar_widget() -> Scrollbar<'static> {
    Scrollbar::new(ScrollbarOrientation::VerticalRight)
        .begin_symbol(Some("↑"))
        .end_symbol(Some("↓"))
}

fn calculate_scrollbar_area(area: Rect) -> Rect {
    Rect {
        x: area.x + area.width.saturating_sub(1),
        y: area.y,
        width: 1,
        height: area.height,
    }
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
