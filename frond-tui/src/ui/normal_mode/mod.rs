use crate::app::AppState;
use ratatui::Frame;

use super::layout::AppLayout;
use super::status;

pub mod breadcrumb;
pub mod content;
pub mod highlighting;
pub mod scrolling;

pub fn render(frame: &mut Frame, app_state: &mut AppState) {
    let layout = AppLayout::new(frame.area(), app_state.mode);

    let AppLayout::Normal {
        breadcrumb,
        content,
        status,
    } = layout;

    breadcrumb::render(frame, breadcrumb, app_state);
    content::render(frame, content, app_state);
    status::render(frame, status, app_state);
}
