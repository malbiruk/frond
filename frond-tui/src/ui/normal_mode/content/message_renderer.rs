//! Core message rendering logic
//!
//! Handles the main rendering loop with highlighting cache optimization and message iteration.

use super::{layout_calc, widget_factory};
use crate::app::AppState;
use ratatui::layout::Rect;
use ratatui::Frame;

/// Render all messages with scroll support and highlighting cache optimization
pub fn render_messages_with_scroll(
    frame: &mut Frame,
    area: Rect,
    app_state: &mut AppState,
    viewport_width: u16,
) {
    let content_area = layout_calc::calculate_content_area(area);
    let mut y_offset = layout_calc::calculate_initial_y_offset(content_area, app_state.scroll_offset);

    // Step 1: Extract message data with visibility info to avoid borrowing conflicts
    let messages = app_state.current_messages();
    let message_data: Vec<(uuid::Uuid, bool)> = messages
        .iter()
        .map(|msg| {
            let cumulative_offset = y_offset + layout_calc::calculate_message_heights_before(&messages, msg.id(), viewport_width);
            let is_visible = layout_calc::should_render_message(cumulative_offset, content_area, msg, viewport_width);
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
    y_offset = layout_calc::calculate_initial_y_offset(content_area, app_state.scroll_offset);
    for (message_id, _) in message_data {
        let message = if let Some(branch) = app_state.current_branch() {
            branch.get_message_by_id(message_id)
        } else {
            break;
        };

        if let Some(message) = message {
            if layout_calc::should_render_message(y_offset, content_area, message, viewport_width) {
                render_single_message(frame, content_area, message, app_state, y_offset);
            }

            y_offset += layout_calc::calculate_message_height_for_rendering(message, viewport_width) as isize;

            if layout_calc::is_below_viewport(y_offset, content_area) {
                break;
            }
        }
    }
}

/// Render a single message at the specified Y offset
pub fn render_single_message(
    frame: &mut Frame,
    content_area: Rect,
    message: &frond_core::Message,
    app_state: &AppState,
    y_offset: isize,
) {
    let message_area =
        layout_calc::calculate_visible_message_area(content_area, y_offset, message, content_area.width);

    if message_area.height > 0 {
        let message_widget = widget_factory::create_message_widget(message, app_state, y_offset, content_area);
        frame.render_widget(message_widget, message_area);
    }
}