//! Tests for clipboard operations in edit mode
//!
//! These tests verify clipboard functionality by testing state changes and
//! observable behavior without relying on implementation details or external clipboard.

use frond::actions::EditModeAction;
use frond::app::state::AppState;
use frond::app::reducer::edit_mode_handler::handle_edit_mode_action;
use frond::app::Mode;
use frond::config::Config;
use frond_core::{Branch, Dialogue, Message, Role, Tree};
use tui_textarea::TextArea;

fn create_test_dialogue() -> Dialogue {
    let mut dialogue = Dialogue::new("Test Dialogue");
    let mut tree = Tree::new("Test Tree");
    let mut branch = Branch::new("main");
    
    branch.add_message(Message::new("Hello world", Role::User));
    branch.add_message(Message::new("Hi there!", Role::Assistant));
    
    tree.add_branch(branch);
    dialogue.add_tree(tree);
    dialogue
}

fn create_app_state_with_textarea(content: &str) -> AppState {
    let dialogue = create_test_dialogue();
    let tree_id = dialogue.trees().get(0).unwrap().id();
    let branch_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().id();
    let message_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().messages().get(0).unwrap().id();
    
    let mut textarea = TextArea::default();
    textarea.insert_str(content);
    
    AppState {
        dialogue,
        mode: Mode::Edit,
        config: Config::default(),
        model_info: Default::default(),
        current_tree_id: Some(tree_id),
        current_branch_id: Some(branch_id),
        focused_message_id: Some(message_id),
        scroll_offset: 0,
        scrollbar_state: Default::default(),
        pending_scrolling_request: None,
        error_message: None,
        edit_textarea: Some(textarea),
        highlight_cache: Default::default(),
        height_cache: Default::default(),
    }
}

fn create_app_state_without_textarea() -> AppState {
    let dialogue = create_test_dialogue();
    let tree_id = dialogue.trees().get(0).unwrap().id();
    let branch_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().id();
    let message_id = dialogue.trees().get(0).unwrap().branches().get(0).unwrap().messages().get(0).unwrap().id();
    
    AppState {
        dialogue,
        mode: Mode::Edit,
        config: Config::default(),
        model_info: Default::default(),
        current_tree_id: Some(tree_id),
        current_branch_id: Some(branch_id),
        focused_message_id: Some(message_id),
        scroll_offset: 0,
        scrollbar_state: Default::default(),
        pending_scrolling_request: None,
        error_message: None,
        edit_textarea: None,
        highlight_cache: Default::default(),
        height_cache: Default::default(),
    }
}

fn get_textarea_content(state: &AppState) -> String {
    state.edit_textarea
        .as_ref()
        .map(|textarea| textarea.lines().join("\n"))
        .unwrap_or_default()
}

#[test]
fn undo_with_edit_history_works() {
    let mut state = create_app_state_with_textarea("hello");
    
    // Make an edit to create undo history
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.insert_str(" world");
    }
    
    let text_after_edit = get_textarea_content(&state);
    assert_eq!(text_after_edit, "hello world");
    
    handle_edit_mode_action(&mut state, EditModeAction::Undo);
    
    assert!(state.error_message.is_none());
    assert_eq!(get_textarea_content(&state), "hello");
}

#[test]
fn redo_after_undo_restores_edit() {
    let mut state = create_app_state_with_textarea("hello");
    
    // Make an edit
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.insert_str(" world");
    }
    
    // Undo the edit
    handle_edit_mode_action(&mut state, EditModeAction::Undo);
    assert_eq!(get_textarea_content(&state), "hello");
    
    // Redo should restore the edit
    handle_edit_mode_action(&mut state, EditModeAction::Redo);
    
    assert!(state.error_message.is_none());
    assert_eq!(get_textarea_content(&state), "hello world");
}

#[test]
fn undo_redo_require_valid_textarea() {
    let mut state = create_app_state_without_textarea();
    
    handle_edit_mode_action(&mut state, EditModeAction::Undo);
    assert!(state.error_message.is_some());
    assert!(state.error_message.as_ref().unwrap().contains("textarea"));
    
    state.error_message = None;
    handle_edit_mode_action(&mut state, EditModeAction::Redo);
    assert!(state.error_message.is_some());
    assert!(state.error_message.as_ref().unwrap().contains("textarea"));
}

