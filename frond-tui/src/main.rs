use color_eyre::eyre::Result;
use frond_core::Dialogue;
use ratatui::prelude::Stylize;
use ratatui::{
    DefaultTerminal, Frame,
    crossterm::event::{self, Event},
    layout::{Constraint, Layout},
    style::Color,
    widgets::{Block, BorderType::Rounded, Paragraph, Widget},
};

#[derive(Debug, Default)]
struct AppState {
    dialogues: Vec<Dialogue>,
}

fn main() -> Result<()> {
    let mut state = AppState::default();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal, &mut state);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal, app_state: &mut AppState) -> Result<()> {
    loop {
        terminal.draw(|frame| render(frame, app_state))?;
        if let Event::Key(key) = event::read()? {
            match key.code {
                event::KeyCode::Char('q') => break,
                _ => {}
            }
        }
    }
    Ok(())
}

fn render(frame: &mut Frame, app_state: &AppState) {
    let [border_area] = Layout::vertical([Constraint::Fill(1)])
        .margin(1)
        .areas(frame.area());

    Block::bordered()
        .border_type(Rounded)
        .fg(Color::Yellow)
        .render(border_area, frame.buffer_mut());

    Paragraph::new("Hello, world!").render(frame.area(), frame.buffer_mut());
}
