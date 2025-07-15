use super::layout::AppLayout;
use crate::app::AppState;
use ratatui::{
    Frame,
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Wrap},
};

pub fn render(frame: &mut Frame, app_state: &AppState) {
    let layout = AppLayout::new(frame.area(), app_state.mode);

    match layout {
        AppLayout::Normal {
            breadcrumb,
            content,
            status,
        } => {
            render_breadcrumb(frame, breadcrumb, app_state);
            render_content(frame, content, app_state);
            render_status(frame, status, app_state);
        }
    }
}

fn render_breadcrumb(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let breadcrumb_text = build_breadcrumb_text(app_state);
    let token_info = format!(
        "{}K/{}K",
        app_state.config.model_info.tokens_used / 1000,
        app_state.config.model_info.tokens_available / 1000
    );

    let title_line = create_title_line(breadcrumb_text, token_info, area.width);

    let breadcrumb = Paragraph::new("").block(Block::new().borders(Borders::TOP).title(title_line));

    frame.render_widget(breadcrumb, area);
}

fn render_content(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let messages = app_state.current_messages();

    if messages.is_empty() {
        let empty_msg = Paragraph::new("No messages in this branch")
            .alignment(Alignment::Center)
            .style(Style::default().fg(Color::Gray));
        frame.render_widget(empty_msg, area);
        return;
    }

    render_messages(frame, area, messages, app_state);
}

fn render_messages(
    frame: &mut Frame,
    area: Rect,
    messages: Vec<&frond_core::Message>,
    app_state: &AppState,
) {
    let mut y_offset = area.top();

    for message in messages {
        if y_offset >= area.bottom() {
            break;
        }

        let theme = match message.role() {
            frond_core::Role::Assistant => &app_state.config.theme.assistant,
            frond_core::Role::User => &app_state.config.theme.user,
        };

        let title = format!(" {} ", theme.display_name);

        // Calculate required height for this message
        let content_width = area.width.saturating_sub(4);
        let wrapped_lines = wrap_text(message.content(), content_width as usize);
        let required_height = wrapped_lines.len() + 2;

        let available_height = area.bottom().saturating_sub(y_offset);
        let message_height = required_height.min(available_height as usize) as u16;

        if message_height < 3 {
            break;
        }

        let message_area = Rect {
            x: area.x,
            y: y_offset,
            width: area.width,
            height: message_height,
        };

        let block = Block::bordered()
            .title(title)
            .title_alignment(Alignment::Left)
            .title_style(Style::default().fg(theme.title_color))
            .style(Style::default().fg(theme.frame_color));

        let paragraph = Paragraph::new(message.content())
            .block(block)
            .style(Style::default().fg(theme.text_color))
            .wrap(Wrap { trim: false });

        frame.render_widget(paragraph, message_area);

        y_offset += message_height;
    }
}

fn render_status(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let mode_text = format!("Mode: {:?}", app_state.mode);
    let model_info = format!(
        "{}: {}",
        app_state.config.model_info.provider, app_state.config.model_info.model
    );

    let status_line = create_status_line(mode_text, model_info, area.width);

    let mode_widget =
        Paragraph::new("").block(Block::new().borders(Borders::TOP).title(status_line));

    frame.render_widget(mode_widget, area);

    // Render help line
    let help_area = Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: 1,
    };

    let help_line = create_help_line(app_state);
    let help_widget = Paragraph::new(help_line).alignment(Alignment::Center);

    frame.render_widget(help_widget, help_area);
}

fn create_title_line(left_text: String, right_text: String, width: u16) -> Line<'static> {
    let available_width = width as usize;
    let left_len = left_text.len();
    let right_len = right_text.len();
    let dash_count = available_width.saturating_sub(left_len + right_len + 2); // +2 for spaces

    Line::from(vec![
        Span::raw(left_text),
        Span::raw(" "),
        Span::raw("─".repeat(dash_count)),
        Span::raw(" "),
        Span::raw(right_text),
    ])
}

fn create_status_line(left_text: String, right_text: String, width: u16) -> Line<'static> {
    let available_width = width as usize;
    let left_len = left_text.len();
    let right_len = right_text.len();
    let dash_count = available_width.saturating_sub(left_len + right_len + 2); // +2 for spaces

    Line::from(vec![
        Span::raw(left_text),
        Span::raw(" "),
        Span::raw("─".repeat(dash_count)),
        Span::raw(" "),
        Span::raw(right_text),
    ])
}

fn create_help_line(app_state: &AppState) -> Line<'static> {
    let binding = vec![];
    let bindings = app_state
        .config
        .keybindings
        .mode_bindings
        .get(&app_state.mode)
        .unwrap_or(&binding);

    let mut spans = Vec::new();
    for (i, binding) in bindings.iter().enumerate() {
        if i > 0 {
            spans.push(Span::raw("  "));
        }
        spans.push(Span::styled(
            binding.key.to_string(),
            Style::default().fg(app_state.config.theme.help_key_color),
        ));
        spans.push(Span::raw(format!(": {}", binding.description)));
    }

    Line::from(spans)
}

fn build_breadcrumb_text(app_state: &AppState) -> String {
    let mut parts = vec![app_state.dialogue.name().to_string()];

    if app_state.dialogue.trees().len() > 1 {
        if let Some(tree) = app_state.current_tree() {
            parts.push(tree.name().to_string());
        }
    }

    if let Some(tree) = app_state.current_tree() {
        if tree.branches().len() > 1 {
            if let Some(branch) = app_state.current_branch() {
                parts.push(branch.name().to_string());
            }
        }
    }

    parts.join(" > ")
}

fn wrap_text(text: &str, width: usize) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut current_line = text;

    while current_line.len() > width {
        if let Some(space_pos) = current_line[..width].rfind(' ') {
            lines.push(&current_line[..space_pos]);
            current_line = &current_line[space_pos + 1..];
        } else {
            lines.push(&current_line[..width]);
            current_line = &current_line[width..];
        }
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    lines
}
