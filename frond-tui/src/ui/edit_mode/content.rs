//! Content rendering for edit mode
//!
//! Handles bottom-anchored TextArea rendering with focus styling continuity

use crate::app::AppState;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Alignment;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType, Paragraph};
use uuid::Uuid;

/// Main entry point for rendering the edit mode content area
pub fn render(frame: &mut Frame, area: Rect, app_state: &mut AppState) {
    // Only initialize if not already initialized (append mode initializes immediately)
    if app_state.edit_textarea.is_none() && app_state.mode == crate::app::Mode::Edit {
        initialize_textarea_with_wrapping(app_state, area.width);
    }

    let Some(focused_message_id) = app_state.focused_message_id else {
        render_no_message_selected(frame, area);
        return;
    };

    // Get the data we need before taking mutable references
    let (is_append_mode, message_role) = {
        let branch = app_state.current_branch();
        if branch.is_none() {
            render_no_branch_selected(frame, area);
            return;
        }
        let branch = branch.unwrap();

        let message = branch.get_message_by_id(focused_message_id);
        if message.is_none() {
            render_message_not_found(frame, area);
            return;
        }
        let message = message.unwrap();

        let is_append = branch.messages().iter().last().map(|last| last.id()) == Some(message.id())
            && *message.role() == frond_core::Role::User;

        (is_append, message.role().clone())
    };

    if app_state.edit_textarea.is_some() {
        if is_append_mode {
            // In append mode, show history above and textarea at bottom
            render_append_mode(frame, area, message_role, app_state);
        } else {
            // Full-screen edit mode for existing messages
            let textarea = app_state.edit_textarea.as_ref().unwrap();
            render_fullscreen_edit_mode(frame, area, textarea, message_role, app_state);
        }
    } else {
        render_textarea_not_initialized(frame, area);
    }
}

/// Render full-screen edit mode for existing messages
fn render_fullscreen_edit_mode(
    frame: &mut Frame,
    area: Rect,
    textarea: &tui_textarea::TextArea,
    message_role: frond_core::Role,
    app_state: &AppState,
) {
    let block = create_edit_mode_block(message_role, app_state);

    let mut styled_textarea = textarea.clone();
    styled_textarea.set_block(block);
    styled_textarea.set_style(Style::default().fg(Color::Reset));
    styled_textarea.set_cursor_line_style(Style::default());

    frame.render_widget(&styled_textarea, area);
}

/// Render append mode with history above and growing textarea at bottom
fn render_append_mode(
    frame: &mut Frame,
    area: Rect,
    message_role: frond_core::Role,
    app_state: &mut AppState,
) {
    // Get and clone the textarea before any mutable operations
    let textarea = app_state.edit_textarea.as_ref().unwrap().clone();

    // Calculate the height needed for the textarea considering wrapped lines
    // Each line might wrap, so we need to calculate actual display lines
    let wrap_width = area.width.saturating_sub(2) as usize; // -2 for borders
    let mut display_lines = 0;
    for line in textarea.lines() {
        if line.is_empty() {
            display_lines += 1;
        } else {
            // Calculate how many display lines this logical line takes
            let line_width = line.chars().count();
            display_lines += line_width.div_ceil(wrap_width); // Ceiling division
        }
    }

    // Add 2 for borders
    let textarea_height = (display_lines + 2).min(area.height as usize) as u16;

    // Split the area: history above, textarea at bottom
    let history_height = area.height.saturating_sub(textarea_height);

    let history_area = ratatui::layout::Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: history_height,
    };

    let textarea_area = ratatui::layout::Rect {
        x: area.x,
        y: area.y + history_height,
        width: area.width,
        height: textarea_height,
    };

    // Render the message history with simple scrolling
    if history_height > 0 {
        // Get the message being edited so we can exclude it
        let editing_message_id = app_state.focused_message_id.unwrap();

        // Calculate actual textarea growth using the SAME logic as textarea height calculation
        let textarea_growth = display_lines.saturating_sub(1); // Growth beyond minimum 1 line

        // Simple approach: render history with scroll adjustment
        render_history_excluding_message(
            frame,
            history_area,
            app_state,
            editing_message_id,
            textarea_growth,
        );
    }

    // Render the textarea at the bottom
    let block = create_edit_mode_block(message_role, app_state);
    let mut styled_textarea = textarea;
    styled_textarea.set_block(block);
    styled_textarea.set_style(Style::default().fg(Color::Reset));
    styled_textarea.set_cursor_line_style(Style::default());

    frame.render_widget(&styled_textarea, textarea_area);
}

