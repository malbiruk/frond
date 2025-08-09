use super::utils::textarea_operation;
use crate::app::state::AppState;

pub fn handle_undo(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.undo();
    });
}

pub fn handle_redo(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.redo();
    });
}

pub fn handle_copy(state: &mut AppState) {
    handle_copy_or_cut_operation(state, "copy", |textarea| {
        textarea.copy();
    });
}

pub fn handle_cut(state: &mut AppState) {
    handle_copy_or_cut_operation(state, "cut", |textarea| {
        textarea.cut();
    });
}

pub fn handle_paste(state: &mut AppState) {
    match get_desktop_clipboard_text() {
        Ok(desktop_text) if !desktop_text.is_empty() => {
            textarea_operation(state, |textarea| {
                textarea.insert_str(&desktop_text);
            });
        }
        Ok(_) | Err(_) => {
            // Desktop clipboard empty or failed, silently use textarea's internal paste
            textarea_operation(state, |textarea| {
                textarea.paste();
            });
        }
    }
}

pub fn handle_select_all(state: &mut AppState) {
    textarea_operation(state, |textarea| {
        textarea.select_all();
    });
}

// Common operation for copy and cut that handles desktop clipboard integration
fn handle_copy_or_cut_operation<F>(state: &mut AppState, operation: &str, textarea_op: F)
where
    F: FnOnce(&mut tui_textarea::TextArea),
{
    let mut clipboard_error: Option<String> = None;

    textarea_operation(state, |textarea| {
        // Get selected text before the operation
        let selected_text = get_selected_text_if_any(textarea);

        // Perform the textarea operation
        textarea_op(textarea);

        // Copy to desktop clipboard if we had selected text
        if let Some(text) = selected_text {
            if let Err(e) = copy_to_desktop_clipboard(&text) {
                clipboard_error = Some(format!("Failed to {} to clipboard: {}", operation, e));
            }
        }
    });

    // Set error after the textarea operation if needed
    if let Some(error) = clipboard_error {
        state.error_message = Some(error);
    }
}

// Extract selected text if any exists
fn get_selected_text_if_any(textarea: &tui_textarea::TextArea) -> Option<String> {
    textarea
        .selection_range()
        .map(|selection| get_selected_text(textarea, selection))
}

// Desktop clipboard operations
fn copy_to_desktop_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    use cli_clipboard::{ClipboardContext, ClipboardProvider};
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(text.to_owned())?;
    Ok(())
}

fn get_desktop_clipboard_text() -> Result<String, Box<dyn std::error::Error>> {
    use cli_clipboard::{ClipboardContext, ClipboardProvider};
    let mut ctx = ClipboardContext::new()?;
    ctx.get_contents()
}

fn get_selected_text(
    textarea: &tui_textarea::TextArea,
    selection: ((usize, usize), (usize, usize)),
) -> String {
    let lines = textarea.lines();
    let ((start_row, start_col), (end_row, end_col)) = selection;

    // Ensure start is before end
    let ((start_row, start_col), (end_row, end_col)) =
        if start_row < end_row || (start_row == end_row && start_col <= end_col) {
            ((start_row, start_col), (end_row, end_col))
        } else {
            ((end_row, end_col), (start_row, start_col))
        };

    let mut result = String::new();

    if start_row == end_row {
        // Selection within single line
        if let Some(line) = lines.get(start_row) {
            let chars: Vec<char> = line.chars().collect();
            if end_col <= chars.len() && start_col <= chars.len() {
                result = chars[start_col..end_col].iter().collect();
            }
        }
    } else {
        // Multi-line selection
        for (line_idx, line) in lines.iter().enumerate() {
            if line_idx < start_row || line_idx > end_row {
                continue;
            }

            let chars: Vec<char> = line.chars().collect();
            if line_idx == start_row {
                // First line: from start_col to end
                if start_col < chars.len() {
                    result.push_str(&chars[start_col..].iter().collect::<String>());
                }
                result.push('\n');
            } else if line_idx == end_row {
                // Last line: from start to end_col
                if end_col <= chars.len() {
                    result.push_str(&chars[..end_col].iter().collect::<String>());
                }
            } else {
                // Middle lines: entire line
                result.push_str(line);
                result.push('\n');
            }
        }
    }

    result
}
