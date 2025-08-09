//! Content rendering for normal mode
//!
//! Main orchestration module that coordinates focus resolution, message rendering,
//! and UI state updates.

pub mod focus_resolver;
pub mod layout_calc;
pub mod message_renderer;
pub mod widget_factory;

use crate::app::AppState;
use crate::ui::normal_mode::scrolling;
use ratatui::layout::Rect;
use ratatui::Frame;

/// Main entry point for rendering the content area
pub fn render(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    let viewport_height = area.height as usize;
    let content_area = layout_calc::calculate_content_area(area);
    let viewport_width = content_area.width;

    resolve_pending_focus_request_handler(app_state, viewport_height, viewport_width);
    update_focus_and_scroll_state(app_state, viewport_height, viewport_width);

    // Check if we have messages
    let has_messages = app_state.current_branch()
        .map(|b| !b.messages().is_empty())
        .unwrap_or(false);

    if !has_messages {
        widget_factory::render_empty_message(frame, area);
        return;
    }

    message_renderer::render_messages_with_scroll(frame, area, app_state, viewport_width);
    widget_factory::render_scrollbar(frame, area, app_state);
}

/// Handle pending focus requests from the app state
fn resolve_pending_focus_request_handler(
    app_state: &mut AppState,
    viewport_height: usize,
    viewport_width: u16,
) {
    if let Some(request) = app_state.pending_scrolling_request.take() {
        let messages = app_state.current_messages();

        let focus_result = focus_resolver::resolve_pending_focus_request(
            request,
            &messages,
            app_state.scroll_offset,
            viewport_height,
            viewport_width,
        );

        if let Some((message_id, scroll_offset)) = focus_result {
            app_state.focused_message_id = Some(message_id);
            app_state.scroll_offset = scroll_offset;
        }
    }
}

/// Update focus and scroll state based on current viewport
fn update_focus_and_scroll_state(
    app_state: &mut AppState,
    viewport_height: usize,
    viewport_width: u16,
) {
    let messages = app_state.current_messages();
    let total_content_height = scrolling::calculate_total_content_height_with_cache(&messages, viewport_width, app_state);

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
    let total_height = scrolling::calculate_total_content_height_with_cache(&messages, viewport_width, app_state);
    app_state.scrollbar_state = scrolling::update_scrollbar_state(
        app_state.scrollbar_state,
        app_state.scroll_offset,
        viewport_height,
        total_height,
    );
}