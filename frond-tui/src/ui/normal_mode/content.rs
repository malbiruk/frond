use super::scrolling;
use crate::app::AppState;
use crate::app::state;
use ratatui::prelude::Alignment;
use ratatui::style::Style;
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, Wrap};
use ratatui::{Frame, layout::Rect};
use uuid;

pub fn render(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let viewport_height = area.height as usize;
    let viewport_width = area.width;

    resolve_pending_focus_request(app_state, viewport_height, viewport_width);
    update_focus_and_scroll_state(app_state, viewport_height, viewport_width);

    // Check if we have messages
    let has_messages = app_state.current_branch()
        .map(|b| !b.messages().is_empty())
        .unwrap_or(false);

    if !has_messages {
        render_empty_message(frame, area);
        return;
    }

    render_messages_with_scroll(frame, area, app_state, viewport_width);
    render_scrollbar(frame, area, app_state);
}

fn render_messages_with_scroll(
    frame: &mut Frame,
    area: Rect,
    app_state: &mut AppState,
    viewport_width: u16,
) {
    let content_area = calculate_content_area(area);
    let mut y_offset = calculate_initial_y_offset(content_area, app_state.scroll_offset);

    // Step 1: Extract message data with visibility info to avoid borrowing conflicts
    let message_data: Vec<(uuid::Uuid, bool)> = app_state.current_messages()
        .into_iter()
        .map(|msg| {
            let is_visible = should_render_message(y_offset + calculate_message_heights_before(app_state, msg.id(), viewport_width), content_area, msg, viewport_width);
            (msg.id(), is_visible)
        })
        .collect();

    // Step 2: Populate cache only for visible messages (mutable borrow)
    for &(message_id, is_visible) in &message_data {
        if is_visible && !app_state.highlight_cache.contains_key(&message_id) {
            if let Some(branch) = app_state.current_branch() {
                if let Some(message) = branch.get_message_by_id(message_id) {
                    let highlighted_text = crate::ui::normal_mode::highlighting::highlight_markdown(message.content());
                    app_state.highlight_cache.insert(message_id, highlighted_text);
                }
            }
        }
    }

    // Step 3: Render with cached data (immutable borrow)
    y_offset = calculate_initial_y_offset(content_area, app_state.scroll_offset);
    for (message_id, _) in message_data {
        let message = if let Some(branch) = app_state.current_branch() {
            branch.get_message_by_id(message_id)
        } else {
            break;
        };

        if let Some(message) = message {
            if should_render_message(y_offset, content_area, message, viewport_width) {
                render_single_message(frame, content_area, message, app_state, y_offset);
            }

            y_offset += calculate_message_height_for_rendering(message, viewport_width) as isize;

            if is_below_viewport(y_offset, content_area) {
                break;
            }
        }
    }
}

fn resolve_pending_focus_request(
    app_state: &mut AppState,
    viewport_height: usize,
    viewport_width: u16,
) {
    if let Some(request) = app_state.pending_scrolling_request.take() {
        let messages = app_state.current_messages();

        let focus_result = match request {
            state::ScrollingRequest::ScrollToMessage(message_id) => resolve_focus_specific_message(
                &messages,
                message_id,
                viewport_height,
                viewport_width,
            ),
            state::ScrollingRequest::ScrollToLastMessage => {
                resolve_focus_last_message(&messages, viewport_height, viewport_width)
            }
            state::ScrollingRequest::ScrollToMessageWithSameIndexOrLast { previous_index } => {
                resolve_focus_same_index_or_last(
                    &messages,
                    previous_index,
                    viewport_height,
                    viewport_width,
                )
            }
            state::ScrollingRequest::ScrollToTop => {
                resolve_focus_top(&messages, viewport_height, viewport_width)
            }
            state::ScrollingRequest::ScrollToBottom => {
                resolve_focus_bottom(&messages, viewport_height, viewport_width)
            }
            state::ScrollingRequest::ScrollPageUp => resolve_focus_scroll_by(
                &messages,
                app_state.scroll_offset,
                -(viewport_height as isize),
                viewport_height,
                viewport_width,
            ),
            state::ScrollingRequest::ScrollPageDown => resolve_focus_scroll_by(
                &messages,
                app_state.scroll_offset,
                viewport_height as isize,
                viewport_height,
                viewport_width,
            ),
            state::ScrollingRequest::ScrollHalfPageUp => resolve_focus_scroll_by(
                &messages,
                app_state.scroll_offset,
                -((viewport_height as f32 / 2.0).ceil() as isize),
                viewport_height,
                viewport_width,
            ),
            state::ScrollingRequest::ScrollHalfPageDown => resolve_focus_scroll_by(
                &messages,
                app_state.scroll_offset,
                (viewport_height as f32 / 2.0).ceil() as isize,
                viewport_height,
                viewport_width,
            ),
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

fn resolve_focus_top(
    messages: &[&frond_core::Message],
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(uuid::Uuid, isize)> {
    messages.first().map(|msg| {
        let scroll_offset = scrolling::calculate_scroll_to_focus_message(
            messages,
            msg.id(),
            viewport_height as isize,
            viewport_width,
        )
        .unwrap_or(0);
        (msg.id(), scroll_offset)
    })
}

fn resolve_focus_bottom(
    messages: &[&frond_core::Message],
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(uuid::Uuid, isize)> {
    resolve_focus_last_message(messages, viewport_height, viewport_width)
}

fn resolve_focus_scroll_by(
    messages: &[&frond_core::Message],
    current_scroll_offset: isize,
    delta: isize,
    viewport_height: usize,
    viewport_width: u16,
) -> Option<(uuid::Uuid, isize)> {
    let total_content_height = scrolling::calculate_total_content_height(messages, viewport_width);
    let new_scroll_offset = scrolling::clamp_scroll_offset(
        current_scroll_offset + delta,
        viewport_height,
        total_content_height,
    );
    let focused_message_id = scrolling::update_focused_message_from_scroll(
        messages,
        new_scroll_offset,
        viewport_height as isize,
        viewport_width,
    );
    focused_message_id.map(|id| (id, new_scroll_offset))
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

fn calculate_message_heights_before(
    app_state: &AppState,
    target_message_id: uuid::Uuid,
    viewport_width: u16,
) -> isize {
    let mut total_height = 0isize;
    let messages = app_state.current_messages();
    
    for message in messages {
        if message.id() == target_message_id {
            break;
        }
        total_height += calculate_message_height_for_rendering(message, viewport_width) as isize;
    }
    
    total_height
}

fn render_empty_message(frame: &mut Frame, area: Rect) {
    let empty_msg = Paragraph::new("No messages in this branch")
        .alignment(Alignment::Center)
        .style(Style::default());
    frame.render_widget(empty_msg, area);
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
    let message_area =
        calculate_visible_message_area(content_area, y_offset, message, content_area.width);

    if message_area.height > 0 {
        let message_widget = create_message_widget(message, app_state, y_offset, content_area);
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
    let scroll_offset = calculate_message_scroll_offset(y_offset, content_area);

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

fn calculate_message_scroll_offset(y_offset: isize, content_area: Rect) -> u16 {
    if y_offset < content_area.y as isize {
        (content_area.y as isize - y_offset) as u16
    } else {
        0
    }
}

fn create_message_block(
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

fn apply_focus_styling(
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
