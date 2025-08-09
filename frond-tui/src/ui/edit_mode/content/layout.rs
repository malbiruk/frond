use ratatui::layout::Rect;

pub struct AppendModeLayout {
    pub history: Rect,
    pub textarea: Rect,
}

pub struct ProgressiveEditLayout {
    pub history_above: Rect,
    pub textarea: Rect,
    pub history_below: Rect,
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

pub fn calculate_adaptive_progressive_layout(
    area: Rect,
    textarea_height: u16,
    actual_history_height: u16,
) -> ProgressiveEditLayout {
    let available_height = area.height;

    // Always keep textarea centered (handle overflow case)
    let space_above = if textarea_height >= available_height {
        0
    } else {
        (available_height - textarea_height) / 2
    };
    let space_below = if textarea_height >= available_height {
        0
    } else {
        available_height - textarea_height - space_above
    };

    // If history is smaller than allocated space, position it right above textarea

    if actual_history_height <= space_above {
        // Position history right above textarea
        let gap_above = space_above - actual_history_height;

        ProgressiveEditLayout {
            history_above: Rect {
                x: area.x,
                y: area.y + gap_above, // Push history down so it's right above textarea
                width: area.width,
                height: actual_history_height,
            },
            textarea: Rect {
                x: area.x,
                y: area.y + space_above,
                width: area.width,
                height: textarea_height,
            },
            history_below: Rect {
                x: area.x,
                y: area.y + space_above + textarea_height,
                width: area.width,
                height: space_below,
            },
        }
    } else {
        // Use full space above
        ProgressiveEditLayout {
            history_above: Rect {
                x: area.x,
                y: area.y,
                width: area.width,
                height: space_above,
            },
            textarea: Rect {
                x: area.x,
                y: area.y + space_above,
                width: area.width,
                height: textarea_height,
            },
            history_below: Rect {
                x: area.x,
                y: area.y + space_above + textarea_height,
                width: area.width,
                height: space_below,
            },
        }
    }
}
