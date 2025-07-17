use super::normal_mode;
use crate::app::AppState;
use crate::app::Mode;
use ratatui::Frame;

pub fn render(frame: &mut Frame, app_state: &AppState) {
    match app_state.mode {
        Mode::Normal => normal_mode::render(frame, app_state),
        Mode::Edit(_edit_mode) => todo!(),
    }
}
