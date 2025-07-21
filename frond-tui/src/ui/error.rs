use crate::app::AppState;
use ratatui::prelude::Alignment;
use ratatui::text::Text;
use ratatui::{
    Frame,
    buffer::Buffer,
    layout::Rect,
    style::{Style, Stylize},
    text::Line,
    widgets::{Block, Borders, Clear, Paragraph, Widget, Wrap},
};

#[derive(Debug, Default)]
pub struct Popup<'a> {
    pub title: Line<'a>,
    pub content: &'a str,
    pub border_style: Style,
    pub title_style: Style,
    pub style: Style,
}

impl Widget for Popup<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Clear.render(area, buf);
        let block = Block::new()
            .title(self.title)
            .title_style(self.title_style)
            .borders(Borders::ALL)
            .border_style(self.border_style);
        Paragraph::new(self.content)
            .wrap(Wrap { trim: true })
            .alignment(Alignment::Center)
            .style(self.style)
            .block(block)
            .render(area, buf);
    }
}

pub fn render_error_popup(frame: &mut Frame, app_state: &AppState, area: Rect, padding_y: u16) {
    if let Some(ref msg) = app_state.error_message {
        let padded_content = pad_content_vertically(msg, padding_y);
        let popup = Popup {
            content: &padded_content,
            style: Style::new().white(),
            title: Line::from(" Error "),
            title_style: Style::new().red(),
            border_style: Style::new().red(),
        };
        frame.render_widget(popup, area);
    }
}

fn pad_content_vertically(content: &str, padding_y: u16) -> String {
    let pad_top = padding_y / 2;
    let pad_bottom = padding_y - pad_top;
    let mut s = String::new();
    for _ in 0..pad_top {
        s.push('\n');
    }
    s.push_str(content);
    for _ in 0..pad_bottom {
        s.push('\n');
    }
    s
}

pub fn measure_popup(msg: &str, max_width: u16, padding_x: u16, padding_y: u16) -> (u16, u16) {
    let text = Text::from(msg);
    let mut width = 0;
    let mut lines = 0;
    for line in text.lines.iter() {
        let len = line.width() as u16;
        if len > width {
            width = len;
        }
        lines += 1;
    }
    let popup_width = (width + 2 * padding_x).min(max_width);
    let popup_height = lines as u16 + 2 * padding_y;
    (popup_width, popup_height)
}
