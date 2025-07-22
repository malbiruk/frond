use crate::app::AppState;
use crate::input::InputHandler;
use ratatui::prelude::Alignment;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::text::Span;
use ratatui::widgets::Block;
use ratatui::widgets::Borders;
use ratatui::widgets::Paragraph;
use ratatui::{Frame, layout::Rect};

pub fn render(frame: &mut Frame, area: Rect, app_state: &AppState) {
    render_status_bar(frame, area, app_state);
    render_help_line(frame, area, app_state);
}

fn render_status_bar(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let mode_text = build_mode_text(app_state);
    let model_info = build_model_info(app_state);
    let status_widget = create_status_widget(mode_text, model_info);
    frame.render_widget(status_widget, area);
}

fn build_mode_text(app_state: &AppState) -> String {
    if app_state.error_message.is_some() {
        return format!("Mode: {:?} > Error", app_state.mode);
    }
    format!("Mode: {:?}", app_state.mode)
}

fn build_model_info(app_state: &AppState) -> String {
    format!(
        "{}: {}",
        app_state.config.model.provider, app_state.config.model.name
    )
}

fn create_status_widget(left_text: String, right_text: String) -> Paragraph<'static> {
    let block = Block::new()
        .borders(Borders::TOP)
        .title(Line::from(format!("{} ", left_text)).left_aligned())
        .title(Line::from(format!(" {}", right_text)).right_aligned());

    Paragraph::new("").block(block)
}

fn render_help_line(frame: &mut Frame, area: Rect, app_state: &AppState) {
    let help_area = create_help_area(area);
    let help_line = create_help_line(app_state);
    let help_widget = create_help_widget(help_line);
    frame.render_widget(help_widget, help_area);
}

fn create_help_area(area: Rect) -> Rect {
    Rect {
        x: area.x,
        y: area.y + 1,
        width: area.width,
        height: 1,
    }
}

fn create_help_line(app_state: &AppState) -> Line<'static> {
    if app_state.error_message.is_some() {
        return create_error_help_line(app_state);
    }

    let input_handler = InputHandler::new(&app_state.config);
    let available_schemas = input_handler.get_essential_schemas(app_state.mode);
    let action_spans = build_action_spans(app_state, &available_schemas);
    Line::from(action_spans)
}

fn create_error_help_line(app_state: &AppState) -> Line<'static> {
    Line::from(vec![
        Span::styled(
            "esc",
            Style::default().fg(app_state.config.theme.help_key_color),
        ),
        Span::raw(": dismiss"),
    ])
}

fn build_action_spans(
    app_state: &AppState,
    available_schemas: &[(&'static str, &crate::actions::ActionSchema)],
) -> Vec<Span<'static>> {
    let mut spans = Vec::new();

    for (i, (action_id, schema)) in available_schemas.iter().enumerate() {
        if should_add_separator(i) {
            spans.push(Span::raw("  "));
        }

        if should_skip_schema(schema, app_state) {
            continue;
        }

        if let Some(action_spans) = create_action_spans(app_state, action_id, schema) {
            spans.extend(action_spans);
        }
    }

    spans
}

fn should_add_separator(index: usize) -> bool {
    index > 0
}

fn should_skip_schema(schema: &crate::actions::ActionSchema, app_state: &AppState) -> bool {
    schema.requires_focus() && app_state.focused_message_id.is_none()
}

fn create_action_spans(
    app_state: &AppState,
    action_id: &str,
    schema: &crate::actions::ActionSchema,
) -> Option<Vec<Span<'static>>> {
    let key_chord = app_state
        .config
        .get_key_for_action(app_state.mode, action_id)?;
    let key_display = format_key_chord(key_chord);

    Some(vec![
        Span::styled(
            key_display,
            Style::default().fg(app_state.config.theme.help_key_color),
        ),
        Span::raw(format!(": {}  ", schema.name())),
    ])
}

fn format_key_chord(key_chord: &crate::input::KeyChord) -> String {
    let modifiers_string = build_modifiers_string(key_chord);
    let key_string = format_key_code(key_chord.key);
    format!("{}{}", modifiers_string, key_string)
}

fn build_modifiers_string(key_chord: &crate::input::KeyChord) -> String {
    use ratatui::crossterm::event::KeyModifiers;

    let mut result = String::new();

    if key_chord.modifiers.contains(KeyModifiers::CONTROL) {
        result.push_str("ctrl-");
    }
    if key_chord.modifiers.contains(KeyModifiers::SHIFT) {
        result.push_str("shift-");
    }
    if key_chord.modifiers.contains(KeyModifiers::ALT) {
        result.push_str("alt-");
    }

    result
}

fn format_key_code(key_code: ratatui::crossterm::event::KeyCode) -> String {
    use ratatui::crossterm::event::KeyCode;

    match key_code {
        KeyCode::Char(c) => c.to_string(),
        KeyCode::Up => "↑".to_string(),
        KeyCode::Down => "↓".to_string(),
        KeyCode::Left => "←".to_string(),
        KeyCode::Right => "→".to_string(),
        KeyCode::Esc => "esc".to_string(),
        KeyCode::Enter => "enter".to_string(),
        KeyCode::Tab => "tab".to_string(),
        KeyCode::Backspace => "backspace".to_string(),
        KeyCode::Delete => "delete".to_string(),
        KeyCode::Home => "home".to_string(),
        KeyCode::End => "end".to_string(),
        KeyCode::PageUp => "pgup".to_string(),
        KeyCode::PageDown => "pgdn".to_string(),
        _ => "?".to_string(),
    }
}

fn create_help_widget(help_line: Line<'static>) -> Paragraph<'static> {
    Paragraph::new(help_line).alignment(Alignment::Center)
}
