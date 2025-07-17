use super::AppState;
use crate::actions::{ActionDispatcher, UIAction};
use crate::input::InputHandler;
use color_eyre::eyre::Result;
use ratatui::{
    DefaultTerminal,
    crossterm::event::{self, Event},
};

pub struct App {
    state: AppState,
    input_handler: InputHandler,
}

impl App {
    pub fn new() -> Self {
        let state = AppState::default();
        let input_handler = InputHandler::new(&state.config);

        Self {
            state,
            input_handler,
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            // Render the UI
            terminal.draw(|frame| crate::ui::render(frame, &self.state))?;

            // Handle input events
            if let Event::Key(key_event) = event::read()? {
                if let Some(action) = self.input_handler.handle_input(
                    key_event,
                    self.state.mode,
                    self.state.focused_message_id,
                ) {
                    match action {
                        UIAction::Quit => break,
                        other => {
                            // Dispatch the action through the flux system
                            self.state.dispatch(other);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn state(&self) -> &AppState {
        &self.state
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
