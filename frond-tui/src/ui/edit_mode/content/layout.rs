use ratatui::layout::Rect;

pub struct AppendModeLayout {
    pub history: Rect,
    pub textarea: Rect,
}

pub fn calculate_append_layout(area: Rect, textarea_height: u16) -> AppendModeLayout {
    // If textarea needs more space than available content area, give it the full area
    if textarea_height >= area.height {
        return AppendModeLayout {
            history: Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: 0,
            },
            textarea: Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: area.height, // Give textarea the full content area height
            },
        };
    }
    
    let history_height = area.height - textarea_height;
    
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