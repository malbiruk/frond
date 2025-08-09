use super::{layout_calc, widget_factory};
use crate::app::AppState;
use ratatui::{Frame, layout::Rect};

pub fn render_messages_with_scroll(
    frame: &mut Frame,
    area: Rect,
    app_state: &mut AppState,
    _viewport_width: u16,
) {
    let content_area = layout_calc::calculate_content_area(area);
    app_state.populate_caches_for_current_branch(content_area.width);
    render_messages(frame, area, app_state);
}

fn render_messages(
    frame: &mut Frame,
    area: Rect,
    app_state: &mut AppState,
) {
    let content_area = layout_calc::calculate_content_area(area);
    let mut y_offset = layout_calc::calculate_initial_y_offset(content_area, app_state.scroll_offset);

    let message_data = collect_message_data(app_state);

    for (message_id, content, role) in message_data {
        if should_render_message(app_state, message_id, y_offset, &content, role, content_area, content_area.width) {
            render_message(frame, content_area, message_id, &content, role, app_state, y_offset);
        }

        y_offset += get_message_height(app_state, message_id, &content, role, content_area.width);

        if layout_calc::is_below_viewport(y_offset, content_area) {
            break;
        }
    }
}

fn collect_message_data(app_state: &AppState) -> Vec<(uuid::Uuid, String, frond_core::Role)> {
    app_state.current_messages()
        .iter()
        .map(|msg| (msg.id(), msg.content().to_string(), *msg.role()))
        .collect()
}

fn should_render_message(
    app_state: &AppState,
    message_id: uuid::Uuid,
    y_offset: isize,
    content: &str,
    role: frond_core::Role,
    content_area: Rect,
    content_width: u16,
) -> bool {
    let message_height = if let Some(cached_height) = app_state.get_cached_height(message_id, content_width) {
        cached_height as isize
    } else {
        let temp_msg = frond_core::Message::new(content, role);
        crate::ui::normal_mode::scrolling::calculate_message_display_height(&temp_msg, content_width) as isize
    };

    let message_bottom = y_offset + message_height;
    let viewport_bottom = (content_area.y + content_area.height) as isize;
    message_bottom > content_area.y as isize && y_offset < viewport_bottom
}

fn get_message_height(
    app_state: &AppState,
    message_id: uuid::Uuid,
    content: &str,
    role: frond_core::Role,
    content_width: u16,
) -> isize {
    if let Some(cached_height) = app_state.get_cached_height(message_id, content_width) {
        cached_height as isize
    } else {
        let temp_msg = frond_core::Message::new(content, role);
        crate::ui::normal_mode::scrolling::calculate_message_display_height(&temp_msg, content_width) as isize
    }
}

fn render_message(
    frame: &mut Frame,
    content_area: Rect,
    message_id: uuid::Uuid,
    content: &str,
    role: frond_core::Role,
    app_state: &mut AppState,
    y_offset: isize,
) {
    let message_area = calculate_message_area(app_state, message_id, content, role, content_area, y_offset);

    if message_area.height > 0 {
        let message_widget = widget_factory::create_message_widget_from_data(
            message_id, content, role, app_state, y_offset, content_area,
        );
        frame.render_widget(message_widget, message_area);
    }
}

fn calculate_message_area(
    app_state: &AppState,
    message_id: uuid::Uuid,
    content: &str,
    role: frond_core::Role,
    content_area: Rect,
    y_offset: isize,
) -> Rect {
    let message_height = if let Some(cached_height) = app_state.get_cached_height(message_id, content_area.width) {
        cached_height as isize
    } else {
        let temp_msg = frond_core::Message::new(content, role);
        crate::ui::normal_mode::scrolling::calculate_message_display_height(&temp_msg, content_area.width) as isize
    };

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


