use crate::app::AppState;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType};
use ratatui::prelude::Alignment;

pub fn initialize_from_message(app_state: &mut AppState, viewport_width: u16) {
    let message_id = match app_state.focused_message_id {
        Some(id) => id,
        None => {
            app_state.error_message = Some("No message selected for editing".to_string());
            return;
        }
    };

    let message_content = get_message_content(app_state, message_id);
    let content = match message_content {
        Ok(content) => content,
        Err(error) => {
            app_state.error_message = Some(error);
            return;
        }
    };

    let mut textarea = tui_textarea::TextArea::default();
    let wrap_width = calculate_wrap_width(viewport_width);
    let wrapped_lines = wrap_content(&content, wrap_width);

    insert_lines_into_textarea(&mut textarea, &wrapped_lines);
    position_cursor_at_start(&mut textarea);

    app_state.edit_textarea = Some(textarea);
}

pub fn style_textarea(
    textarea: &mut tui_textarea::TextArea,
    message_role: frond_core::Role,
    app_state: &AppState,
) {
    let block = create_focused_block(message_role, app_state);
    textarea.set_block(block);
    textarea.set_style(Style::default().fg(Color::Reset));
    textarea.set_cursor_line_style(Style::default());
}

pub fn calculate_display_height(textarea: &tui_textarea::TextArea, area_width: u16) -> u16 {
    let wrap_width = area_width.saturating_sub(2) as usize;
    let display_lines = textarea.lines()
        .iter()
        .map(|line| calculate_line_display_height(line, wrap_width))
        .sum::<usize>();
    
    (display_lines + 2).min(area_width as usize) as u16
}

fn get_message_content(app_state: &AppState, message_id: uuid::Uuid) -> Result<String, String> {
    let branch = app_state.current_branch()
        .ok_or_else(|| "No branch selected".to_string())?;

    let message = branch.get_message_by_id(message_id)
        .ok_or_else(|| "Message not found".to_string())?;

    Ok(message.content().to_string())
}

fn calculate_wrap_width(viewport_width: u16) -> usize {
    viewport_width.saturating_sub(4) as usize
}

fn wrap_content(content: &str, wrap_width: usize) -> Vec<String> {
    content
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
        .collect()
}

fn insert_lines_into_textarea(textarea: &mut tui_textarea::TextArea, lines: &[String]) {
    for (i, line) in lines.iter().enumerate() {
        if i > 0 {
            textarea.insert_newline();
        }
        textarea.insert_str(line);
    }
}

fn position_cursor_at_start(textarea: &mut tui_textarea::TextArea) {
    textarea.move_cursor(tui_textarea::CursorMove::Top);
    textarea.move_cursor(tui_textarea::CursorMove::Head);
}

fn calculate_line_display_height(line: &str, wrap_width: usize) -> usize {
    if line.is_empty() {
        1
    } else {
        line.chars().count().div_ceil(wrap_width)
    }
}

fn create_focused_block(message_role: frond_core::Role, app_state: &AppState) -> Block<'static> {
    let (display_name, title_color, border_color) = match message_role {
        frond_core::Role::Assistant => (
            "Assistant",
            app_state.config.theme.highlight_color,
            app_state.config.theme.highlight_color,
        ),
        frond_core::Role::User => ("User", Color::Reset, Color::Reset),
    };

    Block::bordered()
        .title(format!(" {} ", display_name))
        .title_alignment(Alignment::Left)
        .title_style(Style::default().fg(title_color))
        .border_type(BorderType::Double)
        .style(Style::default().fg(border_color))
}