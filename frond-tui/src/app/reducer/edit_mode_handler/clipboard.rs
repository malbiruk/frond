use super::utils::textarea_operation;
use crate::app::state::AppState;
use cli_clipboard::{ClipboardContext, ClipboardProvider};

// Public API - Simple delegation to textarea operations
pub fn handle_undo(state: &mut AppState) {
    simple_textarea_operation(state, |textarea| {
        textarea.undo();
    });
}

pub fn handle_redo(state: &mut AppState) {
    simple_textarea_operation(state, |textarea| {
        textarea.redo();
    });
}

pub fn handle_copy(state: &mut AppState) {
    clipboard_operation(state, ClipboardOperation::Copy);
}

pub fn handle_cut(state: &mut AppState) {
    clipboard_operation(state, ClipboardOperation::Cut);
}

pub fn handle_paste(state: &mut AppState) {
    paste_from_clipboard(state);
}

pub fn handle_select_all(state: &mut AppState) {
    simple_textarea_operation(state, |textarea| {
        textarea.select_all();
    });
}

// Operation types
enum ClipboardOperation {
    Copy,
    Cut,
}

impl ClipboardOperation {
    fn name(&self) -> &'static str {
        match self {
            Self::Copy => "copy",
            Self::Cut => "cut",
        }
    }

    fn execute(&self, textarea: &mut tui_textarea::TextArea) {
        match self {
            Self::Copy => {
                textarea.copy();
            }
            Self::Cut => {
                textarea.cut();
            }
        }
    }
}

// Core operations
fn simple_textarea_operation<F>(state: &mut AppState, operation: F)
where
    F: FnOnce(&mut tui_textarea::TextArea),
{
    textarea_operation(state, operation);
}

fn clipboard_operation(state: &mut AppState, operation: ClipboardOperation) {
    let mut clipboard_error = None;

    textarea_operation(state, |textarea| {
        if let Some(selected_text) = extract_selected_text(textarea) {
            operation.execute(textarea);
            
            if let Err(e) = sync_to_desktop_clipboard(&selected_text) {
                clipboard_error = Some(format!(
                    "Failed to {} to clipboard: {}",
                    operation.name(),
                    e
                ));
            }
        } else {
            operation.execute(textarea);
        }
    });

    if let Some(error) = clipboard_error {
        state.error_message = Some(error);
    }
}

fn paste_from_clipboard(state: &mut AppState) {
    match read_desktop_clipboard() {
        Ok(text) if !text.is_empty() => {
            textarea_operation(state, |textarea| {
                textarea.insert_str(&text);
            });
        }
        _ => {
            // Fall back to internal clipboard
            textarea_operation(state, |textarea| {
                textarea.paste();
            });
        }
    }
}

// Clipboard integration
fn sync_to_desktop_clipboard(text: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut ctx = ClipboardContext::new()?;
    ctx.set_contents(text.to_owned())?;
    Ok(())
}

fn read_desktop_clipboard() -> Result<String, Box<dyn std::error::Error>> {
    let mut ctx = ClipboardContext::new()?;
    ctx.get_contents()
}

// Text extraction
fn extract_selected_text(textarea: &tui_textarea::TextArea) -> Option<String> {
    textarea
        .selection_range()
        .map(|range| build_selected_text(textarea, range))
}

fn build_selected_text(
    textarea: &tui_textarea::TextArea,
    selection: ((usize, usize), (usize, usize)),
) -> String {
    let lines = textarea.lines();
    let (start, end) = normalize_selection_range(selection);
    
    if is_single_line_selection(start, end) {
        extract_single_line_text(lines, start, end)
    } else {
        extract_multi_line_text(lines, start, end)
    }
}

fn normalize_selection_range(
    selection: ((usize, usize), (usize, usize)),
) -> ((usize, usize), (usize, usize)) {
    let ((start_row, start_col), (end_row, end_col)) = selection;
    
    if start_row < end_row || (start_row == end_row && start_col <= end_col) {
        ((start_row, start_col), (end_row, end_col))
    } else {
        ((end_row, end_col), (start_row, start_col))
    }
}

fn is_single_line_selection(start: (usize, usize), end: (usize, usize)) -> bool {
    start.0 == end.0
}

fn extract_single_line_text(
    lines: &[String],
    start: (usize, usize),
    end: (usize, usize),
) -> String {
    lines
        .get(start.0)
        .and_then(|line| {
            let chars: Vec<char> = line.chars().collect();
            if end.1 <= chars.len() && start.1 <= chars.len() {
                Some(chars[start.1..end.1].iter().collect())
            } else {
                None
            }
        })
        .unwrap_or_default()
}

fn extract_multi_line_text(
    lines: &[String],
    start: (usize, usize),
    end: (usize, usize),
) -> String {
    let mut result = String::new();
    
    for (line_idx, line) in lines.iter().enumerate() {
        if line_idx < start.0 || line_idx > end.0 {
            continue;
        }
        
        append_line_segment(&mut result, line, line_idx, start, end);
        
        if line_idx < end.0 {
            result.push('\n');
        }
    }
    
    result
}

fn append_line_segment(
    result: &mut String,
    line: &str,
    line_idx: usize,
    start: (usize, usize),
    end: (usize, usize),
) {
    let chars: Vec<char> = line.chars().collect();
    
    let segment = match line_idx {
        idx if idx == start.0 => extract_from_position(&chars, start.1, chars.len()),
        idx if idx == end.0 => extract_from_position(&chars, 0, end.1),
        _ => line.to_string(),
    };
    
    result.push_str(&segment);
}

fn extract_from_position(chars: &[char], start: usize, end: usize) -> String {
    if start < chars.len() && end <= chars.len() {
        chars[start..end].iter().collect()
    } else if start < chars.len() {
        chars[start..].iter().collect()
    } else {
        String::new()
    }
}

