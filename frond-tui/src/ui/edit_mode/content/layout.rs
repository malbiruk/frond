use ratatui::layout::Rect;

pub struct AppendModeLayout {
    pub history: Rect,
    pub textarea: Rect,
}

pub fn calculate_append_layout(area: Rect, textarea_height: u16) -> AppendModeLayout {
    let history_height = area.height.saturating_sub(textarea_height);
    
    AppendModeLayout {
        history: Rect {
            x: area.x,
            y: area.y,
            width: area.width,
            height: history_height,
        },
        textarea: Rect {
            x: area.x,
            y: area.y + history_height,
            width: area.width,
            height: textarea_height,
        },
    }
}