/// Simple renderer for history messages excluding the edited one
fn render_history_excluding_message(
    frame: &mut Frame,
    area: Rect,
    app_state: &mut AppState,
    exclude_message_id: Uuid,
    _textarea_growth: usize,
) {
    // Get messages excluding the one being edited
    let messages: Vec<&frond_core::Message> = app_state
        .current_messages()
        .into_iter()
        .filter(|m| m.id() != exclude_message_id)
        .collect();

    if messages.is_empty() {
        return;
    }

    // Calculate total height of all messages
    let total_height: usize = messages
        .iter()
        .map(|m| crate::ui::normal_mode::scrolling::calculate_message_display_height(m, area.width))
        .sum();

    // Simple scroll: just show the bottom messages that fit
    let available_height = area.height as usize;
    let scroll_offset = total_height.saturating_sub(available_height);

    // Render messages, skipping the scrolled portion
    let mut current_height = 0usize;
    let mut render_y = area.y;

    for message in messages.iter() {
        let message_height = crate::ui::normal_mode::scrolling::calculate_message_display_height(
            message, area.width,
        );

        // Skip messages that are scrolled off the top
        if current_height + message_height <= scroll_offset {
            current_height += message_height;
            continue;
        }

        // Stop if we've run out of space
        if render_y >= area.y + area.height {
            break;
        }

        // Calculate visible portion of this message
        let skip_lines = scroll_offset.saturating_sub(current_height);

        let visible_height =
            (message_height - skip_lines).min((area.y + area.height - render_y) as usize);

        if visible_height > 0 {
            let message_area = ratatui::layout::Rect {
                x: area.x,
                y: render_y,
                width: area.width,
                height: visible_height as u16,
            };

            render_message_widget(frame, message_area, message, app_state);
            render_y += visible_height as u16;
        }

        current_height += message_height;
    }
}

/// Helper to render a single message widget
fn render_message_widget(
    frame: &mut Frame,
    area: Rect,
    message: &frond_core::Message,
    app_state: &AppState,
) {
    use ratatui::text::Text;

    // Determine styling based on role
    let (title, border_color) = match message.role() {
        frond_core::Role::Assistant => ("Assistant", app_state.config.theme.highlight_color),
        frond_core::Role::User => ("User", Color::Reset),
    };

    // Create block with single border (not focused)
    let block = Block::bordered()
        .title(format!(" {} ", title))
        .title_alignment(Alignment::Left)
        .border_type(BorderType::Plain)
        .style(Style::default().fg(border_color));

    // Try to use cached highlighted content if available
    let content = if let Some(highlighted) = app_state.highlight_cache.get(&message.id()) {
        highlighted.clone()
    } else {
        Text::raw(message.content())
    };

    let paragraph = Paragraph::new(content)
        .block(block)
        .wrap(ratatui::widgets::Wrap { trim: false });

    frame.render_widget(paragraph, area);
}

/// Create block with same styling as focused message in normal mode
fn create_edit_mode_block(message_role: frond_core::Role, app_state: &AppState) -> Block<'static> {
    let (display_name, title_color, border_color) = match message_role {
        frond_core::Role::Assistant => (
            "Assistant",
            app_state.config.theme.highlight_color,
            app_state.config.theme.highlight_color,
        ),
        frond_core::Role::User => ("User", Color::Reset, Color::Reset),
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
