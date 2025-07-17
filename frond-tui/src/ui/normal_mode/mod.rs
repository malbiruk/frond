use crate::app::AppState;
use ratatui::Frame;

use super::layout::AppLayout;

mod breadcrumb;
mod content;
pub mod scrolling;
mod status;

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
