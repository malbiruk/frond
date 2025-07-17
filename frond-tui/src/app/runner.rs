use super::AppState;
use crate::actions::{ActionDispatcher, CommonAction, UIAction};
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
            terminal.draw(|frame| crate::ui::render(frame, &mut self.state))?;

            if let Event::Key(key_event) = event::read()? {
                if let Some(action) = self.input_handler.handle_input(
                    key_event,
                    self.state.mode,
                    self.state.focused_message_id,
                ) {
                    match action {
                        UIAction::Common(CommonAction::Quit) => break,
                        other => {
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
