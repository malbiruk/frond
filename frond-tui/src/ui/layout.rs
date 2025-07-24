use crate::app::Mode;
use ratatui::layout::{Constraint, Layout, Rect};

pub enum AppLayout {
    Normal {
        breadcrumb: Rect,
        content: Rect,
        status: Rect,
    },
}

impl AppLayout {
    pub fn new(area: Rect, mode: Mode) -> Self {
        match mode {
            Mode::Normal => Self::normal_layout(area),
            Mode::Edit => Self::edit_layout(area),
        }
    }

    fn normal_layout(area: Rect) -> Self {
        let chunks = Layout::vertical([
            Constraint::Length(2), // Breadcrumb
            Constraint::Min(0),    // Content (flexible)
            Constraint::Length(2), // Status
        ])
        .margin(1)
        .split(area);

        Self::Normal {
            breadcrumb: chunks[0],
            content: chunks[1],
            status: chunks[2],
        }
    }

    fn edit_layout(area: Rect) -> Self {
        let chunks = Layout::vertical([
            Constraint::Length(3), // Breadcrumb
            Constraint::Min(0),    // Content (flexible)
            Constraint::Length(2), // Status
        ])
        .margin(1)
        .split(area);

        Self::Normal {
            breadcrumb: chunks[0],
            content: chunks[1],
            status: chunks[2],
        }
    }
}
