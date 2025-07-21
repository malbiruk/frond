use super::error::measure_popup;
use super::normal_mode;
use crate::app::AppState;
use crate::app::Mode;
use crate::ui::error::render_error_popup;
use ratatui::Frame;

pub fn render(frame: &mut Frame, app_state: &mut AppState) {
    match app_state.mode {
        Mode::Normal => normal_mode::render(frame, app_state),
        Mode::Edit(_edit_mode) => todo!(),
    }

    if let Some(ref msg) = app_state.error_message {
        let area = frame.area();
        let max_width = area.width.min(60);
        let padding_x = 4;
        let padding_y = 2;
        let (popup_width, popup_height) = measure_popup(msg, max_width, padding_x, padding_y);

        let popup_area = ratatui::layout::Rect {
            x: area.x + (area.width.saturating_sub(popup_width)) / 2,
            y: area.y + (area.height.saturating_sub(popup_height)) / 2,
            width: popup_width,
            height: popup_height,
        };
        render_error_popup(frame, app_state, popup_area, padding_y);
    }
}
