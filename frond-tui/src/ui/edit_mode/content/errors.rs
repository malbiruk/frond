use ratatui::{Frame, layout::Rect, style::{Color, Style}, widgets::Paragraph, prelude::Alignment};

pub fn render_no_message_selected(frame: &mut Frame, area: Rect) {
    render_error(frame, area, "No message selected for editing");
}

pub fn render_no_branch_selected(frame: &mut Frame, area: Rect) {
    render_error(frame, area, "No branch selected");
}

pub fn render_message_not_found(frame: &mut Frame, area: Rect) {
    render_error(frame, area, "Message not found");
}

pub fn render_textarea_not_initialized(frame: &mut Frame, area: Rect) {
    render_error(frame, area, "TextArea not initialized");
}

fn render_error(frame: &mut Frame, area: Rect, message: &str) {
    let error_msg = Paragraph::new(message)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::Red));
    frame.render_widget(error_msg, area);
}