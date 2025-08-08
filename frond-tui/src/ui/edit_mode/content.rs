//! Content rendering for edit mode
//!
//! Handles bottom-anchored TextArea rendering with focus styling continuity

use crate::app::AppState;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph};
use ratatui::Frame;

/// Main entry point for rendering the edit mode content area
pub fn render(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    // If we're just entering edit mode and textarea is not initialized yet, initialize it with proper wrapping
    if app_state.edit_textarea.is_none() && app_state.mode == crate::app::Mode::Edit {
        initialize_textarea_with_wrapping(app_state, area.width);
    }
    
    // Check if we have a message being edited
    let Some(focused_message_id) = app_state.focused_message_id else {
        render_no_message_selected(frame, area);
        return;
    };

    let Some(branch) = app_state.current_branch() else {
        render_no_branch_selected(frame, area);
        return;
    };

    let Some(message) = branch.get_message_by_id(focused_message_id) else {
        render_message_not_found(frame, area);
        return;
    };

    // Render the TextArea with focus styling
    if let Some(ref textarea) = app_state.edit_textarea {
        render_textarea_with_focus_styling(frame, area, textarea, message, app_state);
    } else {
        render_textarea_not_initialized(frame, area);
    }
}

/// Render TextArea with the same focus styling as normal mode
fn render_textarea_with_focus_styling(
    frame: &mut Frame,
    area: Rect,
    textarea: &tui_textarea::TextArea,
    message: &frond_core::Message,
    app_state: &AppState,
) {
    // Calculate edit area (bottom-anchored positioning will be implemented later)
    let edit_area = calculate_edit_area(area, textarea.lines().len());
    
    // Create block with focus styling (double border + role colors)
    let block = create_edit_mode_block(message, app_state);
    
    // Create TextArea widget with focus styling
    let mut styled_textarea = textarea.clone();
    styled_textarea.set_block(block);
    
    // Set text style to terminal default color (not border color)
    styled_textarea.set_style(Style::default().fg(Color::Reset));
    
    // Remove underline from current line
    styled_textarea.set_cursor_line_style(Style::default());
    
    frame.render_widget(&styled_textarea, edit_area);
}

/// Calculate the area for the TextArea based on content size and bottom-anchoring
fn calculate_edit_area(viewport: Rect, _line_count: usize) -> Rect {
    // For now, use the full viewport - will implement smart positioning later
    // TODO: Implement bottom-anchored positioning based on content size
    viewport
}

/// Create block with same styling as focused message in normal mode
fn create_edit_mode_block(
    message: &frond_core::Message,
    app_state: &AppState,
) -> Block<'static> {
    // Determine title and colors based on message role
    let (display_name, title_color, border_color) = match message.role() {
        frond_core::Role::Assistant => (
            "Assistant",
            app_state.config.theme.highlight_color,
            app_state.config.theme.highlight_color,
        ),
        frond_core::Role::User => (
            "User",
            Color::Reset,
            Color::Reset,
        ),
    };

    let title = format!(" {} ", display_name);
    
    Block::bordered()
        .title(title)
        .title_alignment(Alignment::Left)
        .title_style(Style::default().fg(title_color))
        .border_type(BorderType::Double) // Always double border in edit mode (focus indication)
        .style(Style::default().fg(border_color))
}

/// Render error states
fn render_no_message_selected(frame: &mut Frame, area: Rect) {
    let error_msg = Paragraph::new("No message selected for editing")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    frame.render_widget(error_msg, area);
}

fn render_no_branch_selected(frame: &mut Frame, area: Rect) {
    let error_msg = Paragraph::new("No branch selected")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    frame.render_widget(error_msg, area);
}

fn render_message_not_found(frame: &mut Frame, area: Rect) {
    let error_msg = Paragraph::new("Message not found")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    frame.render_widget(error_msg, area);
}

fn render_textarea_not_initialized(frame: &mut Frame, area: Rect) {
    let error_msg = Paragraph::new("TextArea not initialized")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    frame.render_widget(error_msg, area);
}

/// Initialize textarea with message content and proper word wrapping based on viewport width
fn initialize_textarea_with_wrapping(app_state: &mut AppState, viewport_width: u16) {
    let Some(message_id) = app_state.focused_message_id else {
        app_state.error_message = Some("No message selected for editing".to_string());
        return;
    };

    // Get the message content
    let message_content = if let Some(branch) = app_state.current_branch() {
        if let Some(message) = branch.get_message_by_id(message_id) {
            message.content().to_string()
        } else {
            app_state.error_message = Some("Message not found".to_string());
            return;
        }
    } else {
        app_state.error_message = Some("No branch selected".to_string());
        return;
    };

    // Initialize TextArea with pre-wrapped message content
    let mut textarea = tui_textarea::TextArea::default();
    
    // Calculate wrap width based on actual viewport, accounting for borders and scrollbar
    let wrap_width = viewport_width.saturating_sub(4) as usize; // -2 for borders, -1 for scrollbar, -1 for padding
    
    let wrapped_lines: Vec<String> = message_content
        .lines()
        .flat_map(|line| {
            if line.is_empty() {
                vec![String::new()]
            } else {
                textwrap::wrap(line, wrap_width)
                    .into_iter()
                    .map(|cow| cow.to_string())
                    .collect::<Vec<_>>()
            }
        })
        .collect();
    
    // Insert wrapped lines into textarea
    for (i, line) in wrapped_lines.iter().enumerate() {
        if i > 0 {
            textarea.insert_newline();
        }
        textarea.insert_str(line);
    }
    
    // Position cursor at the beginning of the message
    textarea.move_cursor(tui_textarea::CursorMove::Top);
    textarea.move_cursor(tui_textarea::CursorMove::Head);
    
    app_state.edit_textarea = Some(textarea);
}