use frond_core::Message;
use uuid::Uuid;

pub fn update_focused_message_from_scroll(
    messages: &[&Message],
    scroll_offset: isize,
    viewport_height: isize,
    viewport_width: u16,
) -> Option<Uuid> {
    if messages.is_empty() {
        return None;
    }

    let center_line = calculate_center_line(scroll_offset, viewport_height);
    find_message_at_center_line(messages, center_line, viewport_width)
}

fn calculate_center_line(scroll_offset: isize, viewport_height: isize) -> isize {
    scroll_offset + viewport_height / 2
}

fn find_message_at_center_line(
    messages: &[&Message],
    center_line: isize,
    viewport_width: u16,
) -> Option<Uuid> {
    if center_line < 0 {
        return messages.first().map(|m| m.id());
    }

    let center_line_usize = center_line as usize;
    let mut current_line = 0;

    for message in messages {
        let message_height = super::calculate_message_display_height(message, viewport_width);
        if current_line + message_height > center_line_usize {
            return Some(message.id());
        }
        current_line += message_height;
    }

    messages.last().map(|m| m.id())
}

pub fn update_focused_message_after_deletion(
    messages: &[&Message],
    deleted_index: usize,
) -> Option<Uuid> {
    if messages.is_empty() {
        return None;
    }

    if deleted_index < messages.len() {
        Some(messages[deleted_index].id())
    } else if deleted_index > 0 {
        Some(messages[deleted_index - 1].id())
    } else {
        None
    }
}

pub fn get_first_message_id(messages: &[&Message]) -> Option<Uuid> {
    messages.first().map(|m| m.id())
}

pub fn calculate_scroll_to_focus_message(
    messages: &[&Message],
    message_id: Uuid,
    viewport_height: isize,
    viewport_width: u16,
) -> Option<isize> {
    let message_y_position = calculate_message_y_position(messages, message_id, viewport_width)?;
    let message_height = find_message_height(messages, message_id, viewport_width)?;

    let message_center = message_y_position as isize + (message_height as isize / 2);
    let viewport_center = viewport_height / 2;

    Some(message_center - viewport_center)
}

fn calculate_message_y_position(
    messages: &[&Message],
    target_message_id: Uuid,
    viewport_width: u16,
) -> Option<usize> {
    let mut y_position = 0;

    for message in messages {
        if message.id() == target_message_id {
            return Some(y_position);
        }
        y_position += super::calculate_message_display_height(message, viewport_width);
    }

    None
}

fn find_message_height(
    messages: &[&Message],
    target_message_id: Uuid,
    viewport_width: u16,
) -> Option<usize> {
    messages
        .iter()
        .find(|msg| msg.id() == target_message_id)
        .map(|msg| super::calculate_message_display_height(msg, viewport_width))
}
