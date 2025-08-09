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
    // Step 1: Collect all messages to avoid borrowing issues
    let messages = app_state.current_messages();
    let message_ids: Vec<uuid::Uuid> = messages.iter().map(|msg| msg.id()).collect();
    
    // Step 2: Pre-populate highlight cache for visible messages
    for &message_id in &message_ids {
        if !app_state.highlight_cache.contains_key(&message_id) {
            if let Some(branch) = app_state.current_branch() {
                if let Some(message) = branch.get_message_by_id(message_id) {
                    // Check if message is visible (simplified check)
                    let highlighted_text = crate::ui::normal_mode::highlighting::highlight_markdown(message.content());
                    app_state.highlight_cache.insert(message_id, highlighted_text);
                }
            }
        }
    }

    // Step 3: Simple render loop
    render_messages_simple(frame, area, app_state, viewport_width);
}

/// Simple message rendering without complex caching logic
fn render_messages_simple(
    frame: &mut Frame,
    area: Rect,
    app_state: &mut AppState,
    viewport_width: u16,
) {
    let content_area = layout_calc::calculate_content_area(area);
    let mut y_offset = layout_calc::calculate_initial_y_offset(content_area, app_state.scroll_offset);

    // Collect message IDs first to avoid borrowing conflicts
    let message_ids: Vec<uuid::Uuid> = app_state.current_messages()
        .iter()
        .map(|msg| msg.id())
        .collect();

    // Collect message data first to avoid borrowing conflicts
    let message_data: Vec<(uuid::Uuid, String, frond_core::Role)> = message_ids
        .iter()
        .filter_map(|&id| {
            if let Some(branch) = app_state.current_branch() {
                if let Some(message) = branch.get_message_by_id(id) {
                    Some((id, message.content().to_string(), *message.role()))
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    // Now render using owned data - no borrowing conflicts
    for (message_id, content, role) in message_data {
        // Check if message should be rendered (using cached height if available)
        let should_render = if let Some(cached_height) = app_state.get_cached_height(message_id, viewport_width) {
            let message_bottom = y_offset + cached_height as isize;
            let viewport_bottom = (content_area.y + content_area.height) as isize;
            message_bottom > content_area.y as isize && y_offset < viewport_bottom
        } else {
            // Fallback using temporary message
            let temp_msg = create_temp_message(message_id, &content, role);
            layout_calc::should_render_message(y_offset, content_area, &temp_msg, viewport_width)
        };

        if should_render {
            render_single_message_by_id(frame, content_area, message_id, &content, role, app_state, y_offset);
        }

        // Calculate height for next iteration (use cache if available, otherwise calculate)
        let message_height = if let Some(cached_height) = app_state.get_cached_height(message_id, viewport_width) {
            cached_height as isize
        } else {
            let temp_msg = create_temp_message(message_id, &content, role);
            layout_calc::calculate_message_height_for_rendering(&temp_msg, viewport_width) as isize
        };

        y_offset += message_height;

        if layout_calc::is_below_viewport(y_offset, content_area) {
            break;
        }
    }
}

/// Render a single message by ID using owned data to avoid borrowing conflicts
fn render_single_message_by_id(
    frame: &mut Frame,
    content_area: Rect,
    message_id: uuid::Uuid,
    content: &str,
    role: frond_core::Role,
    app_state: &mut AppState,
    y_offset: isize,
) {
    // Calculate message area using cached height if available
    let message_area = if let Some(cached_height) = app_state.get_cached_height(message_id, content_area.width) {
        // Use cached height for quick calculation
        let message_bottom = y_offset + cached_height as isize;
        let viewport_bottom = (content_area.y + content_area.height) as isize;

        let visible_start = y_offset.max(content_area.y as isize) as u16;
        let visible_end = message_bottom.min(viewport_bottom) as u16;
        let visible_height = visible_end.saturating_sub(visible_start);

        ratatui::layout::Rect {
            x: content_area.x,
            y: visible_start,
            width: content_area.width,
            height: visible_height,
        }
    } else {
        // Fallback to calculation - this will be cached in create_message_widget_from_data
        layout_calc::calculate_visible_message_area(content_area, y_offset, &create_temp_message(message_id, content, role), content_area.width)
    };

    if message_area.height > 0 {
        let message_widget = widget_factory::create_message_widget_from_data(
            message_id, content, role, app_state, y_offset, content_area
        );
        frame.render_widget(message_widget, message_area);
    }
}

/// Create a temporary message for layout calculations
fn create_temp_message(_id: uuid::Uuid, content: &str, role: frond_core::Role) -> frond_core::Message {
    let msg = frond_core::Message::new(content, role);
    // We can't set the ID directly, but for layout calculations the ID doesn't matter
    // The height calculation only uses content, not the ID
    msg
}

/// Render a single message at the specified Y offset
pub fn render_single_message(
    frame: &mut Frame,
    content_area: Rect,
    message: &frond_core::Message,
    app_state: &mut AppState,
    y_offset: isize,
) {
    let message_area =
        layout_calc::calculate_visible_message_area_with_cache(content_area, y_offset, message, content_area.width, app_state);

    if message_area.height > 0 {
        let message_widget = widget_factory::create_message_widget(message, app_state, y_offset, content_area);
        frame.render_widget(message_widget, message_area);
    }
}