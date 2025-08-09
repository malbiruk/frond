mod errors;
mod history;
pub mod layout;
mod message;
mod textarea;

use crate::app::AppState;
use ratatui::{Frame, layout::Rect};

use errors::*;
use layout::*;
use textarea as textarea_utils;

pub fn render(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    if should_initialize_textarea(app_state) {
        textarea_utils::initialize_from_message(app_state, area.width);
    }

    let focused_message_id = match app_state.focused_message_id {
        Some(id) => id,
        None => {
            render_no_message_selected(frame, area);
            return;
        }
    };

    let edit_context = match get_edit_context(app_state, focused_message_id) {
        Ok(context) => context,
        Err(error_type) => {
            render_context_error(frame, area, error_type);
            return;
        }
    };

    match app_state.edit_textarea.as_ref() {
        Some(_) if edit_context.is_append_mode => {
            render_append_mode(frame, area, edit_context.message_role, app_state);
        }
        Some(_) => {
            render_progressive_edit_mode(frame, area, edit_context.message_role, app_state);
        }
        None => {
            render_textarea_not_initialized(frame, area);
        }
    }
}

struct EditContext {
    is_append_mode: bool,
    message_role: frond_core::Role,
}

fn should_initialize_textarea(app_state: &AppState) -> bool {
    app_state.edit_textarea.is_none() && app_state.mode == crate::app::Mode::Edit
}

fn get_edit_context(
    app_state: &AppState,
    message_id: uuid::Uuid,
) -> Result<EditContext, ContextError> {
    let branch = app_state.current_branch().ok_or(ContextError::NoBranch)?;
    let message = branch
        .get_message_by_id(message_id)
        .ok_or(ContextError::MessageNotFound)?;

    Ok(EditContext {
        is_append_mode: app_state.is_append_mode(),
        message_role: message.role().clone(),
    })
}

enum ContextError {
    NoBranch,
    MessageNotFound,
}

fn render_context_error(frame: &mut Frame, area: Rect, error: ContextError) {
    match error {
        ContextError::NoBranch => render_no_branch_selected(frame, area),
        ContextError::MessageNotFound => render_message_not_found(frame, area),
    }
}

fn render_progressive_edit_mode(
    frame: &mut Frame,
    area: Rect,
    message_role: frond_core::Role,
    app_state: &mut AppState,
) {
    let textarea = app_state.edit_textarea.as_ref().unwrap().clone();
    let textarea_height = textarea_utils::calculate_display_height(&textarea, area.width);

    // If message is already bigger than viewport, just fill the area
    if textarea_height >= area.height {
        let mut styled_textarea = textarea.clone();
        textarea_utils::style_textarea(&mut styled_textarea, message_role, app_state);
        frame.render_widget(&styled_textarea, area);
        return;
    }

    let editing_message_id = app_state.focused_message_id.unwrap();

    // Calculate actual history height needed
    let messages_before = get_messages_before(app_state, editing_message_id);
    let actual_history_height = calculate_messages_height(&messages_before, area.width);

    // Use adaptive layout that positions history right above textarea when possible
    let layout =
        calculate_adaptive_progressive_layout(area, textarea_height, actual_history_height);

    // Render history above if visible
    if layout.history_above.height > 0 {
        history::render_excluding_message(
            frame,
            layout.history_above,
            app_state,
            editing_message_id,
        );
    }

    // Render textarea
    let mut styled_textarea = textarea;
    textarea_utils::style_textarea(&mut styled_textarea, message_role, app_state);
    frame.render_widget(&styled_textarea, layout.textarea);

    // Render history below if visible
    if layout.history_below.height > 0 {
        history::render_excluding_message_below(
            frame,
            layout.history_below,
            app_state,
            editing_message_id,
        );
    }
}

fn render_append_mode(
    frame: &mut Frame,
    area: Rect,
    message_role: frond_core::Role,
    app_state: &mut AppState,
) {
    let textarea = app_state.edit_textarea.as_ref().unwrap().clone();
    let textarea_height = textarea_utils::calculate_display_height(&textarea, area.width);
    let layout = calculate_append_layout(area, textarea_height);

    if layout.history.height > 0 {
        let editing_message_id = app_state.focused_message_id.unwrap();
        history::render_excluding_message(frame, layout.history, app_state, editing_message_id);
    }

    let mut styled_textarea = textarea;
    textarea_utils::style_textarea(&mut styled_textarea, message_role, app_state);
    frame.render_widget(&styled_textarea, layout.textarea);
}

// Helper functions
fn get_messages_before(app_state: &AppState, exclude_id: uuid::Uuid) -> Vec<&frond_core::Message> {
    let all_messages = app_state.current_messages();
    let mut messages_before = Vec::new();

    for message in all_messages {
        if message.id() == exclude_id {
            break;
        }
        messages_before.push(message);
    }

    messages_before
}

fn calculate_messages_height(messages: &[&frond_core::Message], width: u16) -> u16 {
    let total_height: usize = messages
        .iter()
        .map(|m| crate::ui::normal_mode::scrolling::calculate_message_display_height(m, width))
        .sum();
    total_height as u16
}