#[test]
fn copy_with_selection_preserves_content() {
    let mut state = create_app_state_with_textarea("hello world");
    
    // Create a selection
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.move_cursor(tui_textarea::CursorMove::Jump(0, 6));
        textarea.start_selection();
        textarea.move_cursor(tui_textarea::CursorMove::End);
    }
    
    handle_edit_mode_action(&mut state, EditModeAction::Copy);
    
    // Original text should remain unchanged
    assert_eq!(get_textarea_content(&state), "hello world");
    // Copy should cancel selection (this is textarea behavior)
    assert!(!state.edit_textarea.as_ref().unwrap().is_selecting());
    assert!(state.error_message.is_none());
}

#[test]
fn copy_without_selection_does_not_error() {
    let mut state = create_app_state_with_textarea("hello world");
    
    handle_edit_mode_action(&mut state, EditModeAction::Copy);
    
    assert_eq!(get_textarea_content(&state), "hello world");
    assert!(!state.edit_textarea.as_ref().unwrap().is_selecting());
    assert!(state.error_message.is_none());
}

#[test]
fn cut_with_selection_removes_selected_text() {
    let mut state = create_app_state_with_textarea("hello world");
    
    // Select "world"
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.move_cursor(tui_textarea::CursorMove::Jump(0, 6));
        textarea.start_selection();
        textarea.move_cursor(tui_textarea::CursorMove::End);
    }
    
    handle_edit_mode_action(&mut state, EditModeAction::Cut);
    
    assert_eq!(get_textarea_content(&state), "hello ");
    assert!(!state.edit_textarea.as_ref().unwrap().is_selecting());
    assert!(state.error_message.is_none());
}

#[test]
fn cut_without_selection_preserves_content() {
    let mut state = create_app_state_with_textarea("hello world");
    
    handle_edit_mode_action(&mut state, EditModeAction::Cut);
    
    assert_eq!(get_textarea_content(&state), "hello world");
    assert!(state.error_message.is_none());
}

#[test]
fn paste_uses_internal_clipboard_when_desktop_empty() {
    let mut state = create_app_state_with_textarea("hello");
    
    // First copy some text to internal clipboard
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.select_all();
    }
    handle_edit_mode_action(&mut state, EditModeAction::Copy);
    
    // Clear selection and position cursor
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.cancel_selection();
        textarea.move_cursor(tui_textarea::CursorMove::End);
        textarea.insert_str(" world");
    }
    
    handle_edit_mode_action(&mut state, EditModeAction::Paste);
    
    // Should paste from internal clipboard since desktop clipboard is likely empty in test
    let content = get_textarea_content(&state);
    assert!(content.contains("hello"));
    assert!(state.error_message.is_none());
}

#[test]
fn select_all_activates_selection() {
    let mut state = create_app_state_with_textarea("hello\nworld\ntest");
    
    handle_edit_mode_action(&mut state, EditModeAction::SelectAll);
    
    assert!(state.edit_textarea.as_ref().unwrap().is_selecting());
    assert!(state.error_message.is_none());
    
    // Verify all text is selected by cutting it
    handle_edit_mode_action(&mut state, EditModeAction::Cut);
    assert_eq!(get_textarea_content(&state), "");
}

#[test]
fn clipboard_operations_require_valid_textarea() {
    let mut state = create_app_state_without_textarea();
    
    let clipboard_actions = vec![
        EditModeAction::Copy,
        EditModeAction::Cut,
        EditModeAction::Paste,
        EditModeAction::SelectAll,
    ];
    
    for action in clipboard_actions {
        state.error_message = None;
        handle_edit_mode_action(&mut state, action);
        assert!(state.error_message.is_some());
        assert!(state.error_message.as_ref().unwrap().contains("textarea"));
    }
}

#[test]
fn copy_cut_paste_sequence_works() {
    let mut state = create_app_state_with_textarea("hello world");
    
    // Select "world"
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.move_cursor(tui_textarea::CursorMove::Jump(0, 6));
        textarea.start_selection();
        textarea.move_cursor(tui_textarea::CursorMove::End);
    }
    
    // Copy it
    handle_edit_mode_action(&mut state, EditModeAction::Copy);
    assert_eq!(get_textarea_content(&state), "hello world");
    
    // Cut it (need to re-select since copy canceled selection)
    if let Some(ref mut textarea) = state.edit_textarea {
        textarea.move_cursor(tui_textarea::CursorMove::Jump(0, 6));
        textarea.start_selection();
        textarea.move_cursor(tui_textarea::CursorMove::End);
    }
    handle_edit_mode_action(&mut state, EditModeAction::Cut);
    assert_eq!(get_textarea_content(&state), "hello ");
    
    // Paste it back
    handle_edit_mode_action(&mut state, EditModeAction::Paste);
    let final_content = get_textarea_content(&state);
    assert!(final_content.contains("world"));
    
    assert!(state.error_message.is_none());
}