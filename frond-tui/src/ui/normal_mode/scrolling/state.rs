use ratatui::widgets::ScrollbarState;

pub fn update_scrollbar_state(
    scrollbar_state: ScrollbarState,
    scroll_offset: isize,
    viewport_height: usize,
    total_content_height: usize,
) -> ScrollbarState {
    let scroll_range = calculate_scroll_range(viewport_height, total_content_height);
    let min_scroll = calculate_min_scroll(viewport_height);
    let scrollbar_position = calculate_scrollbar_position(scroll_offset, min_scroll, scroll_range);

    scrollbar_state
        .content_length(scroll_range.max(1))
        .viewport_content_length(viewport_height)
        .position(scrollbar_position)
}

fn calculate_scroll_range(viewport_height: usize, total_content_height: usize) -> usize {
    let min_scroll = calculate_min_scroll(viewport_height);
    let max_scroll = total_content_height as isize - 3;
    let scroll_range = max_scroll - min_scroll;
    scroll_range.max(0) as usize
}

fn calculate_min_scroll(viewport_height: usize) -> isize {
    -(viewport_height as isize - 3)
}

fn calculate_scrollbar_position(
    scroll_offset: isize,
    min_scroll: isize,
    scroll_range: usize,
) -> usize {
    let position = (scroll_offset - min_scroll).max(0) as usize;
    position.min(scroll_range)
}

pub fn clamp_scroll_offset(
    scroll_offset: isize,
    viewport_height: usize,
    total_content_height: usize,
) -> isize {
    let min_scroll = calculate_min_scroll(viewport_height);
    let max_scroll = total_content_height as isize - 3;
    scroll_offset.clamp(min_scroll, max_scroll)
}
