mod app;
mod config;
mod ui;

use app::AppState;
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};
use ui::render;

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
